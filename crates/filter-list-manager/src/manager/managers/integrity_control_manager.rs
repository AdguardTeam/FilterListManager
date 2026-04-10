use crate::storage::repositories::db_metadata_repository::DBMetadataRepository;
use crate::storage::repositories::filter_includes_repository::FilterIncludesRepository;
use crate::storage::repositories::filter_repository::FilterRepository;
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

    /// Signs all filter rules, includes, metadata, and the filter count using
    /// the integrity key from configuration.
    ///
    /// Uses streaming iteration to avoid loading all rule bodies into memory
    /// at once — only `(id, signature)` pairs are accumulated.
    ///
    /// # Failure
    ///
    /// Returns [`FLMError::InvalidConfiguration`] if `integrity_key` is not
    /// set in configuration.
    pub(crate) fn sign_all_data(
        &self,
        connection_manager: &DbConnectionManager,
        configuration: &Configuration,
    ) -> FLMResult<()> {
        let integrity_key =
            configuration
                .integrity_key
                .as_deref()
                .ok_or(FLMError::InvalidConfiguration(
                    "integrity_key is required for sign_all_data",
                ))?;

        let derived_key = integrity::derive_key(integrity_key);

        connection_manager.execute_db(|mut conn: Connection| {
            let rules_list_repository = RulesListRepository::new();
            let filter_includes_repository = FilterIncludesRepository::new();
            let filter_repository = FilterRepository::new();

            // 1. Collect signatures for rules and includes (streaming)
            let rules_signatures = rules_list_repository
                .sign_and_collect_signatures_streaming(&conn, &derived_key)
                .map_err(FLMError::from_database)?;

            let includes_signatures = filter_includes_repository
                .sign_and_collect_signatures_streaming(&conn, &derived_key)
                .map_err(FLMError::from_database)?;

            // 2. Collect signatures for filter metadata (streaming)
            let metadata_signatures = filter_repository
                .sign_and_collect_metadata_signatures_streaming(&conn, &derived_key)
                .map_err(FLMError::from_database)?;

            // 3. Count all filters
            let count = filter_repository
                .count_all(&conn)
                .map_err(FLMError::from_database)?;

            let count_signature = integrity::sign_filter_count(&derived_key, count);

            // 4. Write everything in a single transaction
            with_transaction(&mut conn, |tx: &Transaction| {
                rules_list_repository.batch_update_signatures(tx, &rules_signatures)?;
                filter_includes_repository.batch_update_signatures(tx, &includes_signatures)?;
                filter_repository.batch_update_metadata_signatures(tx, &metadata_signatures)?;

                // Update count signature in metadata table
                let mut meta = DBMetadataRepository::read(tx)?.unwrap_or_default();
                meta.filter_count_signature = Some(count_signature);
                DBMetadataRepository::save(tx, &meta)
            })
        })
    }

    /// Verifies integrity signatures of all filter rules, includes,
    /// metadata, and the filter count.
    ///
    /// Uses streaming iteration to verify one row at a time without loading
    /// all rule bodies into memory. Stops at the first failed entity.
    ///
    /// # Failure
    ///
    /// - Returns [`FLMError::InvalidConfiguration`] if `integrity_key` is not
    ///   set in configuration.
    /// - Returns [`FLMError::FilterIntegrityCheckFailed`] if any entity has a
    ///   missing or invalid signature (filter_id = 0 is used for count mismatch).
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
            let filter_repository = FilterRepository::new();

            // 1. Verify rules
            if let Some(filter_id) = RulesListRepository::new()
                .verify_all_streaming(&conn, &derived_key)
                .map_err(FLMError::from_database)?
            {
                return Err(FLMError::FilterIntegrityCheckFailed(filter_id));
            }

            // 2. Verify includes
            if let Some(filter_id) = FilterIncludesRepository::new()
                .verify_all_streaming(&conn, &derived_key)
                .map_err(FLMError::from_database)?
            {
                return Err(FLMError::FilterIntegrityCheckFailed(filter_id));
            }

            // 3. Verify filter metadata signatures
            if let Some(filter_id) = filter_repository
                .verify_all_metadata_streaming(&conn, &derived_key)
                .map_err(FLMError::from_database)?
            {
                return Err(FLMError::FilterIntegrityCheckFailed(filter_id));
            }

            // 4. Verify filter count
            let meta = DBMetadataRepository::read(&conn)
                .map_err(FLMError::from_database)?
                .unwrap_or_default();

            let current_count = filter_repository
                .count_all(&conn)
                .map_err(FLMError::from_database)?;

            match meta.filter_count_signature {
                Some(ref stored_sig) => {
                    if !integrity::verify_filter_count(&derived_key, current_count, stored_sig) {
                        // Use filter_id = 0 as a sentinel for count mismatch
                        return Err(FLMError::FilterIntegrityCheckFailed(0));
                    }
                }
                None => {
                    // Signature not yet set — treat as failure
                    return Err(FLMError::FilterIntegrityCheckFailed(0));
                }
            }

            Ok(())
        })
    }
}
