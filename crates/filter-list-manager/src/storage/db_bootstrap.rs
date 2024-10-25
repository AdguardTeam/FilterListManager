use crate::storage::constants::{
    CUSTOM_FILTERS_GROUP_ID, SERVICE_GROUP_ID, USER_RULES_FILTER_LIST_ID,
};
use crate::storage::entities::db_metadata_entity::DBMetadataEntity;
use crate::storage::entities::filter_entity::FilterEntity;
use crate::storage::entities::filter_group_entity::FilterGroupEntity;
use crate::storage::entities::rules_list_entity::RulesListEntity;
use crate::storage::repositories::db_metadata_repository::DBMetadataRepository;
use crate::storage::repositories::filter_group_repository::FilterGroupRepository;
use crate::storage::repositories::filter_repository::FilterRepository;
use crate::storage::repositories::rules_list_repository::RulesListRepository;
use crate::storage::repositories::Repository;
use crate::FilterId;
use rusqlite::Transaction;

/// Returns the filter identifier added by default when the base is bootstrapped.
pub(crate) fn get_bootstrapped_filter_id() -> FilterId {
    USER_RULES_FILTER_LIST_ID
}

/// Make special "User rules" filter
fn make_user_rules_filter_entity() -> FilterEntity {
    let mut user_rules_entity = FilterEntity::default();

    user_rules_entity.filter_id = Some(USER_RULES_FILTER_LIST_ID);
    user_rules_entity.group_id = SERVICE_GROUP_ID;
    user_rules_entity.title = String::from("User rules");
    user_rules_entity.is_enabled = true;
    user_rules_entity.version = String::from("1.0.0.0");
    user_rules_entity.is_trusted = true;

    user_rules_entity
}

/// Creates an empty rule list for the user rules filter.
/// This should not be deleted, as user rules are not set in the usual way when downloaded from the `download_url`.
fn create_user_rules_rules_list_entity() -> RulesListEntity {
    RulesListEntity {
        filter_id: USER_RULES_FILTER_LIST_ID,
        text: String::new(),
        disabled_text: String::new(),
    }
}

/// Creates a special group for custom filters.
fn create_group_entity_for_custom_filters() -> FilterGroupEntity {
    FilterGroupEntity {
        group_id: CUSTOM_FILTERS_GROUP_ID,
        name: "Custom filters".to_string(),
        display_number: 0,
    }
}

/// Creates metadata info
fn create_db_metadata() -> DBMetadataEntity {
    DBMetadataEntity::default()
}

/// Makes specific queries to an empty database with a schema
pub(crate) fn db_bootstrap(transaction: &mut Transaction) -> rusqlite::Result<()> {
    let filter_entity = make_user_rules_filter_entity();
    let rule_entity = create_user_rules_rules_list_entity();
    let custom_group = create_group_entity_for_custom_filters();
    let metadata_entity = create_db_metadata();

    DBMetadataRepository::save(transaction, &metadata_entity)?;

    FilterRepository::new().insert(transaction, &[filter_entity])?;
    RulesListRepository::new().insert(transaction, &[rule_entity])?;
    FilterGroupRepository::new().insert(transaction, &[custom_group])
}
