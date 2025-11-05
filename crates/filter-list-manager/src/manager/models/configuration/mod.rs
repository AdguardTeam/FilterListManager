//! Configuration-related objects for [`crate::FilterListManager`]
pub mod filter_list_type;
pub mod filters_compilation_policy;
pub mod locale;
pub mod request_proxy_mode;

pub use self::filter_list_type::FilterListType;
pub use self::filters_compilation_policy::FiltersCompilationPolicy;
pub use self::locale::Locale;
pub use self::request_proxy_mode::RequestProxyMode;

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
    /// There is one exception for local filters:
    /// If `should_ignore_expires_for_local_urls` is set to true, then expires will be ignored for local filters.
    pub default_filter_list_expires_period_sec: i32,
    /// Settings for filters compilation or collection from compiled parts.
    ///
    /// ### Compilation
    /// During the update, each filter will be "compiled" into main filter and its includes.
    /// Main filter remains unchanged. But in includes, (include, if/else/endif) directives will be resolved, using this policy.
    /// Recursive includes will be inlined.
    ///
    /// ### Collection
    /// When you get filters, they will be collected from compiled parts (main filter + includes).
    /// All directives in main filter will be resolved, using this policy, and includes will be injected.
    pub filters_compilation_policy: FiltersCompilationPolicy,
    /// URL of the index (filters.json) file
    pub metadata_url: String,
    /// URL of the locales (filters_i18n.json) file
    pub metadata_locales_url: String,
    /// Requests timeouts in milliseconds. Default value 60000
    pub request_timeout_ms: i32,
    /// Requests proxy mode
    pub request_proxy_mode: RequestProxyMode,
    /// Should ignore expires for local urls during update
    /// This may be useful for local filters update.
    /// Default value: false
    pub should_ignore_expires_for_local_urls: bool,
    /// “Uplifting” a database is a set of measures that brings the database up to date:
    /// * Database creation
    /// * Filling with schema
    /// * Creation of service tables and entities
    /// * Migrating between versions of a given library
    ///
    /// If you want to disable this option, you will need to manually call `flm.lift_up_database()`
    /// when you update the library in your application.
    /// Default value: true
    pub auto_lift_up_database: bool,
    /// Client app name
    pub app_name: String,
    /// Client app version
    pub version: String,
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
        locale.replace('-', LOCALES_DELIMITER)
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
            filters_compilation_policy: Default::default(),
            metadata_url: String::new(),
            request_proxy_mode: RequestProxyMode::UseSystemProxy,
            metadata_locales_url: String::new(),
            should_ignore_expires_for_local_urls: false,
            request_timeout_ms: DEFAULT_REQUEST_TIMEOUT_MS,
            auto_lift_up_database: true,
            app_name: String::new(),
            version: String::new(),
        }
    }
}
