//! Special module for conversion between flm objects and their protobuf-counterparts

use crate::outer_error::AGOuterError;
use crate::protobuf_generated::filter_list_manager;
use adguard_flm::{
    ActiveRulesInfo, Configuration, DisabledRulesRaw, FilterGroup, FilterListMetadata,
    FilterListMetadataWithBody, FilterListRules, FilterListRulesRaw, FilterListType, FilterTag,
    FullFilterList, RequestProxyMode, RulesCountByFilter, StoredFilterMetadata, UpdateFilterError,
    UpdateResult,
};

impl From<Vec<String>> for filter_list_manager::CompilerConditionalConstants {
    fn from(value: Vec<String>) -> Self {
        Self {
            compiler_conditional_constants: value,
        }
    }
}

impl From<filter_list_manager::CompilerConditionalConstants> for Vec<String> {
    fn from(value: filter_list_manager::CompilerConditionalConstants) -> Self {
        value.compiler_conditional_constants
    }
}

impl From<Configuration> for filter_list_manager::Configuration {
    fn from(value: Configuration) -> Self {
        let mut proxy_addr = String::new();

        let proxy_mode = match value.request_proxy_mode {
            RequestProxyMode::UseSystemProxy => 0,
            RequestProxyMode::NoProxy => 1,
            RequestProxyMode::UseCustomProxy { addr } => {
                proxy_addr = addr;

                2
            }
        };

        let compiler_conditional_constants = value.compiler_conditional_constants.map(Into::into);

        Self {
            filter_list_type: match value.filter_list_type {
                FilterListType::STANDARD => filter_list_manager::FilterListType::Standard as i32,
                FilterListType::DNS => filter_list_manager::FilterListType::Dns as i32,
            },
            working_directory: value.working_directory,
            locale: value.locale,
            default_filter_list_expires_period_sec: value.default_filter_list_expires_period_sec,
            compiler_conditional_constants,
            metadata_url: value.metadata_url,
            metadata_locales_url: value.metadata_locales_url,
            request_timeout_ms: value.request_timeout_ms,
            auto_lift_up_database: value.auto_lift_up_database,
            request_proxy_mode: proxy_mode,
            request_custom_proxy_addr: proxy_addr,
            app_name: value.app_name,
            version: value.version,
        }
    }
}

impl From<filter_list_manager::Configuration> for Configuration {
    fn from(val: filter_list_manager::Configuration) -> Self {
        let compiler_conditional_constants = val.compiler_conditional_constants.map(Into::into);

        Configuration {
            filter_list_type: match val.filter_list_type {
                1 => FilterListType::DNS,
                0 => FilterListType::STANDARD,
                _ => unimplemented!(), // TODO: how it will fail in compile time?
            },
            working_directory: val.working_directory,
            locale: val.locale,
            default_filter_list_expires_period_sec: val.default_filter_list_expires_period_sec,
            compiler_conditional_constants,
            metadata_url: val.metadata_url,
            metadata_locales_url: val.metadata_locales_url,
            request_timeout_ms: val.request_timeout_ms,
            request_proxy_mode: match val.request_proxy_mode {
                1 => RequestProxyMode::NoProxy,
                2 => RequestProxyMode::UseCustomProxy {
                    addr: val.request_custom_proxy_addr,
                },
                _ => RequestProxyMode::UseSystemProxy,
            },
            auto_lift_up_database: val.auto_lift_up_database,
            app_name: val.app_name,
            version: val.version,
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

            AGOuterError::DatabaseBusy => Self {
                message,
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::DatabaseBusy(
                    filter_list_manager::DatabaseBusy {}
                )),
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
            AGOuterError::InvalidConfiguration(msg) => Self {
                error: Some(crate::protobuf_generated::filter_list_manager::ag_outer_error::Error::InvalidConfiguration(
                    filter_list_manager::InvalidConfiguration {
                        msg: msg.to_string(),
                    },
                )),
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
            rules_count: value.rules_count,
        }
    }
}

impl From<filter_list_manager::FilterListRules> for FilterListRules {
    fn from(val: filter_list_manager::FilterListRules) -> Self {
        FilterListRules {
            filter_id: val.filter_id,
            rules: val.rules,
            disabled_rules: val.disabled_rules,
            rules_count: val.rules_count,
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

impl From<FilterListMetadataWithBody> for filter_list_manager::FilterListMetadataWithBody {
    fn from(value: FilterListMetadataWithBody) -> Self {
        Self {
            metadata: Some(filter_list_manager::FilterListMetadata::from(
                value.metadata,
            )),
            filter_body: value.filter_body,
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

impl From<FilterListRulesRaw> for filter_list_manager::FilterListRulesRaw {
    fn from(value: FilterListRulesRaw) -> Self {
        Self {
            filter_id: value.filter_id,
            rules: value.rules,
            disabled_rules: value.disabled_rules,
            rules_count: value.rules_count,
        }
    }
}

impl From<DisabledRulesRaw> for filter_list_manager::DisabledRulesRaw {
    fn from(value: DisabledRulesRaw) -> Self {
        Self {
            filter_id: value.filter_id,
            text: value.text,
        }
    }
}

impl From<RulesCountByFilter> for filter_list_manager::RulesCountByFilter {
    fn from(value: RulesCountByFilter) -> Self {
        Self {
            filter_id: value.filter_id,
            rules_count: value.rules_count,
        }
    }
}
