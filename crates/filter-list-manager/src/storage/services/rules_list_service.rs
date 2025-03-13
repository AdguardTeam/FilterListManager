use std::collections::HashSet;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

use chrono::Utc;
use rusqlite::Connection;
use rusqlite::Error;
use rusqlite::Transaction;

use crate::storage::blob::write_to_stream;
use crate::storage::entities::rules_list_entity::RulesListEntity;
use crate::storage::repositories::filter_repository::FilterRepository;
use crate::storage::repositories::rules_list_repository::RulesListRepository;
use crate::storage::repositories::Repository;
use crate::storage::services::constants::EXTRA_COMMENT_MARKER;
use crate::storage::services::constants::NON_RULE_MARKER;
use crate::storage::sql_generators::operator::SQLOperator;
use crate::storage::with_transaction;
use crate::storage::DbConnectionManager;
use crate::utils::parsing::LF_BYTES_SLICE;
use crate::DisabledRulesRaw;
use crate::FLMError;
use crate::FLMResult;
use crate::FilterId;
use crate::FilterListRules;
use crate::FilterListRulesRaw;
use crate::RulesCountByFilter;

/// Service for rules list repository
pub(crate) struct RulesListService;

impl RulesListService {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    pub(crate) fn get_disabled_rules(
        &self,
        connection_manager: &DbConnectionManager,
        ids: Vec<FilterId>,
    ) -> FLMResult<Vec<DisabledRulesRaw>> {
        connection_manager.execute_db(|connection: Connection| {
            RulesListRepository::new()
                .get_disabled_rules_by_ids(&connection, &ids)
                .map_err(FLMError::from_database)
        })
    }

    pub(crate) fn get_filter_rules_as_strings(
        &self,
        connection_manager: &DbConnectionManager,
        ids: Vec<FilterId>,
    ) -> FLMResult<Vec<FilterListRulesRaw>> {
        let result = connection_manager.execute_db(|conn: Connection| {
            RulesListRepository::new()
                .select(
                    &conn,
                    Some(SQLOperator::FieldIn(
                        "filter_id",
                        ids.into_iter().map(Into::into).collect(),
                    )),
                )
                .map_err(FLMError::from_database)
        })?;

        Ok(result
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect())
    }

    pub(crate) fn get_rules_count(
        &self,
        connection_manager: &DbConnectionManager,
        ids: Vec<FilterId>,
    ) -> FLMResult<Vec<RulesCountByFilter>> {
        connection_manager.execute_db(|connection: Connection| {
            RulesListRepository::new()
                .get_rules_count(&connection, &ids)
                .map_err(FLMError::from_database)
        })
    }

    pub(crate) fn save_custom_filter_rules(
        &self,
        connection_manager: &DbConnectionManager,
        rules: FilterListRules,
    ) -> FLMResult<()> {
        connection_manager.execute_db(move |mut conn: Connection| {
            let filter_repository = FilterRepository::new();

            let result = filter_repository
                .select(
                    &conn,
                    Some(FilterRepository::custom_filter_with_id(rules.filter_id)),
                )
                .map_err(FLMError::from_database)?;

            match result {
                Some(mut filters) if !filters.is_empty() => {
                    with_transaction(&mut conn, |transaction: &Transaction| {
                        // SAFETY: index "0" always present in this branch until condition
                        // `!filters.is_empty()` is met.
                        let filter = unsafe { filters.get_unchecked_mut(0) };

                        filter.last_update_time = Utc::now().timestamp();

                        filter_repository.insert(transaction, &filters)?;

                        RulesListRepository::new().insert(
                            transaction,
                            &[RulesListService::new().update_rules_count(rules)],
                        )
                    })
                }

                _ => Err(FLMError::EntityNotFound(rules.filter_id as i64)),
            }
        })
    }

