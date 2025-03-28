use chrono::Utc;
use rusqlite::types::Value;
use rusqlite::Connection;
use rusqlite::Transaction;

use crate::storage::entities::filter::filter_entity::FilterEntity;
use crate::storage::entities::rules_list::disabled_rules_entity::DisabledRulesEntity;
use crate::storage::entities::rules_list::rules_count_entity::RulesCountEntity;
use crate::storage::entities::rules_list::rules_list_entity::RulesListEntity;
use crate::storage::repositories::filter_repository::FilterRepository;
use crate::storage::repositories::rules_list_repository::MapFilterIdOnRulesList;
use crate::storage::repositories::rules_list_repository::RulesListRepository;
use crate::storage::repositories::Repository;
use crate::storage::sql_generators::operator::SQLOperator;
use crate::storage::with_transaction;
use crate::storage::DbConnectionManager;
use crate::ActiveRulesInfo;
use crate::DisabledRulesRaw;
use crate::FLMError;
use crate::FLMResult;
use crate::FilterId;
use crate::FilterListRules;
use crate::FilterListRulesRaw;
use crate::RulesCountByFilter;

/// This marks line as "non-rule" line: comment, directive, etc.
pub const NON_RULE_MARKER: char = '!';

/// Also, comment line can start from "# " sequence
pub const EXTRA_COMMENT_MARKER: &str = "# ";

/// Manager for rules list logic
pub(crate) struct RulesListManager;

impl RulesListManager {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    /// Gets active rules
    pub(crate) fn get_active_rules(
        &self,
        connection_manager: &DbConnectionManager,
    ) -> FLMResult<Vec<ActiveRulesInfo>> {
        let (list, mut rules): (Vec<FilterEntity>, MapFilterIdOnRulesList) = connection_manager
            .execute_db(|conn: Connection| {
                let enabled_filters = FilterRepository::new()
                    .select(
                        &conn,
                        Some(SQLOperator::FieldEqualValue("is_enabled", true.into())),
                    )
                    .map_err(FLMError::from_database)?
                    .unwrap_or_default();

                let filter_ids = enabled_filters
                    .iter()
                    .filter_map(|entity| entity.filter_id)
                    .map(Into::into)
                    .collect::<Vec<Value>>();

                let map = RulesListRepository::new()
                    .select_mapped(&conn, Some(SQLOperator::FieldIn("filter_id", filter_ids)))
                    .map_err(FLMError::from_database)?;

                Ok((enabled_filters, map))
            })?;

        let active_rules: Vec<ActiveRulesInfo> = list
            .into_iter()
            .flat_map(|filter_entity: FilterEntity| {
                if let Some(filter_id) = filter_entity.filter_id {
                    if let Some(rule_entity) = rules.remove(&filter_id) {
                        if rule_entity.filter_id == filter_id {
                            let disabled_lines =
                                rule_entity.disabled_text.lines().collect::<Vec<&str>>();

                            // Make a difference of rule_entity.text from rule_entity.disabled_text
                            let filtered_rules = rule_entity
                                .text
                                .lines()
                                .filter(|line| {
                                    !disabled_lines
                                        .iter()
                                        .any(|line_from_disabled| line_from_disabled == line)
                                })
                                .map(ToString::to_string)
                                .collect::<Vec<String>>();

                            return Some(ActiveRulesInfo {
                                filter_id,
                                group_id: filter_entity.group_id,
                                is_trusted: filter_entity.is_trusted,
                                rules: filtered_rules,
                            });
                        }
                    }
                }

                None
            })
            .collect();

        Ok(active_rules)
    }

    /// Gets disabled rules
    pub(crate) fn get_disabled_rules(
        &self,
        connection_manager: &DbConnectionManager,
        ids: Vec<FilterId>,
    ) -> FLMResult<Vec<DisabledRulesRaw>> {
        let result: Vec<DisabledRulesEntity> =
            connection_manager.execute_db(|conn: Connection| {
                RulesListRepository::new()
                    .get_disabled_rules_by_ids(&conn, &ids)
                    .map_err(FLMError::from_database)
            })?;

        let disabled_rules: Vec<DisabledRulesRaw> = result.into_iter().map(Into::into).collect();

        Ok(disabled_rules)
    }

