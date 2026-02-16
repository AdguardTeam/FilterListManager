use crate::storage::repositories::filter_includes_repository::FilterIncludesRepository;
use crate::storage::repositories::rules_list_repository::RulesListRepository;
use crate::storage::with_transaction;
use crate::storage::DbConnectionManager;
use crate::utils::integrity;
use crate::{Configuration, FLMError, FLMResult};
use rusqlite::{Connection, Transaction};

/// Manager for filter integrity control operations
pub(crate) struct IntegrityControlManager;

impl IntegrityControlManager {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    /// Signs all filter rules and includes entities with the integrity key
    /// from configuration.
    ///
    /// # Failure
    ///
    /// Returns [`FLMError::InvalidConfiguration`] if `integrity_key` is not
    /// set in configuration.
    pub(crate) fn sign_all_filter_rules(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
    ) -> FLMResult<()> {
        let integrity_key =
            configuration
                .integrity_key
                .as_deref()
                .ok_or(FLMError::InvalidConfiguration(
                    "integrity_key is required for sign_all_filter_rules",
                ))?;

        let derived_key = integrity::derive_key(integrity_key);

        connection_manager.execute_db(|mut conn: Connection| {
            let rules_list_repository = RulesListRepository::new();
            let filter_includes_repository = FilterIncludesRepository::new();

            let mut rules_entities = rules_list_repository
                .select(&conn, None)
                .map_err(FLMError::from_database)?
                .unwrap_or_default();

            let mut includes_entities = filter_includes_repository
                .select(&conn, None)
                .map_err(FLMError::from_database)?;

            for entity in rules_entities.iter_mut() {
                integrity::sign_rules_list_entity(&derived_key, entity);
            }

            for entity in includes_entities.iter_mut() {
                integrity::sign_filter_include_entity(&derived_key, entity);
            }

            with_transaction(&mut conn, |tx: &Transaction| {
                rules_list_repository.update_integrity_signatures(tx, &rules_entities)?;
                filter_includes_repository.update_integrity_signatures(tx, &includes_entities)
            })
        })
    }

    /// Verifies integrity signatures of all filter rules and includes
    /// entities in the database.
    ///
    /// # Failure
    ///
    /// - Returns [`FLMError::InvalidConfiguration`] if `integrity_key` is not
    ///   set in configuration.
    /// - Returns [`FLMError::FilterIntegrityCheckFailed`] if any entity has a
    ///   missing or invalid signature.
    pub(crate) fn verify_integrity(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
    ) -> FLMResult<()> {
        let integrity_key =
            configuration
                .integrity_key
                .as_deref()
                .ok_or(FLMError::InvalidConfiguration(
                    "integrity_key is required for verify_integrity",
                ))?;

        let derived_key = integrity::derive_key(integrity_key);

        connection_manager.execute_db(|conn: Connection| {
            let rules_entities = RulesListRepository::new()
                .select(&conn, None)
                .map_err(FLMError::from_database)?
                .unwrap_or_default();

            for entity in &rules_entities {
                integrity::verify_rules_list_entity(&derived_key, entity)?;
            }

            let includes_entities = FilterIncludesRepository::new()
                .select(&conn, None)
                .map_err(FLMError::from_database)?;

            for entity in &includes_entities {
                integrity::verify_filter_include_entity(&derived_key, entity)?;
            }

            Ok(())
        })
    }
}
