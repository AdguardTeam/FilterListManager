use crate::manager::models::configuration::Configuration;
use crate::storage::entities::filter::filter_entity::FilterEntity;
use crate::storage::entities::filter::filter_include_entity::FilterIncludeEntity;
use crate::storage::entities::rules_list::rules_list_entity::RulesListEntity;
use crate::{FLMError, FLMResult, FilterId};
use blake3::{derive_key as derive_key_impl, Hash, Hasher};
use std::fmt::Write;

/// Domain separation context for blake3 key derivation.
/// Ensures that the same user-provided secret produces different derived keys
/// when used in different applications or for different purposes.
const KEY_DERIVATION_CONTEXT: &str = "adguard-flm integrity signature v1";

/// Generates a cryptographically secure random 256-bit key encoded as a 64-character hex string.
///
/// This key can be used as `Configuration::integrity_key` to enable integrity protection
/// for filter rules stored in the database.
///
/// # Example
///
/// ```rust,ignore
/// use adguard_flm::generate_random_key;
///
/// let key = generate_random_key()?;
/// config.integrity_key = Some(key);
/// ```
pub fn generate_random_key() -> FLMResult<String> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes)
        .map_err(|e| FLMError::Other(format!("Couldn't generate random key: {}", e)))?;

    Ok(bytes.iter().fold(String::with_capacity(64), |mut acc, b| {
        let _ = write!(&mut acc, "{:02x}", b);
        acc
    }))
}

/// Derives a 32-byte key from an arbitrary-length integrity key string
/// using blake3's key derivation function.
pub(crate) fn derive_key(integrity_key: &str) -> [u8; 32] {
    derive_key_impl(KEY_DERIVATION_CONTEXT, integrity_key.as_bytes())
}

/// Derives a key from configuration if integrity_key is set.
/// Returns `None` if integrity protection is disabled.
pub(crate) fn derive_key_if_needed(configuration: &Configuration) -> Option<[u8; 32]> {
    configuration.integrity_key.as_deref().map(derive_key)
}

/// Signs a [`RulesListEntity`] in-place using the derived key.
pub(crate) fn sign_rules_list_entity(derived_key: &[u8; 32], entity: &mut RulesListEntity) {
    entity.integrity_signature = Some(
        sign(derived_key, entity.filter_id, &entity.text)
            .to_hex()
            .to_string(),
    );
}

/// Signs a [`FilterIncludeEntity`] in-place using the derived key.
pub(crate) fn sign_filter_include_entity(derived_key: &[u8; 32], entity: &mut FilterIncludeEntity) {
    entity.integrity_signature = Some(
        sign(derived_key, entity.filter_id, &entity.body)
            .to_hex()
            .to_string(),
    );
}

/// Verifies a [`RulesListEntity`] integrity signature.
/// Returns [`FLMError::FilterIntegrityCheckFailed`] if signature is missing or invalid.
pub(crate) fn verify_rules_list_entity(
    derived_key: &[u8; 32],
    entity: &RulesListEntity,
) -> FLMResult<()> {
    if let Some(ref key) = entity.integrity_signature {
        if verify(derived_key, entity.filter_id, &entity.text, key) {
            return Ok(());
        }
    }

    Err(FLMError::FilterIntegrityCheckFailed(entity.filter_id))
}

/// Verifies a [`FilterIncludeEntity`] integrity signature.
/// Returns [`FLMError::FilterIntegrityCheckFailed`] if signature is missing or invalid.
pub(crate) fn verify_filter_include_entity(
    derived_key: &[u8; 32],
    entity: &FilterIncludeEntity,
) -> FLMResult<()> {
    if let Some(ref key) = entity.integrity_signature {
        if verify(derived_key, entity.filter_id, &entity.body, key) {
            return Ok(());
        }
    }

    Err(FLMError::FilterIntegrityCheckFailed(entity.filter_id))
}

/// Signs rules_list and filter_includes entities if integrity_key is set in configuration.
/// No-op if integrity_key is not set.
pub(crate) fn sign_entities_if_needed(
    configuration: &Configuration,
    rules_entity: &mut RulesListEntity,
    includes_entities: &mut [FilterIncludeEntity],
) {
    if let Some(derived) = derive_key_if_needed(configuration) {
        sign_rules_list_entity(&derived, rules_entity);
        for include in includes_entities.iter_mut() {
            sign_filter_include_entity(&derived, include);
        }
    }
}

// ---------------------------------------------------------------------------
// Filter metadata integrity
// ---------------------------------------------------------------------------

