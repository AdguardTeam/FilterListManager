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
