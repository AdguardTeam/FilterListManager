use crate::filters::parser::rule_lines_collector::RuleLinesCollector;
use crate::storage::entities::rules_list_entity::RulesListEntity;

pub(crate) struct RulesListService;

impl RulesListService {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    pub(crate) fn update_rules_count(&self, entity: RulesListEntity) -> RulesListEntity {
        let mut rule_lines_collector = RuleLinesCollector::new();

        entity
            .text
            .split('\n')
            .for_each(|line| rule_lines_collector.increment_rules_count(line));

        let mut new_entity = entity;

        new_entity.rules_count = rule_lines_collector.get_rules_count();

        new_entity
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::constants::USER_RULES_FILTER_LIST_ID;
    use crate::storage::entities::rules_list_entity::RulesListEntity;
    use crate::Configuration;

    use super::RulesListService;

    #[test]
    fn test_update_rules_count() {
        let filter_id = USER_RULES_FILTER_LIST_ID;
        let text = "Text\n!Text\n# Text\n\n\nText".to_string();
        let disabled_text = "Disabled Text".to_string();
        let rules_count = 0;

        let user_rules_count_result = 2;

        let entity = RulesListEntity {
            filter_id,
            text: text.clone(),
            disabled_text: disabled_text.clone(),
            rules_count,
        };

        let RulesListEntity {
            filter_id: new_filter_id,
            text: new_text,
            disabled_text: new_disabled_text,
            rules_count: new_rules_count,
        } = RulesListService::new().update_rules_count(entity);

        assert_eq!(new_filter_id, filter_id);
        assert_eq!(new_text, text);
        assert_eq!(new_disabled_text, disabled_text);
        assert_eq!(new_rules_count, user_rules_count_result);
    }
}
