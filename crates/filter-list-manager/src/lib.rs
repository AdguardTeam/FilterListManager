//! # AdGuard filter list manager
//!
//! This crate will be useful for various operations with filter lists:
//! - Fetching
//! - Storing
//! - Calculating updates
//! - ... and more
//!
//!
//! ## Simple usage
//!
//! ```rust
//! use adguard_flm::{FilterListManagerImpl, FilterListManager, Configuration};
//!
//! let mut configuration = Configuration::default();
//! configuration.metadata_url = "https://filters.adtidy.org/extension/safari/filters.json".to_string();
//! configuration.metadata_locales_url = "https://filters.adtidy.org/extension/safari/filters_i18n.json".to_string();
//! configuration.locale = "pt_PT".to_string();
//!
//! let flm = FilterListManagerImpl::new(configuration);
//!
//! // Saving custom filter as trusted with custom title.
//! let filter = flm.install_custom_filter_list(
//!     String::from("https://example.com/my-custom-filter.txt"),
//!     true,
//!     Some(String::from("My Filter")),
//!     None
//! ).unwrap();
//!
//! // Enable it.
//! let _ = flm.enable_filter_lists(vec![filter.id], true)
//!     .unwrap();
//!
//! // ... somewhere else, I want to update filters.
//! let updated_filters = flm.update_filters(false, 0, false)
//!     .unwrap();
//!
//! // ... then get all filters.
//! let filters_list = flm.get_full_filter_lists();
//! ```

pub use crate::filters::parser::parser_error::FilterParserError;
pub use crate::io::error::IOError;
pub use crate::io::http::error::HttpClientError;
/// # Re-exports
pub use crate::manager::filter_list_manager_impl::FilterListManagerImpl;
pub use crate::manager::models::active_rules_info::ActiveRulesInfo;
pub use crate::manager::models::configuration::Configuration;
pub use crate::manager::models::configuration::FilterListType;
pub use crate::manager::models::configuration::Locale;
pub use crate::manager::models::filter_group::FilterGroup;
pub use crate::manager::models::filter_list_rules::FilterListRules;
pub use crate::manager::models::filter_tag::FilterTag;
pub use crate::manager::models::flm_error::FLMError;
pub use crate::manager::models::update_result::UpdateFilterError;
pub use crate::manager::models::FilterId;
pub use crate::manager::models::FilterListMetadata;
pub use crate::manager::models::FullFilterList;
pub use crate::manager::models::UpdateResult;
pub use crate::manager::FilterListManager;
pub use crate::storage::constants::*;

#[doc(hidden)]
pub mod filters;
#[doc(hidden)]
pub(crate) mod io;
pub mod manager;
pub mod storage;
#[cfg(test)]
mod test_utils;
pub(crate) mod utils;

/// Customized [`Result`] with error [`FLMError`]
pub type FLMResult<T> = Result<T, FLMError>;
