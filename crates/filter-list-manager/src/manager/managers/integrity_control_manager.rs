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
    /// Uses streaming iteration to avoid loading all rule bodies into memory
    /// at once — only `(id, signature)` pairs are accumulated.
    ///
    /// # Failure
    ///
    /// Returns [`FLMError::InvalidConfiguration`] if `integrity_key` is not
    /// set in configuration.
    pub(crate) fn sign_all_rules(
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

            let rules_signatures = rules_list_repository
                .sign_and_collect_signatures_streaming(&conn, &derived_key)
                .map_err(FLMError::from_database)?;

            let includes_signatures = filter_includes_repository
                .sign_and_collect_signatures_streaming(&conn, &derived_key)
                .map_err(FLMError::from_database)?;

            with_transaction(&mut conn, |tx: &Transaction| {
                rules_list_repository.batch_update_signatures(tx, &rules_signatures)?;
                filter_includes_repository.batch_update_signatures(tx, &includes_signatures)
            })
        })
    }

    /// Verifies integrity signatures of all filter rules and includes
    /// entities in the database.
    ///
    /// Uses streaming iteration to verify one row at a time without loading
    /// all rule bodies into memory. Stops at the first failed entity.
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
            if let Some(filter_id) = RulesListRepository::new()
                .verify_all_streaming(&conn, &derived_key)
                .map_err(FLMError::from_database)?
            {
                return Err(FLMError::FilterIntegrityCheckFailed(filter_id));
            }

            if let Some(filter_id) = FilterIncludesRepository::new()
                .verify_all_streaming(&conn, &derived_key)
                .map_err(FLMError::from_database)?
            {
                return Err(FLMError::FilterIntegrityCheckFailed(filter_id));
            }

            Ok(())
        })
    }
}