    /// Gets filter rules by ids
    pub(crate) fn get_filter_rules_as_strings(
        &self,
        connection_manager: &DbConnectionManager,
        ids: Vec<FilterId>,
    ) -> FLMResult<Vec<FilterListRulesRaw>> {
        let result: Option<Vec<RulesListEntity>> =
            connection_manager.execute_db(|conn: Connection| {
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

        let filter_rules_as_string: Vec<FilterListRulesRaw> = result
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(filter_rules_as_string)
    }

    /// Gets rules count
    pub(crate) fn get_rules_count(
        &self,
        connection_manager: &DbConnectionManager,
        ids: Vec<FilterId>,
    ) -> FLMResult<Vec<RulesCountByFilter>> {
        let result: Vec<RulesCountEntity> = connection_manager.execute_db(|conn: Connection| {
            RulesListRepository::new()
                .get_rules_count(&conn, &ids)
                .map_err(FLMError::from_database)
        })?;

        let rules_count: Vec<RulesCountByFilter> = result.into_iter().map(Into::into).collect();

        Ok(rules_count)
    }

    /// Saves custom filter rules
    pub(crate) fn save_custom_filter_rules(
        &self,
        connection_manager: &DbConnectionManager,
        rules: FilterListRules,
    ) -> FLMResult<()> {
        let rules: FilterListRules = self.update_rules_count(rules);
        let _ = connection_manager.execute_db(move |mut conn: Connection| {
            let filter_repository = FilterRepository::new();

            let result = filter_repository
                .select(
                    &conn,
                    Some(FilterRepository::custom_filter_with_id(rules.filter_id)),
                )
                .map_err(FLMError::from_database)?;

            match result {
                Some(mut filters) if !filters.is_empty() => {
                    let _ = with_transaction(&mut conn, |tx: &Transaction| {
                        // SAFETY: index "0" always present in this branch until condition
                        // `!filters.is_empty()` is met.
                        let filter = unsafe { filters.get_unchecked_mut(0) };

                        filter.last_update_time = Utc::now().timestamp();

                        filter_repository.insert(tx, &filters)?;

                        RulesListRepository::new().insert(tx, &[rules.into()])
                    });

                    Ok(())
                }

                _ => Err(FLMError::EntityNotFound(rules.filter_id as i64)),
            }
        })?;

        Ok(())
    }

    /// Saves disabled rules
    pub(crate) fn save_disabled_rules(
        &self,
        connection_manager: &DbConnectionManager,
        filter_id: FilterId,
        disabled_rules: Vec<String>,
    ) -> FLMResult<()> {
        let _ = connection_manager.execute_db(move |mut conn: Connection| {
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

            let _ = with_transaction(&mut conn, |transaction: &Transaction| {
                rules_list_repository
                    .set_disabled_rules(transaction, filter_id, disabled_rules.join("\n"))
                    .map(|_| ())
            });

            Ok(())
        })?;

        Ok(())
    }
}

impl RulesListManager {
    /// Calculates rules count in rules list
    fn update_rules_count(&self, mut rules: FilterListRules) -> FilterListRules {
        let rules_count = rules
            .rules
            .iter()
            .filter(|line| RulesListManager::is_line_is_rule(line))
            .count() as i32;

        rules.rules_count = rules_count;

        rules
    }
}

impl RulesListManager {
    /// Determines if string is a rule
    pub(crate) fn is_line_is_rule(line: &str) -> bool {
        !(line.is_empty()
            || line.starts_with(NON_RULE_MARKER)
            || line.starts_with(EXTRA_COMMENT_MARKER))
    }
}

#[cfg(test)]
mod tests {
    use crate::manager::managers::rules_list_manager::RulesListManager;
    use crate::storage::constants::USER_RULES_FILTER_LIST_ID;
    use crate::FilterListRules;

    #[test]
    fn test_update_rules_count() {
        let filter_id = USER_RULES_FILTER_LIST_ID;
        let rules: Vec<String> = "Text\n!Text\n# Text\n\n\nText"
            .to_string()
            .split('\n')
            .map(str::to_string)
            .collect();
        let disabled_rules: Vec<String> = "Disabled Text"
            .to_string()
            .split('\n')
            .map(str::to_string)
            .collect();
        let rules_count = 0;

        let user_rules_count_result = 2;

        let filter_list_rules = FilterListRules {
            filter_id,
            rules: rules.clone(),
            disabled_rules: disabled_rules.clone(),
            rules_count,
        };

        let FilterListRules {
            filter_id: new_filter_id,
            rules: new_rules,
            disabled_rules: new_disabled_rules,
            rules_count: new_rules_count,
        } = RulesListManager::new().update_rules_count(filter_list_rules);

        assert_eq!(new_filter_id, filter_id);
        assert_eq!(new_rules, rules);
        assert_eq!(new_disabled_rules, disabled_rules);
        assert_eq!(new_rules_count, user_rules_count_result);
    }
}
