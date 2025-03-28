use rusqlite::Connection;

use crate::manager::models::configuration::LOCALES_DELIMITER;
use crate::storage::repositories::localisation::filter_localisations_repository::FilterLocalisationRepository;
use crate::storage::DbConnectionManager;
use crate::Configuration;
use crate::FLMError;
use crate::FLMResult;
use crate::Locale;
use crate::RequestProxyMode;

/// This module updates configuration
pub(crate) struct ConfigurationUpdateManager;

impl ConfigurationUpdateManager {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    /// Changes locale
    pub(crate) fn change_locale(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &mut Configuration,
        suggested_locale: Locale,
    ) -> FLMResult<bool> {
        // Get saved locales
        let saved_locales: Vec<String> = connection_manager.execute_db(|conn: Connection| {
            FilterLocalisationRepository::new()
                .select_available_locales(&conn)
                .map_err(FLMError::from_database)
        })?;

        // Process suggested locale
        let normalized_locale = Configuration::normalize_locale_string(&suggested_locale);
        let mut fallback_locale: Option<&str> = None;

        if let Some(position) = normalized_locale.find(LOCALES_DELIMITER) {
            fallback_locale = Some(&normalized_locale[0..position])
        }

        let mut is_found_fallback_locale = false;
        for locale in saved_locales {
            if locale == normalized_locale {
                configuration.locale = locale;

                return Ok(true);
            }

            if let Some(value) = fallback_locale {
                if locale == value {
                    is_found_fallback_locale = true;
                }
            }
        }

        // We didn't find exact locale, but we may use fallback
        if is_found_fallback_locale {
            if let Some(value) = fallback_locale {
                configuration.locale = value.to_string();

                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Sets proxy mode
    pub(crate) fn set_proxy_mode(&self, configuration: &mut Configuration, mode: RequestProxyMode) {
        configuration.request_proxy_mode = mode;
    }
}
