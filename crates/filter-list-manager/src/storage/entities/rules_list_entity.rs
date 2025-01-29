use crate::manager::models::filter_list_rules::FilterListRules;
use crate::manager::models::filter_list_rules_raw::FilterListRulesRaw;
use crate::manager::models::FilterId;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub(crate) struct RulesListEntity {
    pub(crate) filter_id: FilterId,
    pub(crate) text: String,
    pub(crate) disabled_text: String,
}

impl From<RulesListEntity> for FilterListRules {
    fn from(value: RulesListEntity) -> Self {
        FilterListRules {
            filter_id: value.filter_id,
            rules: value.text.lines().map(str::to_string).collect(),
            disabled_rules: value.disabled_text.lines().map(str::to_string).collect(),
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

impl From<RulesListEntity> for FilterListRulesRaw {
    fn from(value: RulesListEntity) -> Self {
        FilterListRulesRaw {
            filter_id: value.filter_id,
            rules: value.text,
            disabled_rules: value.disabled_text,
        }
    }
}
