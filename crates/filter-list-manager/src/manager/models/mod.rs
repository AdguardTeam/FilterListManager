//! Models associated with [`crate::FilterListManager`]
pub mod active_rules_info;
pub mod configuration;
pub mod disabled_rules_raw;
pub mod filter_group;
pub mod filter_list_metadata;
pub mod filter_list_metadata_with_body;
pub mod filter_list_rules;
pub mod filter_list_rules_raw;
pub mod filter_tag;
pub mod flm_error;
pub mod full_filter_list;
pub mod rules_count_by_filter;
pub mod stored_filter_metadata;
pub mod update_result;

pub use self::disabled_rules_raw::DisabledRulesRaw;
pub use self::filter_list_metadata::FilterListMetadata;
pub use self::filter_list_metadata_with_body::FilterListMetadataWithBody;
pub use self::filter_list_rules_raw::FilterListRulesRaw;
pub use self::flm_error::FLMError;
pub use self::full_filter_list::FullFilterList;
pub use self::update_result::UpdateResult;

/// Filter list id type alias
pub type FilterId = i32;
