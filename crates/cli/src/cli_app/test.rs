use adguard_flm::manager::filter_list_manager_impl::FilterListManagerImpl;
use adguard_flm::manager::FilterListManager;
use adguard_flm::Configuration;
use std::time::SystemTime;

#[allow(dead_code)]
fn install_lists() {
    let start = SystemTime::now();
    let manager = FilterListManagerImpl::new(Configuration::default());

    let mut max_lists = 6;
    for n in 1..max_lists {
        manager
            .install_custom_filter_list(
                String::from(format!(
                    "https://filters.adtidy.org/mac_v2/filters/{}.txt",
                    n
                )),
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
    let result = FilterListManagerImpl::new(Configuration::default())
        .get_full_filter_list_by_id(1)
        .unwrap();

    println!("{:?}", result)
}

#[allow(dead_code)]
fn update_filters() {
    let updated = FilterListManagerImpl::new(Configuration::default())
        .update_filters(false, 0, false)
        .unwrap()
        .unwrap();

    println!("Updated filters count: {}", updated.updated_list.len());
    println!("{}", "=".repeat(30));

    updated.updated_list.iter().for_each(|f| {
        println!("Updated filter {}", f.id);
    });
}

pub(crate) fn test() {
    println!("Ok");
}