/// Computes an integrity signature over the 10 critical metadata fields of a
/// filter row.
///
/// The fields and their binary encoding (in order):
/// 1. `filter_id`         – 4 bytes little-endian i32
/// 2. `download_url`      – UTF-8 bytes, length-prefixed (8-byte LE u64)
/// 3. `subscription_url`  – UTF-8 bytes, length-prefixed (8-byte LE u64)
/// 4. `is_trusted`        – 1 byte (0x00 / 0x01)
/// 5. `is_enabled`        – 1 byte (0x00 / 0x01)
/// 6. `is_installed`      – 1 byte (0x00 / 0x01)
/// 7. `version`           – UTF-8 bytes, length-prefixed (8-byte LE u64)
/// 8. `last_update_time`  – 8 bytes little-endian i64
/// 9. `last_download_time`– 8 bytes little-endian i64
/// 10. `expires`          – 4 bytes little-endian i32
///
/// The encoding is intentionally stable and documented here so it can never
/// be changed accidentally. If a change is needed, bump the key derivation
/// context version (v1 → v2) and handle migration.
pub(crate) fn sign_filter_metadata(
    derived_key: &[u8; 32],
    filter_id: FilterId,
    download_url: &str,
    subscription_url: &str,
    is_trusted: bool,
    is_enabled: bool,
    is_installed: bool,
    version: &str,
    last_update_time: i64,
    last_download_time: i64,
    expires: i32,
) -> impl AsRef<str> + std::fmt::Display {
    let mut hasher = Hasher::new_keyed(derived_key);
    hasher.update(&filter_id.to_le_bytes());

    let dl = download_url.as_bytes();
    hasher.update(&(dl.len() as u64).to_le_bytes());
    hasher.update(dl);

    let sl = subscription_url.as_bytes();
    hasher.update(&(sl.len() as u64).to_le_bytes());
    hasher.update(sl);

    hasher.update(&[is_trusted as u8]);
    hasher.update(&[is_enabled as u8]);
    hasher.update(&[is_installed as u8]);

    let vl = version.as_bytes();
    hasher.update(&(vl.len() as u64).to_le_bytes());
    hasher.update(vl);

    hasher.update(&last_update_time.to_le_bytes());
    hasher.update(&last_download_time.to_le_bytes());
    hasher.update(&expires.to_le_bytes());

    hasher.finalize().to_hex()
}

/// Signs filter metadata fields from a [`FilterEntity`] and stores the
/// resulting signature in `entity.integrity_signature`.
///
/// No-op if `entity.filter_id` is `None` (filter not yet persisted).
pub(crate) fn sign_filter_entity(derived_key: &[u8; 32], entity: &mut FilterEntity) {
    let Some(filter_id) = entity.filter_id else {
        return;
    };

    let signature = sign_filter_metadata(
        derived_key,
        filter_id,
        &entity.download_url,
        &entity.subscription_url,
        entity.is_trusted,
        entity.is_enabled,
        entity.is_installed,
        &entity.version,
        entity.last_update_time,
        entity.last_download_time,
        entity.expires,
    );

    entity.set_integrity_signature(Some(signature.to_string()));
}

/// Signs filter entity metadata if `integrity_key` is set in configuration.
/// No-op if `integrity_key` is not set or `entity.filter_id` is `None`.
pub(crate) fn sign_filter_entity_if_needed(
    configuration: &Configuration,
    entity: &mut FilterEntity,
) {
    if let Some(derived) = derive_key_if_needed(configuration) {
        sign_filter_entity(&derived, entity);
    }
}

/// Verifies filter metadata integrity for a [`FilterEntity`].
/// Returns `true` if the signature is present and valid.
pub(crate) fn verify_filter_entity(derived_key: &[u8; 32], entity: &FilterEntity) -> bool {
    let Some(filter_id) = entity.filter_id else {
        return false;
    };

    let Some(sig) = entity.integrity_signature() else {
        return false;
    };

    let expected = sign_filter_metadata(
        derived_key,
        filter_id,
        &entity.download_url,
        &entity.subscription_url,
        entity.is_trusted,
        entity.is_enabled,
        entity.is_installed,
        &entity.version,
        entity.last_update_time,
        entity.last_download_time,
        entity.expires,
    );

    expected.as_ref() == sig
}

