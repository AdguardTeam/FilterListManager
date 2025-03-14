use crate::storage::entities::rules_list_entity::RulesListEntity;
use crate::storage::services::constants::EXTRA_COMMENT_MARKER;
use crate::storage::services::constants::NON_RULE_MARKER;
use crate::FilterListRules;

/// Service for rules list repository
pub(crate) struct RulesListService;

impl RulesListService {
    pub(crate) const fn new() -> Self {
        Self {}
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
