use adguard_flm::{Configuration, FilterListManager, FilterListManagerImpl, FilterListType};
use std::path::PathBuf;
use std::time::Instant;

#[allow(clippy::field_reassign_with_default)]
/// Entry for `fill_database` command
pub fn entry(
    db_path: &PathBuf,
    index_url: &String,
    index_i18n_url: &String,
    filter_list_type: FilterListType,
) {
    let instant = Instant::now();

    let mut configuration = Configuration::default();
    configuration.filter_list_type = filter_list_type;
    configuration.metadata_url = index_url.to_owned();
    configuration.metadata_locales_url = index_i18n_url.to_owned();

    configuration.working_directory = Some(
        db_path
            .to_owned()
            .into_os_string()
            .to_str()
            .unwrap()
            .to_string(),
    );

    let flm = FilterListManagerImpl::new(configuration).unwrap();

    flm.pull_metadata().unwrap();
    flm.update_filters(true, 0, true).unwrap();

    println!("Completed in {:.2?} sec", instant.elapsed().as_secs_f32());
}