/// Verifies a batch of [`FilterEntity`] items with a pre-derived key.
pub(crate) fn verify_filter_entities(
    derived_key: &[u8; 32],
    entities: &[FilterEntity],
) -> FLMResult<()> {
    for entity in entities {
        if !verify_filter_entity(derived_key, entity) {
            return Err(FLMError::FilterIntegrityCheckFailed(
                entity.filter_id.unwrap_or(0),
            ));
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Filter count integrity
// ---------------------------------------------------------------------------

/// Signs the total number of filter records.
pub(crate) fn sign_filter_count(derived_key: &[u8; 32], count: i64) -> String {
    let mut hasher = Hasher::new_keyed(derived_key);
    // Domain-separate from per-filter signatures by using a fixed prefix.
    hasher.update(b"filter_count:");
    hasher.update(&count.to_le_bytes());
    hasher.finalize().to_hex().to_string()
}

/// Verifies the filter count signature.
/// Returns `true` if the signature matches.
pub(crate) fn verify_filter_count(derived_key: &[u8; 32], count: i64, signature: &str) -> bool {
    sign_filter_count(derived_key, count) == signature
}

/// Verifies a batch of [`RulesListEntity`] items with a pre-derived key.
pub(crate) fn verify_rules_list_entities(
    derived_key: &[u8; 32],
    entities: &[RulesListEntity],
) -> FLMResult<()> {
    for entity in entities {
        verify_rules_list_entity(derived_key, entity)?;
    }
    Ok(())
}

/// Verifies a batch of [`FilterIncludeEntity`] items with a pre-derived key.
pub(crate) fn verify_filter_include_entities(
    derived_key: &[u8; 32],
    entities: &[FilterIncludeEntity],
) -> FLMResult<()> {
    for entity in entities {
        verify_filter_include_entity(derived_key, entity)?;
    }
    Ok(())
}

/// Computes an integrity signature and returns it as a hex string.
/// Used by streaming repository methods that don't work with full entities.
pub(crate) fn sign_content(derived_key: &[u8; 32], filter_id: FilterId, content: &str) -> String {
    sign(derived_key, filter_id, content).to_hex().to_string()
}

/// Verifies an integrity signature for given content.
/// Used by streaming repository methods that don't work with full entities.
pub(crate) fn verify_content(
    derived_key: &[u8; 32],
    filter_id: FilterId,
    content: &str,
    signature: &str,
) -> bool {
    verify(derived_key, filter_id, content, signature)
}

/// Computes an integrity signature for given filter content.
/// Signs `filter_id` concatenated with `content` using blake3 keyed hash.
fn sign(derived_key: &[u8; 32], filter_id: FilterId, content: &str) -> Hash {
    let mut hasher = Hasher::new_keyed(derived_key);
    hasher.update(&filter_id.to_le_bytes());
    hasher.update(content.as_bytes());
    hasher.finalize()
}

/// Verifies an integrity signature against the expected value.
/// Returns `true` if the signature matches.
fn verify(derived_key: &[u8; 32], filter_id: FilterId, content: &str, signature: &str) -> bool {
    let computed = sign(derived_key, filter_id, content);
    // This does not allocate on heap
    computed.to_hex().as_str() == signature
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key_deterministic() {
        let key1 = derive_key("my-secret-key");
        let key2 = derive_key("my-secret-key");
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_derive_key_different_inputs() {
        let key1 = derive_key("key-a");
        let key2 = derive_key("key-b");
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_sign_and_verify() {
        let key = derive_key("test-key");
        let signature = sign(&key, 42, "some rules content");
        let sig_str = signature.to_hex().to_string();
        assert!(verify(&key, 42, "some rules content", &sig_str));
    }

    #[test]
    fn test_verify_fails_on_tampered_content() {
        let key = derive_key("test-key");
        let signature = sign(&key, 42, "original content");
        let sig_str = signature.to_hex().to_string();
        assert!(!verify(&key, 42, "tampered content", &sig_str));
    }

    #[test]
    fn test_verify_fails_on_wrong_filter_id() {
        let key = derive_key("test-key");
        let signature = sign(&key, 42, "content");
        let sig_str = signature.to_hex().to_string();
        assert!(!verify(&key, 99, "content", &sig_str));
    }

    #[test]
    fn test_verify_fails_on_wrong_key() {
        let key1 = derive_key("key-1");
        let key2 = derive_key("key-2");
        let signature = sign(&key1, 42, "content");
        let sig_str = signature.to_hex().to_string();
        assert!(!verify(&key2, 42, "content", &sig_str));
    }

    #[test]
    fn test_sign_empty_content() {
        let key = derive_key("test-key");
        let signature = sign(&key, 1, "");
        let sig_str = signature.to_hex().to_string();
        assert!(verify(&key, 1, "", &sig_str));
        assert_eq!(sig_str.len(), 64);
    }

    #[test]
    fn test_sign_filter_metadata_and_verify_entity() {
        let key = derive_key("metadata-key");
        let mut entity = FilterEntity::default();
        entity.filter_id = Some(42);
        entity.download_url = "https://example.com/filter.txt".to_string();
        entity.subscription_url = "https://example.com/subscription".to_string();
        entity.is_trusted = true;
        entity.is_enabled = true;
        entity.is_installed = true;
        entity.version = "1.2.3".to_string();
        entity.last_update_time = 1700000000;
        entity.last_download_time = 1700000100;
        entity.expires = 86400;

        sign_filter_entity(&key, &mut entity);
        assert!(verify_filter_entity(&key, &entity));

        // Tamper one protected field and verify should fail.
        entity.download_url = "https://evil.example/filter.txt".to_string();
        assert!(!verify_filter_entity(&key, &entity));
    }

    #[test]
    fn test_sign_filter_count_and_verify() {
        let key = derive_key("count-key");
        let sig = sign_filter_count(&key, 10);
        assert!(verify_filter_count(&key, 10, &sig));
        assert!(!verify_filter_count(&key, 11, &sig));
    }
}
