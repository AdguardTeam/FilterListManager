use adguard_flm::Configuration;
use filter_list_manager_ffi::outer_error::AGOuterError;
use filter_list_manager_ffi::FilterListManager;

#[ignore]
#[test]
fn test_cannot_open_database() {
    let flm = FilterListManager::new(Configuration::default());
    let result = flm.get_all_tags().err().unwrap();

    assert_eq!(result, AGOuterError::CannotOpenDatabase);
}

#[ignore]
#[test]
fn test_errors_from_update_custom_metadata() {
    let flm = FilterListManager::new(Configuration::default());

    let title_err = flm
        .update_custom_filter_metadata(0, String::new(), true)
        .err()
        .unwrap();

    assert_eq!(title_err, AGOuterError::FieldIsEmpty("title"));

    let entity_err = flm
        .update_custom_filter_metadata(0, String::new(), true)
        .err()
        .unwrap();

    assert_eq!(entity_err, AGOuterError::EntityNotFound(0));
}
