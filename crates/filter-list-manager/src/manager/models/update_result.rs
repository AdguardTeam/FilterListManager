//! Filters update result models.
use crate::FilterId;
use crate::FullFilterList;

/// Filters update result container.
pub struct UpdateResult {
    /// Currently updated filters.
    /// `title` and `description` fields will be localised with current [`crate::Locale`]
    pub updated_list: Vec<FullFilterList>,
    /// Number of filters not updated due to timeout.
    pub remaining_filters_count: i32,
    /// List of entities containing the filter id and a string representation of the error
    /// that occurred when processing or receiving the filter
    pub filters_errors: Vec<UpdateFilterError>,
}

/// Container for filter updating error
pub struct UpdateFilterError {
    /// ID of that filter tha couldn't be updated
    pub filter_id: FilterId,
    /// Filter error converted to a string. For debugging purposes
    pub message: String,
    /// Filter url
    pub filter_url: Option<String>,
    /// Http client error
    pub http_client_error: Option<String>,
}
