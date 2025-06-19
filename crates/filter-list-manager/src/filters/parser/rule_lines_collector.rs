use crate::filters::parser::is_rule_detector::is_line_is_rule;

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

    /// Pushes rule to collected rule lines
    pub fn push(&mut self, rule: String) {
        self.collected_lines.push(rule);
    }

    /// Returns collected rules as joined string
    pub fn into_body(self) -> String {
        self.collected_lines.join("\n")
    }

    /// Increments rules count if line is rule
    pub fn increment_rules_count(&mut self, line: &str) {
        self.rules_count += i32::from(is_line_is_rule(line))
    }

    /// Returns rules count
    pub fn get_rules_count(&self) -> i32 {
        self.rules_count
    }
}