    pub(crate) fn save_disabled_rules(
        &self,
        connection_manager: &DbConnectionManager,
        filter_id: FilterId,
        disabled_rules: Vec<String>,
    ) -> FLMResult<()> {
        connection_manager.execute_db(move |mut conn: Connection| {
            let rules_list_repository = RulesListRepository::new();

            let rules_lists_count = rules_list_repository
                .count(
                    &conn,
                    Some(SQLOperator::FieldEqualValue("filter_id", filter_id.into())),
                )
                .map_err(FLMError::from_database)?;

            if rules_lists_count == 0 {
                return Err(FLMError::EntityNotFound(filter_id as i64));
            }

            with_transaction(&mut conn, |transaction: &Transaction| {
                rules_list_repository
                    .set_disabled_rules(transaction, filter_id, disabled_rules.join("\n"))
                    .map(|_| ())
            })
        })
    }

    pub(crate) fn save_rules_to_file_blob<P: AsRef<Path>>(
        &self,
        connection_manager: &DbConnectionManager,
        filter_id: FilterId,
        file_path: P,
    ) -> FLMResult<()> {
        let file_already_exists = fs::metadata(&file_path).is_ok();

        let mut handler = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_path)
            .map_err(FLMError::from_io)?;

        connection_manager
            .execute_db(|connection: Connection| {
                let rules_repository = RulesListRepository::new();
                let (disabled_rules, blob) = rules_repository
                    .get_blob_handle_and_disabled_rules(&connection, filter_id)
                    .map_err(|why| match why {
                        Error::QueryReturnedNoRows => FLMError::EntityNotFound(filter_id as i64),
                        err => FLMError::from_database(err),
                    })?;

                let disabled_rules_set = disabled_rules
                    .split(|i| i == &LF_BYTES_SLICE)
                    .map(|value| value.to_vec())
                    .collect::<HashSet<Vec<u8>>>();

                write_to_stream(&mut handler, blob, disabled_rules_set)?;

                Ok(())
            })
            .map_err(|why| {
                drop(handler);
                if !file_already_exists {
                    fs::remove_file(&file_path).unwrap_or(());
                }

                why
            })
    }

    /// Determines if string is a rule
    /// Returns true if string is a rule, false otherwise
    pub(crate) fn is_line_is_rule(&self, line: &str) -> bool {
        !(line.is_empty()
            || line.starts_with(NON_RULE_MARKER)
            || line.starts_with(EXTRA_COMMENT_MARKER))
    }

    /// Calculates rules count in rules list
    /// Returns new rules list
    pub(crate) fn update_rules_count(&self, rules: FilterListRules) -> RulesListEntity {
        let rules_count = rules
            .rules
            .iter()
            .filter(|line| self.is_line_is_rule(line))
            .count() as i32;

        let mut new_entity: RulesListEntity = rules.into();

        new_entity.rules_count = rules_count;

        new_entity
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::constants::USER_RULES_FILTER_LIST_ID;
    use crate::storage::entities::rules_list_entity::RulesListEntity;
    use crate::FilterListRules;

    use super::RulesListService;

    #[test]
    fn test_update_rules_count() {
        let filter_id = USER_RULES_FILTER_LIST_ID;
        let text = "Text\n!Text\n# Text\n\n\nText".to_string();
        let disabled_text = "Disabled Text".to_string();
        let rules_count = 0;

        let user_rules_count_result = 2;

        let rules = FilterListRules {
            filter_id,
            rules: text.split('\n').map(str::to_string).collect(),
            disabled_rules: disabled_text.split('\n').map(str::to_string).collect(),
            rules_count,
        };

        let RulesListEntity {
            filter_id: new_filter_id,
            text: new_text,
            disabled_text: new_disabled_text,
            rules_count: new_rules_count,
        } = RulesListService::new().update_rules_count(rules);

        assert_eq!(new_filter_id, filter_id);
        assert_eq!(new_text, text);
        assert_eq!(new_disabled_text, disabled_text);
        assert_eq!(new_rules_count, user_rules_count_result);
    }
}
