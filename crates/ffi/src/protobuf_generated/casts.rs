use crate::outer_error::AGOuterError;
use crate::protobuf_generated::filter_list_manager;
use adguard_flm::{
    ActiveRulesInfo, Configuration, FilterGroup, FilterListMetadata, FilterListRules,
    FilterListType, FilterTag, FullFilterList, StoredFilterMetadata, UpdateFilterError,
    UpdateResult,
};

impl From<Configuration> for filter_list_manager::Configuration {
    fn from(value: Configuration) -> Self {
        Self {
            filter_list_type: match value.filter_list_type {
                FilterListType::STANDARD => filter_list_manager::FilterListType::Standard as i32,
                FilterListType::DNS => filter_list_manager::FilterListType::Dns as i32,
            },
            working_directory: value.working_directory.unwrap_or_default(),
            locale: value.locale,
            default_filter_list_expires_period_sec: value.default_filter_list_expires_period_sec,
            compiler_conditional_constants: value
                .compiler_conditional_constants
                .unwrap_or_default(),
            metadata_url: value.metadata_url,
            metadata_locales_url: value.metadata_locales_url,
            request_timeout_ms: value.request_timeout_ms,
            auto_lift_up_database: value.auto_lift_up_database,
        }
    }
}

impl Into<Configuration> for filter_list_manager::Configuration {
    fn into(self) -> Configuration {
        let working_directory = if self.working_directory.is_empty() {
            None
        } else {
            Some(self.working_directory)
        };

        let compiler_conditional_constants = if self.compiler_conditional_constants.is_empty() {
            None
        } else {
            Some(self.compiler_conditional_constants)
        };

        Configuration {
            filter_list_type: match self.filter_list_type {
                1 => FilterListType::DNS,
                0 => FilterListType::STANDARD,
                _ => unimplemented!(), // TODO: how it will fail in compile time?
            },
            working_directory,
            locale: self.locale,
            default_filter_list_expires_period_sec: self.default_filter_list_expires_period_sec,
            compiler_conditional_constants,
            metadata_url: self.metadata_url,
            metadata_locales_url: self.metadata_locales_url,
            encryption_key: None, // @TODO: MUST BE REMOVED
            request_timeout_ms: self.request_timeout_ms,
            auto_lift_up_database: self.auto_lift_up_database,
        }
    }
}

impl From<AGOuterError> for filter_list_manager::AgOuterError {
    fn from(value: AGOuterError) -> Self {
        let message = value.to_string();
        match value {
            AGOuterError::CannotOpenDatabase => Self {
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::CannotOpenDatabase(
                    filter_list_manager::CannotOpenDatabase {}
                )),
                message,
            },
            AGOuterError::NotADatabase => Self {
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::NotADatabase(
                    filter_list_manager::NotADatabase {},
                )),
                message,
            },
            AGOuterError::DiskFull => Self {
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::DiskFull(
                    filter_list_manager::DiskFull {},
                )),
                message,
            },
            AGOuterError::EntityNotFound(entity_id) => Self {
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::EntityNotFound(
                    filter_list_manager::EntityNotFound { entity_id },
                )),
                message,
            },
            AGOuterError::PathNotFound(path) => Self {
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::PathNotFound(
                    filter_list_manager::PathNotFound { path },
                )),
                message,
            },
            AGOuterError::PathHasDeniedPermission(path) => Self {
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::PathHasDeniedPermission(
                    filter_list_manager::PathHasDeniedPermission { path },
                )),
                message,
            },
            AGOuterError::PathAlreadyExists(path) => Self {
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::PathAlreadyExists(
                    filter_list_manager::PathAlreadyExists { path },
                )),
                message,
            },
            AGOuterError::TimedOut(message) => Self {
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::TimedOut(
                    filter_list_manager::TimedOut {},
                )),
                message,
            },
            AGOuterError::HttpClientNetworkError(_) => Self {
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::HttpClientNetworkError(
                    filter_list_manager::HttpClientNetworkError {},
                )),
                message,
            },
            AGOuterError::HttpStrict200Response(status_code, url) => Self {
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::HttpStrict200Response(
                    filter_list_manager::HttpStrict200Response {
                        status_code: status_code as u32,
                        url,
                    },
                )),
                message,
            },
            AGOuterError::HttpClientBodyRecoveryFailed(_) => Self {
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::HttpClientBodyRecoveryFailed(
                    filter_list_manager::HttpClientBodyRecoveryFailed {},
                )),
                message,
            },
            AGOuterError::FilterContentIsLikelyNotAFilter(message) => Self {
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::FilterContentIsLikelyNotAFilter(
                    filter_list_manager::FilterContentIsLikelyNotAFilter {},
                )),
                message,
            },
            AGOuterError::FilterParserError(message) => Self {
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::FilterParserError(
                    filter_list_manager::FilterParserError {},
                )),
                message,
            },
            AGOuterError::FieldIsEmpty(field_name) => Self {
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::FieldIsEmpty(
                    filter_list_manager::FieldIsEmpty {
                        field_name: field_name.to_string(),
                    },
                )),
                message,
            },
            AGOuterError::Mutex(_) => Self {
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::Mutex(filter_list_manager::Mutex {})),
                message,
            },
            AGOuterError::Other(_) => Self {
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::Other(filter_list_manager::Other {})),
                message,
            },
        }
    }
}

