mod models;
pub mod outer_error;
mod result;
mod top_level;

use adguard_flm::{
    manager::{
        models::filter_group::FilterGroup, models::filter_tag::FilterTag, models::UpdateResult,
    },
    ActiveRulesInfo, Configuration, FLMResult, FilterId, FilterListManager as IFilterListManager,
    FilterListManagerImpl, FilterListMetadata, FullFilterList, StoredFilterMetadata,
};
pub use adguard_flm::{FilterListType, UpdateFilterError};
use std::ops::Deref;
use std::sync::{Mutex, MutexGuard};

pub use crate::models::FilterListManagerConstants;
use crate::outer_error::AGOuterError;
use crate::result::AGResult;
pub use crate::top_level::*;
use adguard_flm::manager::models::configuration::Locale;
use adguard_flm::manager::models::filter_list_rules::FilterListRules;
use adguard_flm::manager::models::filter_list_rules_raw::FilterListRulesRaw;

pub struct FilterListManager {
    flm: Mutex<FilterListManagerImpl>,
}

impl FilterListManager {
    pub fn new(configuration: Configuration) -> AGResult<Self> {
        let flm = FilterListManagerImpl::new(configuration).map_err(AGOuterError::from)?;
        Ok(Self {
            flm: Mutex::new(*flm),
        })
    }

    pub fn install_custom_filter_list(
        &self,
        download_url: String,
        is_trusted: bool,
        title: Option<String>,
        description: Option<String>,
    ) -> AGResult<FullFilterList> {
        self.wrap(move |flm| {
            flm.install_custom_filter_list(download_url, is_trusted, title, description)
        })
    }

    pub fn enable_filter_lists(&self, ids: Vec<FilterId>, is_enabled: bool) -> AGResult<i64> {
        self.wrap(move |flm| flm.enable_filter_lists(ids, is_enabled).map(|v| v as i64))
    }

    pub fn install_filter_lists(&self, ids: Vec<FilterId>, is_installed: bool) -> AGResult<i64> {
        self.wrap(move |flm| {
            flm.install_filter_lists(ids, is_installed)
                .map(|v| v as i64)
        })
    }

    pub fn delete_custom_filter_lists(&self, ids: Vec<FilterId>) -> AGResult<i64> {
        self.wrap(move |flm| flm.delete_custom_filter_lists(ids).map(|v| v as i64))
    }

    pub fn get_full_filter_lists(&self) -> AGResult<Vec<FullFilterList>> {
        self.wrap(|flm| flm.get_full_filter_lists())
    }

    pub fn get_full_filter_list_by_id(&self, id: FilterId) -> AGResult<Option<FullFilterList>> {
        self.wrap(move |flm| flm.get_full_filter_list_by_id(id))
    }

    pub fn get_stored_filters_metadata(&self) -> AGResult<Vec<StoredFilterMetadata>> {
        self.wrap(|flm| flm.get_stored_filters_metadata())
    }

    pub fn get_stored_filters_metadata_by_id(
        &self,
        filter_id: FilterId,
    ) -> AGResult<Option<StoredFilterMetadata>> {
        self.wrap(move |flm| flm.get_stored_filter_metadata_by_id(filter_id))
    }

    pub fn save_custom_filter_rules(&self, rules: FilterListRules) -> AGResult<()> {
        self.wrap(move |flm| flm.save_custom_filter_rules(rules))
    }

    pub fn save_disabled_rules(
        &self,
        filter_id: FilterId,
        disabled_rules: Vec<String>,
    ) -> AGResult<()> {
        self.wrap(move |flm| flm.save_disabled_rules(filter_id, disabled_rules))
    }

    pub fn update_filters(
        &self,
        ignore_filters_expiration: bool,
        loose_timeout: i32,
        ignore_filters_status: bool,
    ) -> AGResult<Option<UpdateResult>> {
        self.wrap(move |flm| {
            flm.update_filters(
                ignore_filters_expiration,
                loose_timeout,
                ignore_filters_status,
            )
        })
    }

    pub fn force_update_filters_by_ids(
        &self,
        ids: Vec<FilterId>,
        loose_timeout: i32,
    ) -> AGResult<Option<UpdateResult>> {
        self.wrap(move |flm| flm.force_update_filters_by_ids(ids, loose_timeout))
    }

    pub fn fetch_filter_list_metadata(&self, url: String) -> AGResult<FilterListMetadata> {
        // TODO: Do not need mutex here
        self.wrap(move |flm| flm.fetch_filter_list_metadata(url))
    }

    pub fn lift_up_database(&self) -> AGResult<()> {
        self.wrap(|flm| flm.lift_up_database())
    }

    pub fn get_all_tags(&self) -> AGResult<Vec<FilterTag>> {
        self.wrap(|flm| flm.get_all_tags())
    }

    pub fn get_all_groups(&self) -> AGResult<Vec<FilterGroup>> {
        self.wrap(|flm| flm.get_all_groups())
    }

    pub fn change_locale(&self, suggested_locale: Locale) -> AGResult<bool> {
        self.wrap(move |mut flm| flm.change_locale(suggested_locale))
    }

    pub fn pull_metadata(&self) -> AGResult<()> {
        self.wrap(|flm| flm.pull_metadata())
    }

    pub fn update_custom_filter_metadata(
        &self,
        filter_id: FilterId,
        title: String,
        is_trusted: bool,
    ) -> AGResult<bool> {
        self.wrap(move |flm| flm.update_custom_filter_metadata(filter_id, title, is_trusted))
    }

    pub fn get_database_path(&self) -> AGResult<String> {
        self.wrap(|flm| flm.get_database_path())
    }

    pub fn get_database_version(&self) -> AGResult<Option<i32>> {
        self.wrap(|flm| flm.get_database_version())
    }

    pub fn install_custom_filter_from_string(
        &self,
        download_url: String,
        last_download_time: i64,
        is_enabled: bool,
        is_trusted: bool,
        filter_body: String,
        custom_title: Option<String>,
        custom_description: Option<String>,
    ) -> AGResult<FullFilterList> {
        self.wrap(move |flm| {
            flm.install_custom_filter_from_string(
                download_url,
                last_download_time,
                is_enabled,
                is_trusted,
                filter_body,
                custom_title,
                custom_description,
            )
        })
    }

    pub fn get_active_rules(&self) -> AGResult<Vec<ActiveRulesInfo>> {
        self.wrap(|flm| flm.get_active_rules())
    }

    pub fn get_filter_rules_as_strings(
        &self,
        ids: Vec<FilterId>,
    ) -> AGResult<Vec<FilterListRulesRaw>> {
        self.wrap(move |flm| flm.get_filter_rules_as_strings(ids))
    }
}

impl FilterListManager {
    fn wrap<B, U>(&self, block: B) -> AGResult<U>
    where
        B: FnOnce(MutexGuard<FilterListManagerImpl>) -> FLMResult<U>,
    {
        let value = self
            .flm
            .lock()
            .map_err(|why| AGOuterError::Mutex(why.to_string()))?;

        block(value).map_err(AGOuterError::from)
    }
}

uniffi::include_scaffolding!("flm_ffi");
