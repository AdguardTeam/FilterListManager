use crate::manager::models::configuration::Configuration;
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
}
