use adguard_flm::manager::filter_list_manager_impl::FilterListManagerImpl;
use adguard_flm::manager::FilterListManager;
use adguard_flm::Configuration;
use std::time::{Instant, SystemTime};

#[allow(dead_code)]
fn install_lists() {
    let start = SystemTime::now();

    let mut conf = Configuration::default();
    conf.app_name = "FlmApp".to_string();
    conf.version = "1.2.3".to_string();

    let manager = FilterListManagerImpl::new(conf).unwrap();

    let mut max_lists = 6;
    for n in 1..max_lists {
        manager
            .install_custom_filter_list(
                format!("https://filters.adtidy.org/mac_v2/filters/{}.txt", n),
                true,
                None,
                None,
            )
            .unwrap();
    }

    let list_with_includes = "https://raw.githubusercontent.com/AdguardTeam/AdguardFilters/1e480d06a5b5ebc8792856878b0c116569822a70/SpywareFilter/sections/cname_trackers.txt";
    manager
        .install_custom_filter_list(
            list_with_includes.to_string(),
            false,
            Some("Included list".to_string()),
            None,
        )
        .unwrap();
    max_lists += 1;

    let duration = SystemTime::now()
        .duration_since(start)
        .unwrap()
        .as_secs_f32();

    println!("Saving {} lists took {}", max_lists, duration);
}

#[allow(dead_code)]
fn gets_filter_list() {
    let mut conf = Configuration::default();
    conf.app_name = "FlmApp".to_string();
    conf.version = "1.2.3".to_string();

    let result = FilterListManagerImpl::new(conf)
        .unwrap()
        .get_full_filter_list_by_id(1)
        .unwrap();

    println!("{:?}", result)
}

#[allow(clippy::field_reassign_with_default)]
#[allow(dead_code)]
fn update_filters() {
    let start = Instant::now();
    let mut conf = Configuration::default();
    conf.metadata_url = "https://filters.adtidy.org/extension/safari/filters.json".to_string();
    conf.metadata_locales_url =
        "https://filters.adtidy.org/extension/safari/filters_i18n.json".to_string();
    conf.app_name = "FlmApp".to_string();
    conf.version = "1.2.3".to_string();

    let flm = FilterListManagerImpl::new(conf).unwrap();
    //  flm.pull_metadata().unwrap();
    let updated = flm.update_filters(true, 0, true).unwrap().unwrap();

    println!("Updated filters count: {}", updated.updated_list.len());
    println!("{}", "=".repeat(30));
    println!("Time elapsed: {:.2}", start.elapsed().as_secs_f32())
}

pub(crate) fn test() {
    println!("Ok");
}
