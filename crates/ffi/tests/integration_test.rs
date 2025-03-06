use adguard_flm::Configuration;
use filter_list_manager_ffi::outer_error::AGOuterError;
use filter_list_manager_ffi::FilterListManager;

#[ignore]
#[test]
fn test_cannot_open_database() {
    let mut conf = Configuration::default();
    conf.app_name = "FlmApp".to_string();
    conf.version = "1.2.3".to_string();

    let flm = FilterListManager::new(conf).unwrap();
    let result = flm.get_all_tags().err().unwrap();

    assert_eq!(result, AGOuterError::CannotOpenDatabase);
}

#[ignore]
#[test]
fn test_errors_from_update_custom_metadata() {
    let mut conf = Configuration::default();
    conf.app_name = "FlmApp".to_string();
    conf.version = "1.2.3".to_string();
    let flm = FilterListManager::new(conf).unwrap();

    let title_err = flm
        .update_custom_filter_metadata(0, String::new(), true)
        .err()
        .unwrap();

    assert_eq!(title_err, AGOuterError::FieldIsEmpty("title"));

    let entity_err = flm
        .update_custom_filter_metadata(0, "title".to_string(), true)
        .err()
        .unwrap();

    assert_eq!(entity_err, AGOuterError::EntityNotFound(0));
}

#[test]
fn test_create_flm_without_app_name() {
    let flm_err = FilterListManager::new(Configuration::default())
        .err()
        .unwrap();

    assert_eq!(
        flm_err,
        AGOuterError::InvalidConfiguration("app_name is empty")
    );
}

#[test]
fn test_create_flm_without_version() {
    let mut conf = Configuration::default();
    conf.app_name = "FlmApp".to_string();

    let flm_err = FilterListManager::new(conf).err().unwrap();

    assert_eq!(
        flm_err,
        AGOuterError::InvalidConfiguration("version is empty")
    );
}
