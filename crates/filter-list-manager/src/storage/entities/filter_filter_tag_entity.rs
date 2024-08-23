use crate::manager::models::FilterId;

/// Filter Tag relation entity
pub(crate) struct FilterFilterTagEntity {
    pub(crate) filter_id: FilterId,
    pub(crate) tag_id: i32,
}
