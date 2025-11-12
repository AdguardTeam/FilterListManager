use adguard_flm::manager::filter_list_manager_impl::FilterListManagerImpl;
use adguard_flm::manager::FilterListManager;
use adguard_flm::Configuration;
use std::time::{Instant, SystemTime};

#[allow(dead_code)]
#[allow(clippy::field_reassign_with_default)]
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
                format!("https://raw.githubusercontent.com/AdguardTeam/FiltersRegistry/refs/heads/master/platforms/mac_v2/filters/{}.txt", n),
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
#[allow(clippy::field_reassign_with_default)]
fn get_active_filters() {
    let mut conf = Configuration::default();
    conf.app_name = "FlmApp".to_string();
    conf.version = "1.2.3".to_string();

    let start = Instant::now();
    let result = FilterListManagerImpl::new(conf)
        .unwrap()
        .get_active_rules_raw(vec![])
        .unwrap();
    let duration = start.elapsed().as_secs_f32();

    println!("Time elapsed: {:.6} micros", duration);

    println!("{:?}", result.len());
}

#[allow(clippy::field_reassign_with_default)]
#[allow(dead_code)]
fn update_filters() {
    let start = Instant::now();
    let mut conf = Configuration::default();
    conf.metadata_url = "https://raw.githubusercontent.com/AdguardTeam/FiltersRegistry/refs/heads/master/platforms/windows/filters.json".to_string();
    conf.metadata_locales_url =
        "https://raw.githubusercontent.com/AdguardTeam/FiltersRegistry/refs/heads/master/platforms/windows/filters_i18n.json".to_string();
    conf.app_name = "FlmApp".to_string();
    conf.version = "1.2.3".to_string();

    let flm = FilterListManagerImpl::new(conf).unwrap();
    // flm.pull_metadata().unwrap();
    let updated = flm.update_filters(false, 0, false).unwrap().unwrap();

    println!("Updated filters count: {}", updated.updated_list.len());
    updated.updated_list.iter().for_each(|f| {
        println!("Updated filter: {}", f.id);
    });

    println!("Errors count: {}", updated.filters_errors.len());
    updated.filters_errors.iter().for_each(|e| {
        println!("Error: {}", e.message);
    });
    println!("{}", "=".repeat(30));
    println!("Time elapsed: {:.2}", start.elapsed().as_secs_f32())
}

pub(crate) fn test() {
    update_filters();
}
