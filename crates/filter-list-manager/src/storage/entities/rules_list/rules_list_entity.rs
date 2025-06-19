use rusqlite::{Result, Row};

use crate::manager::models::filter_list_rules::FilterListRules;
use crate::manager::models::filter_list_rules_raw::FilterListRulesRaw;
use crate::manager::models::FilterId;
use crate::storage::entities::hydrate::Hydrate;
use crate::string;

/// We should treat all new filters as "possibly having" directives.
pub(crate) const DEFAULT_HAS_DIRECTIVES_VALUE: bool = true;

/// Entity for rules_list table
#[derive(Clone)]
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub(crate) struct RulesListEntity {
    pub(crate) filter_id: FilterId,
    pub(crate) text: String,
    pub(crate) disabled_text: String,
    pub(crate) rules_count: i32,
    pub(in crate::storage) has_directives: bool,
}

impl RulesListEntity {
    /// Creates new instance of [RulesListEntity].
    /// `disabled_text` is empty by default
    pub(crate) const fn new(filter_id: FilterId, text: String, rules_count: i32) -> Self {
        RulesListEntity {
            filter_id,
            text,
            disabled_text: string!(),
            rules_count,
            has_directives: DEFAULT_HAS_DIRECTIVES_VALUE,
        }
    }

    pub(crate) fn set_has_directives(&mut self, has_directives: bool) {
        self.has_directives = has_directives;
    }

    pub(crate) fn has_directives(&self) -> bool {
        self.has_directives
    }
}

impl RulesListEntity {
    #[cfg(test)]
    pub(crate) const fn with_disabled_text(
        filter_id: FilterId,
        text: String,
        disabled_text: String,
        rules_count: i32,
    ) -> Self {
        RulesListEntity {
            filter_id,
            text,
            disabled_text,
            rules_count,
            has_directives: DEFAULT_HAS_DIRECTIVES_VALUE,
        }
    }
}

impl Hydrate for RulesListEntity {
    fn hydrate(row: &Row) -> Result<RulesListEntity> {
        Ok(RulesListEntity {
            filter_id: row.get(0)?,
            text: row.get(1)?,
            disabled_text: row.get(2)?,
            rules_count: row.get(3)?,
            has_directives: row.get(4)?,
        })
    }
}

impl From<RulesListEntity> for FilterListRules {
    fn from(value: RulesListEntity) -> Self {
        FilterListRules {
            filter_id: value.filter_id,
            rules: value.text.lines().map(str::to_string).collect(),
            disabled_rules: value.disabled_text.lines().map(str::to_string).collect(),
            rules_count: value.rules_count,
        }
    }
}

impl From<FilterListRules> for RulesListEntity {
    fn from(value: FilterListRules) -> Self {
        Self {
            filter_id: value.filter_id,
            text: value.rules.join("\n"),
            disabled_text: value.disabled_rules.join("\n"),
            rules_count: value.rules_count,
            has_directives: DEFAULT_HAS_DIRECTIVES_VALUE,
        }
    }
}

impl From<RulesListEntity> for FilterListRulesRaw {
    fn from(value: RulesListEntity) -> Self {
        FilterListRulesRaw {
            filter_id: value.filter_id,
            rules: value.text,
            disabled_rules: value.disabled_text,
            rules_count: value.rules_count,
        }
    }
}
