//! # Filter list manager library core
//!
//! ## Overview
//!
//! This crate represents a library for managing AdGuard filter lists.
//!
//! This library can:
//!
//! - Fetch filter lists
//! - Store the downloaded filter list
//! - Perform filter lists updates
//! - ... and more
//!
//! ## Filters analysis
//!
//! ### List of meta tags that the library parses from filter content
//!
//! - `! Title` - Name of the filter.
//! - `! Description` - Detailed description of the filter.
//! - `! Version` - Current version of the filter.
//! - `! Expires` - Filter expiration period. Will be converted into seconds. [See the tests for an example](./src/filters/parser/metadata/parsers/expires.rs) If this field is missing in the metadata, the global value from the [configuration](./src/manager/models/configuration/mod.rs) will be used. Before updating the filter, the value will be checked and aligned to the lower boundary ([3600](./src/manager/models/configuration/mod.rs)) if it is less than this value.
//! - `! Homepage` - Filter website/homepage.
//! - `! TimeUpdated` - When this filter was updated in registry. Format: `2024-08-13T13:30:53+00:00`.
//! - `! Last modified` - Alias for `TimeUpdated`. Format: `2024-08-13T12:01:26.703Z`. You can choose one format for both fields.
//! - `! Diff-Path` - [Differential updates](https://github.com/ameshkov/diffupdates?tab=readme-ov-file#-diff-path) information
//! - `! License` - Link to filter license.
//! - `! Checksum` - Filter's base64(md5-checksum). Before update/install filter, checksum will be calculated and compared. See the source [here](./src/filters/parser/checksum_validator.rs)
//!
//! ### List of filter preprocessor directives supported by the library
//!
//! [See AdGuard preprocessor directives](https://adguard.com/kb/general/ad-filtering/create-own-filters/#preprocessor-directives)
//!
//! The library supports:
//!
//! - `!#include file_path` - Includes contents of file into filter and process. `file_path` must be:
//! - Absolute url with the [same origin](https://developer.mozilla.org/en-US/docs/Web/Security/Same-origin_policy) as the parent filter.
//! - Relative url.
//! - File url (only if the parent filter's url has `file` scheme).
//! - `!#if/!#endif/!#else` - Condition compilation directives. They can be nested. Supported tokens:
//! - `()` - parentheses
//! - `true/false` - boolean values
//! - `&& ||` - AND/OR operators
//! - `!` - NOT operator
//! - Literal compiler constant from [configuration](./src/manager/models/configuration). For example, `windows`,`mac`, etc... It works like this: if the constant encountered is in the `configuration.compiler_conditional_constants` list, then the condition becomes **true**, **false** otherwise
//!
//! **See the tests for more information:**
//!
//! - [All directives](./src/filters/parser.rs)
//! - [!#include](./src/filters/parser/include_processor.rs)
//! - [!#if / !#endif / !#else](./src/filters/parser/boolean_expression_parser.rs)
//!
//! ## Usage
//!
//! ### Create and setup configuration for library facade
//!
//!
//! // Every instance of FilterListManager must have its own configuration
//! `let mut configuration = Configuration::default();`
//!
//! // Sets urls for filters indices.
//! `configuration.metadata_url = "https://filters.adtidy.org/extension/safari/filters.json".to_string();`
//! `configuration.metadata_locales_url = "https://filters.adtidy.org/extension/safari/filters_i18n.json".to_string();`
//!
//! // Sets locale. Will be used for returning localized strings for filters,
//! // groups, and tags, where applicable.
//! `configuration.locale = "pt_PT".to_string();`
//!
//! // Sets type of filters lists.
//! // By default, FilterListType::STANDARD will be selected.
//! configuration.filter_list_type = FilterListType::DNS;
//!
//! // Creates facade instance
//! `let flm = FilterListManagerImpl::new(configuration);`
//!
//!
//! #### Example references
//!
//! [Configuration reference](./src/manager/models/configuration/mod.rs)\
//! [FilterListManager reference](./src/manager/mod.rs)
//!
//! ---
//!
//! ### How to create and fill up standard filters database
//!
//!
//! // Creates and configures the database. Populates the database with information
//! // from the filter indexes (filters metadata), the paths to which are specified
//! // in the configuration.
//! // In addition, this method applies migrations that have not yet been applied.
//! // See the lift_up_database method for details on "lifting" a database.
//! `flm.pull_metadata();`
//!
//! // Then, downloads the contents of the filters.
//! `flm.update_filters(true, 0, true);`
//!
//!
//! > [!NOTE]
//! > By default, the application operates with a database located in the current
//! > working directory (**cwd**), and the database file name is generated based on
//! > the format `agflm_{configuration.filter_list_type.to_string()}`. For standard
//! > filters, the file path will be `$CWD/agflm_standard.db`.
//!
//! ---
//!
//! ### Database scheme updates
//!
//! Database schema updates (migrations) are possible using the `flm.lift_up_database()` method.
//! The method “raises” the state of the database to the working state.
//!
//! **If the database doesn't exist:**
//! - Creates database
//! - Rolls up the schema
//! - Rolls migrations
//! - Performs bootstrap.
//!
//! **If the database is an empty file:**
//! - Rolls the schema
//! - Rolls migrations
//! - Performs bootstrap.
//!
//! ... and so on.
//!
//! ### Usage notes
//!
//! First connection to database almost always "lift" the database.
//! So you need this only in special cases, like old database backups
//!
//! ---
//!
//! ### Operations with custom filters
//!
//! The library categorizes all filters into three types:
//!
//! 1. **Index Filters** - Filters created by parsing the index (registry).
//! 2. **Custom Filters** - Filters added (and edited) by the user using the
//! library's methods.
//! 3. **Special Filters** - Custom filters preconfigured by the library's scripts.
//!
//! You can refer to the [db constants file][constants] to check the indicators for
//! special and custom filters.
//!
//! // Installs a custom filter.
//! ```
//! use adguard_flm::{Configuration, FilterListManagerImpl, FilterListManager};
//! let flm = FilterListManagerImpl::new(Configuration::default())
//!     .unwrap();
//!
//! let custom_filter = flm.install_custom_filter_list(
//!     String::from("https://example.com/custom_filter.txt"),
//!     true, // The filter list is marked as trusted.
//!     Some(String::from("Custom title")),
//!     Some(String::from("Custom description"))
//! ).unwrap();
//!
//! // Edit metadata.
//! flm.update_custom_filter_metadata(
//!     custom_filter.id,
//!     String::from("new title"),
//!     false // The filter list is marked as not trusted.
//! ).unwrap();
//!
//! // Turn on this filter.
//! flm.enable_filter_lists(vec![custom_filter.id], true).unwrap();
//!
//! // Remove this filter.
//! flm.delete_custom_filter_lists(vec![custom_filter.id]).unwrap();
//! ```
//!
//! #### Installing a custom filter from a string instead of downloading it
//!
//! ```
//! use adguard_flm::{Configuration, FilterListManagerImpl, FilterListManager};
//! let flm = FilterListManagerImpl::new(Configuration::default())
//!     .unwrap();
//!
//! let string_contents = String::from(r###"
//! ! Checksum: ecbiyIyplBZKLeNzi64pGA
//! ...
//! ! JS API START
//! #%#var AG_onLoad=function(func){if(document.readyState==="complete"||document.readyState==="interactive")func();else
//! ...
//! "###);
//!
//! flm.install_custom_filter_from_string(
//!     String::new(), // download url
//!     1719505304i64, // last_download_time value. Explanation: Can we update filter? Answer: (filter.last_download_time + filter.expires < now())
//!     true, // Enabled
//!     true, // Trusted
//!     string_contents, // Filter body
//!     None, // Filter title - Option<String>
//!     None  // Filter description - Option<String>
//! ).unwrap();
//! ```
//!
//! [constants]: ./crates/filter-list-manager/src/storage/constants.rs
//!
//! #### Operations with custom filters rules
//!
//! ```
//! // Saves the structure containing the filter rules.
//! flm.save_custom_filter_rules(/* FilterListRules */ rules_for_new_local_custom_filter);
//!
//! // You can save only disabled rules for the filter list
//! flm.save_disabled_rules(filter.id, /* Vec<String> */ disabled_rules_list);
//! ```
//!
//! #### Example references
//!
//! [FilterListRules reference](./src/manager/models/filter_list_rules.rs)
//!
//! ---
//!
//! ### Get operations
//!
//! ```
//! // Retrieves all filters metadata from the database **with** theirs rules.
//! // Returns Vec<FullFilterList>.
//! flm.get_full_filter_lists();
//!
//! // Retrieves a filter metadata by its ID from the database **with** its rules.
//! // Returns Optional<FullFilterList>.
//! flm.get_full_filter_list_by_id(id);
//!
//! // Retrieves all enabled filters as ActiveRulesInfo.
//! flm.get_active_filters();
//!
//! // Retrieves all filters metadata from the database **without** theirs rules.
//! // Returns Vec<StoredFilterMetadata>
//! flm.get_stored_filters_metadata();
//!
//! // Retrieves a filter metadata by its ID from the database **without** its rules.
//! // Returns Optional<StoredFilterMetadata>.
//! flm.get_stored_filter_metadata_by_id(id);
//! ```
//!
//! #### Example references
//!
//! [FullFilterList reference](./src/manager/models/full_filter_list.rs)\
//! [StoredFilterMetadata reference](./src/manager/models/stored_filter_metadata.rs)\
//! [ActiveRulesInfo reference](./src/manager/models/active_rules_info.rs)
//!
//! ### Other (All) operations
//!
//! [Facade Interface](./src/manager/mod.rs)
//!

pub use crate::filters::parser::parser_error::FilterParserError;
pub use crate::io::error::IOError;
pub use crate::io::http::error::HttpClientError;
/// # Re-exports
pub use crate::manager::filter_list_manager_impl::FilterListManagerImpl;
pub use crate::manager::models::active_rules_info::ActiveRulesInfo;
pub use crate::manager::models::configuration::Configuration;
pub use crate::manager::models::configuration::FilterListType;
pub use crate::manager::models::configuration::Locale;
pub use crate::manager::models::configuration::RequestProxyMode;
pub use crate::manager::models::disabled_rules_raw::DisabledRulesRaw;
pub use crate::manager::models::filter_group::FilterGroup;
pub use crate::manager::models::filter_list_rules::FilterListRules;
pub use crate::manager::models::filter_list_rules_raw::FilterListRulesRaw;
pub use crate::manager::models::filter_tag::FilterTag;
pub use crate::manager::models::flm_error::FLMError;
pub use crate::manager::models::stored_filter_metadata::StoredFilterMetadata;
pub use crate::manager::models::update_result::UpdateFilterError;
pub use crate::manager::models::FilterId;
pub use crate::manager::models::FilterListMetadata;
pub use crate::manager::models::FilterListMetadataWithBody;
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
