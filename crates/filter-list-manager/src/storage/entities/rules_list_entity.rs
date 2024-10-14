use crate::manager::models::filter_list_rules::FilterListRules;
use crate::manager::models::filter_list_rules_raw::FilterListRulesRaw;
use crate::manager::models::FilterId;

#[derive(Clone)]
pub(crate) struct RulesListEntity {
    pub(crate) filter_id: FilterId,
    pub(crate) text: String,
    pub(crate) disabled_text: String,
}

impl Into<FilterListRules> for RulesListEntity {
    fn into(self) -> FilterListRules {
        FilterListRules {
            filter_id: self.filter_id,
            rules: self.text.lines().map(str::to_string).collect(),
            disabled_rules: self.disabled_text.lines().map(str::to_string).collect(),
        }
    }
}

impl From<FilterListRules> for RulesListEntity {
    fn from(value: FilterListRules) -> Self {
        Self {
            filter_id: value.filter_id,
            text: value.rules.join("\n"),
            disabled_text: value.disabled_rules.join("\n"),
        }
    }
}

impl Into<FilterListRulesRaw> for RulesListEntity {
    fn into(self) -> FilterListRulesRaw {
        FilterListRulesRaw {
            filter_id: self.filter_id,
            rules: self.text,
            disabled_rules: self.disabled_text,
        }
    }
}
