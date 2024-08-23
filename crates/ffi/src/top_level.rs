use crate::models::FilterListManagerConstants;
use adguard_flm::Configuration;

/// Default configuration factory
pub fn make_default_configuration() -> Configuration {
    Configuration::default()
}

/// Create constants structure
pub fn make_constants_structure() -> FilterListManagerConstants {
    FilterListManagerConstants::default()
}
