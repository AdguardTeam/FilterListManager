use crate::manager::models::FilterId;
use crate::storage::entities::rules_list_entity::RulesListEntity;
use crate::storage::services::rules_list_service::RulesListService;

/// Used for rules lines collection, then turns to string joined with new lines
#[derive(Default)]
pub(crate) struct RuleLinesCollector {
    collected_lines: Vec<String>,
    rules_count: i32,
}

impl RuleLinesCollector {
    pub const fn new() -> Self {
        Self {
            collected_lines: Vec::new(),
            rules_count: 0,
        }
    }

    pub fn push(&mut self, rule: String) {
        self.collected_lines.push(rule);
    }

    pub fn extract_rule_entity(&mut self, id: FilterId) -> RulesListEntity {
        let text = self.collected_lines.join("\n");

        self.collected_lines.clear();
        self.collected_lines.shrink_to_fit();

        let rules_count = self.rules_count;

        RulesListEntity {
            filter_id: id,
            text,
            disabled_text: String::new(),
            rules_count,
        }
    }

    pub fn increment_rules_count(&mut self, line: &str) {
        self.rules_count += i32::from(RulesListService::new().is_line_is_rule(line))
    }

    pub fn get_rules_count(&self) -> i32 {
        self.rules_count
    }
}
