use crate::FilterId;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct DiffUpdateEntity {
    /// Related filter entity id
    pub(crate) filter_id: FilterId,
    /// Next patch path
    pub(crate) next_path: String,
    /// The time, when we should go after patch contents
    pub(crate) next_check_time: i64,
}
