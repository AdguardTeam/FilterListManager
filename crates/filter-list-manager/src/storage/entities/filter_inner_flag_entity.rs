use crate::FilterId;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct FilterInnerFlagEntity {
    pub filter_id: FilterId,
    pub is_user_title: Option<bool>,
    pub is_user_description: Option<bool>,
}
