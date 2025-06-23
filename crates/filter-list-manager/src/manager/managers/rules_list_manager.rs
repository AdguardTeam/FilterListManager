use chrono::Utc;
use rusqlite::types::Value;
use rusqlite::Connection;
use rusqlite::Transaction;
use std::collections::HashSet;

use crate::filters::parser::filter_collector::FilterCollector;
use crate::filters::parser::filter_compiler::FilterCompiler;
use crate::filters::parser::filter_contents_provider::string_provider::StringProvider;
use crate::filters::parser::is_rule_detector::is_line_is_rule;
use crate::io::http::blocking_client::BlockingClient;
use crate::storage::entities::rules_list::disabled_rules_entity::DisabledRulesEntity;
use crate::storage::entities::rules_list::rules_count_entity::RulesCountEntity;
use crate::storage::entities::rules_list::rules_list_entity::RulesListEntity;
use crate::storage::repositories::filter_includes_repository::FilterIncludesRepository;
use crate::storage::repositories::filter_repository::FilterRepository;
use crate::storage::repositories::rules_list_repository::RulesListRepository;
use crate::storage::repositories::Repository;
use crate::storage::sql_generators::operator::SQLOperator;
use crate::storage::with_transaction;
use crate::storage::DbConnectionManager;
use crate::DisabledRulesRaw;
use crate::FLMError;
use crate::FLMResult;
use crate::FilterId;
use crate::FilterListRules;
use crate::FilterListRulesRaw;
use crate::RulesCountByFilter;
use crate::{ActiveRulesInfo, Configuration};

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
        configuration: &Configuration,
    ) -> FLMResult<Vec<ActiveRulesInfo>> {
        let (list, mut rules, includes_list) =
            connection_manager.execute_db(|conn: Connection| {
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
                    .select_mapped(
                        &conn,
                        Some(SQLOperator::FieldIn("filter_id", filter_ids.clone())),
                    )
                    .map_err(FLMError::from_database)?;

                let includes_list = FilterIncludesRepository::new()
                    .select_mapped(&conn, Some(SQLOperator::FieldIn("filter_id", filter_ids)))
                    .map_err(FLMError::from_database)?;

                Ok((enabled_filters, map, includes_list))
            })?;

        let mut active_rules: Vec<ActiveRulesInfo> = vec![];
        for filter_entity in list {
            if let Some(filter_id) = filter_entity.filter_id {
                if let Some(mut rule_entity) = rules.remove(&filter_id) {
                    if rule_entity.has_directives() {
                        let (new_body, new_count) = FilterCollector::new(configuration)
                            .collect_from_parts(
                                &rule_entity,
                                filter_entity.download_url.as_str(),
                                includes_list.get(&filter_id),
                            )
                            .map_err(FLMError::from_parser_error)?;

                        rule_entity.text = new_body;
                        rule_entity.rules_count = new_count;
                    }

                    let disabled_lines =
                        rule_entity.disabled_text.lines().collect::<HashSet<&str>>();

                    // Make a difference of rule_entity.text from rule_entity.disabled_text
                    let filtered_rules = rule_entity
                        .text
                        .lines()
                        .filter(|line| !disabled_lines.contains(*line))
                        .map(ToString::to_string)
                        .collect::<Vec<String>>();

                    active_rules.push(ActiveRulesInfo {
                        filter_id,
                        group_id: filter_entity.group_id,
                        is_trusted: filter_entity.is_trusted,
                        rules: filtered_rules,
                    });
                }
            }
        }

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
        configuration: &Configuration,
        ids: Vec<FilterId>,
    ) -> FLMResult<Vec<FilterListRulesRaw>> {
        let (mut result, these_includes, download_urls) =
            connection_manager.execute_db(|conn: Connection| {
                let download_urls_map = FilterRepository::new()
                    .select_download_urls(&conn, ids.iter(), ids.len())
                    .map_err(FLMError::from_database)?;

                let values: Vec<Value> = ids.into_iter().map(Into::into).collect();

                let includes = FilterIncludesRepository::new()
                    .select_mapped(
                        &conn,
                        Some(SQLOperator::FieldIn("filter_id", values.clone())),
                    )
                    .map_err(FLMError::from_database)?;

                let rules = RulesListRepository::new()
                    .select(&conn, Some(SQLOperator::FieldIn("filter_id", values)))
                    .map_err(FLMError::from_database)?;

                Ok((rules, includes, download_urls_map))
            })?;

        if let Some(ref mut rules) = result {
            for rule in rules.iter_mut() {
                if rule.has_directives() {
                    if let Some(download_url) = download_urls.get(&rule.filter_id) {
                        let mut collector = FilterCollector::new(configuration);
                        let (new_body, new_count) = collector
                            .collect_from_parts(
                                &rule,
                                download_url,
                                these_includes.get(&rule.filter_id),
                            )
                            .map_err(FLMError::from_parser_error)?;

                        rule.text = new_body;
                        rule.rules_count = new_count;
                    } else {
                        return Err(FLMError::from_display(format!(
                            "Could not find download url for {}",
                            rule.filter_id
                        )));
                    }
                }
            }
        }

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
            let mut rules_counts_in_rules = RulesListRepository::new()
                .get_rules_count(&conn, &ids)
                .map_err(FLMError::from_database)?;

            let rules_counts_in_includes = FilterIncludesRepository::new()
                .get_rules_count_for_filters(&conn, &ids)
                .map_err(FLMError::from_database)?;

            rules_counts_in_rules.iter_mut().for_each(|entity| {
                if let Some(rules_count) = rules_counts_in_includes.get(&entity.filter_id) {
                    entity.rules_count += rules_count.to_owned();
                }
            });

            Ok(rules_counts_in_rules)
        })?;

        let rules_count: Vec<RulesCountByFilter> = result.into_iter().map(Into::into).collect();

        Ok(rules_count)
    }

    /// Saves custom filter rules
    pub(crate) fn save_custom_filter_rules(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
        rules: FilterListRules,
    ) -> FLMResult<()> {
        let rules: FilterListRules = self.update_rules_count(rules);

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
                    let mut filter = filters.remove(0);
                    let http_client = BlockingClient::new(configuration)?;

                    let filter_id = rules.filter_id;
                    let rules_entity = RulesListEntity::from(rules);
                    let mut compiler = FilterCompiler::with_custom_provider(
                        Box::new(StringProvider::new(rules_entity.text, &http_client)),
                        configuration,
                    );

                    compiler
                        .compile(&filter.download_url)
                        .map_err(FLMError::from_parser_error)?;

                    let mut entities = compiler.into_entities(filter_id);
                    entities.rules_list_entity.disabled_text = rules_entity.disabled_text;

                    // TODO: do we need to update metadata here?
                    let _ = with_transaction(&mut conn, |tx: &Transaction| {
                        filter.last_update_time = Utc::now().timestamp();

                        filter_repository.insert(tx, &[filter])?;
                        FilterIncludesRepository::new()
                            .replace_entities_for_filters(tx, &entities.filter_includes_entities)?;

                        RulesListRepository::new().insert(tx, &[entities.rules_list_entity])
                    });

                    Ok(())
                }

                _ => Err(FLMError::EntityNotFound(rules.filter_id as i64)),
            }
        })
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
            .filter(|line| is_line_is_rule(line))
            .count() as i32;

        rules.rules_count = rules_count;

        rules
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