impl From<FullFilterList> for filter_list_manager::FullFilterList {
    fn from(value: FullFilterList) -> Self {
        Self {
            id: value.id,
            group_id: value.group_id,
            time_updated: value.time_updated,
            last_download_time: value.last_download_time,
            title: value.title,
            description: value.description,
            version: value.version,
            display_number: value.display_number,
            download_url: value.download_url,
            subscription_url: value.subscription_url,
            tags: value.tags.into_iter().map(Into::into).collect(),
            expires: value.expires,
            is_trusted: value.is_trusted,
            is_custom: value.is_custom,
            is_enabled: value.is_enabled,
            is_installed: value.is_installed,
            homepage: value.homepage,
            license: value.license,
            checksum: value.checksum,
            languages: value.languages,
            rules: value.rules.map(Into::into),
        }
    }
}

impl From<FilterTag> for filter_list_manager::FilterTag {
    fn from(value: FilterTag) -> Self {
        Self {
            id: value.id,
            keyword: value.keyword,
        }
    }
}

impl From<UpdateResult> for filter_list_manager::UpdateResult {
    fn from(value: UpdateResult) -> Self {
        Self {
            updated_list: value
                .updated_list
                .into_iter()
                .map(|filter| filter.into())
                .collect(),
            remaining_filters_count: value.remaining_filters_count,
            filters_errors: value
                .filters_errors
                .into_iter()
                .map(|error| error.into())
                .collect(),
        }
    }
}

impl From<UpdateFilterError> for filter_list_manager::UpdateFilterError {
    fn from(value: UpdateFilterError) -> Self {
        Self {
            filter_id: value.filter_id,
            message: value.message,
        }
    }
}

impl From<FilterListRules> for filter_list_manager::FilterListRules {
    fn from(value: FilterListRules) -> Self {
        Self {
            filter_id: value.filter_id,
            rules: value.rules,
            disabled_rules: value.disabled_rules,
        }
    }
}

impl Into<FilterListRules> for filter_list_manager::FilterListRules {
    fn into(self) -> FilterListRules {
        FilterListRules {
            filter_id: self.filter_id,
            rules: self.rules,
            disabled_rules: self.disabled_rules,
        }
    }
}

impl From<ActiveRulesInfo> for filter_list_manager::ActiveRulesInfo {
    fn from(value: ActiveRulesInfo) -> Self {
        Self {
            filter_id: value.filter_id,
            group_id: value.group_id,
            is_trusted: value.is_trusted,
            rules: value.rules,
        }
    }
}

impl From<FilterGroup> for filter_list_manager::FilterGroup {
    fn from(value: FilterGroup) -> Self {
        Self {
            id: value.id,
            name: value.name,
            display_number: value.display_number,
        }
    }
}

impl From<FilterListMetadata> for filter_list_manager::FilterListMetadata {
    fn from(value: FilterListMetadata) -> Self {
        Self {
            title: value.title,
            description: value.description,
            time_updated: value.time_updated,
            version: value.version,
            homepage: value.homepage,
            license: value.license,
            checksum: value.checksum,
            url: value.url,
            rules_count: value.rules_count,
        }
    }
}

impl From<StoredFilterMetadata> for filter_list_manager::StoredFilterMetadata {
    fn from(value: StoredFilterMetadata) -> Self {
        Self {
            id: value.id,
            group_id: value.group_id,
            time_updated: value.time_updated,
            last_download_time: value.last_download_time,
            title: value.title,
            description: value.description,
            version: value.version,
            display_number: value.display_number,
            download_url: value.download_url,
            subscription_url: value.subscription_url,
            tags: value.tags.into_iter().map(Into::into).collect(),
            expires: value.expires,
            is_trusted: value.is_trusted,
            is_custom: value.is_custom,
            is_enabled: value.is_enabled,
            is_installed: value.is_installed,
            homepage: value.homepage,
            license: value.license,
            checksum: value.checksum,
            languages: value.languages,
        }
    }
}
