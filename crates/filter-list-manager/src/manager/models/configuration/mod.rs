//! Configuration-related objects for [`crate::FilterListManager`]
pub mod filter_list_type;
pub mod locale;

pub use self::filter_list_type::FilterListType;
pub use self::locale::Locale;
use std::cmp::max;

/// Expires value shouldn't be less than this constant. In seconds
const MINIMAL_EXPIRES_VALUE: i32 = 3600;

/// Default https?-requests timeout in ms.
const DEFAULT_REQUEST_TIMEOUT_MS: i32 = 60000;

/// Default `! Expires` value for downloaded filter lists.
/// Will be used, if value is not set in filter's metadata in seconds
const DEFAULT_EXPIRES_VALUE_FOR_FILTERS: i32 = 86400;

/// Configuration object
pub struct Configuration {
    /// Type of filter lists to manage
    pub filter_list_type: FilterListType,
    /// Absolute path for library working directory.
    /// This will be used for database operating.
    /// if value is [`None`] `cwd` will be used
    pub working_directory: Option<String>,
    /// [`Locale`] is the locale that needs to be used to extract localized names and descriptions.
    /// Locale `en-GB` will be normalized to internal `en_GB` representation.
    /// Default value: en
    pub locale: Locale,
    /// Default period for expires in seconds (unless specified in "Expires",
    /// or its value is too small).
    /// Default value: 86400
    /// Values < 3600 will be clamped to 3600
    pub default_filter_list_expires_period_sec: i32,
    /// List of literal constants for filters conditional compilation
    pub compiler_conditional_constants: Option<Vec<String>>,
    /// URL of the index (filters.json) file
    pub metadata_url: String,
    /// URL of the locales (filters_i18n.json) file
    pub metadata_locales_url: String,
    /// Requests timeouts in milliseconds. Default value 60000
    pub request_timeout_ms: i32,
    /// “Uplifting” a database is a set of measures that brings the database up to date:
    /// * Database creation
    /// * Filling with schema
    /// * Creation of service tables and entities
    /// * Migrating between versions of a given library
    ///
    /// If you want to disable this option, you will need to manually call `flm.lift_up_database()`
    /// when you update the library in your application.
    pub auto_lift_up_database: bool,
}

/// Normalized locales delimiter
pub(crate) const LOCALES_DELIMITER: &str = "_";

impl Configuration {
    /// Normalizing configuration before we can work with it
    pub(crate) fn normalized(&mut self) {
        self.locale = Configuration::normalize_locale_string(&self.locale);
    }

    /// Normalize locale string
    pub(crate) fn normalize_locale_string(locale: &Locale) -> Locale {
        locale.replace("-", LOCALES_DELIMITER)
    }

    /// We shouldn't propagate values less than 3600
    ///
    /// * `filter_expires` - `filter.expires` value
    pub(crate) fn resolve_right_expires_value(&self, filter_expires: i32) -> i32 {
        if filter_expires < MINIMAL_EXPIRES_VALUE {
            max(
                self.default_filter_list_expires_period_sec,
                MINIMAL_EXPIRES_VALUE,
            )
        } else {
            filter_expires
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            filter_list_type: FilterListType::STANDARD,
            working_directory: None,
            locale: "en".to_string(),
            default_filter_list_expires_period_sec: DEFAULT_EXPIRES_VALUE_FOR_FILTERS,
            compiler_conditional_constants: None,
            metadata_url: String::new(),
            metadata_locales_url: String::new(),
            request_timeout_ms: DEFAULT_REQUEST_TIMEOUT_MS,
            auto_lift_up_database: true,
        }
    }
}
