use crate::filters::parser::filter_contents_provider::string_provider::StringProvider;
use crate::filters::parser::FilterParser;
use crate::io::http::blocking_client::BlockingClient;
use crate::storage::entities::rules_list_entity::RulesListEntity;
use crate::utils::memory::heap;
use crate::Configuration;

pub(crate) struct RulesListService;

impl RulesListService {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    pub(crate) fn update_rules_count(
        &self,
        conf: &Configuration,
        entity: RulesListEntity,
    ) -> RulesListEntity {
        let RulesListEntity {
            filter_id,
            text,
            disabled_text,
            rules_count: _,
        } = entity;

        let client = BlockingClient::new(&conf).unwrap();
        let provider = StringProvider::new(text, &client);
        let mut parser = FilterParser::with_custom_provider(heap(provider), &conf);

        parser.parse_from_url(&String::new()).unwrap();

        let RulesListEntity {
            filter_id: _,
            text,
            disabled_text: _,
            rules_count,
        } = parser.extract_rule_entity(0);

        RulesListEntity {
            filter_id,
            text,
            disabled_text,
            rules_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::constants::{USER_RULES_COUNT, USER_RULES_FILTER_LIST_ID};
    use crate::storage::entities::rules_list_entity::RulesListEntity;
    use crate::Configuration;

    use super::RulesListService;

    #[test]
    fn test_update_rules_count() {
        let filter_id = USER_RULES_FILTER_LIST_ID;
        let text = "Text\n!Text\n# Text\n\n\nText".to_string();
        let disabled_text = "Disabled Text".to_string();
        let rules_count = USER_RULES_COUNT;

        let user_rules_count_result = 2;

        let entity = RulesListEntity {
            filter_id,
            text: text.clone(),
            disabled_text: disabled_text.clone(),
            rules_count,
        };

        let mut conf = Configuration::default();
        conf.app_name = "FlmApp".to_string();
        conf.version = "1.2.3".to_string();

        let RulesListEntity {
            filter_id: new_filter_id,
            text: new_text,
            disabled_text: new_disabled_text,
            rules_count: new_rules_count,
        } = RulesListService::new().update_rules_count(&conf, entity);

        assert_eq!(new_filter_id, filter_id);
        assert_eq!(new_text, text);
        assert_eq!(new_disabled_text, disabled_text);
        assert_eq!(new_rules_count, user_rules_count_result);
    }
}
