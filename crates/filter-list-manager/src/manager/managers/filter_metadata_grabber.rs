use crate::filters::parser::metadata::KnownMetadataProperty;
use crate::filters::parser::FilterParser;
use crate::io::http::blocking_client::BlockingClient;
use crate::storage::entities::rules_list::rules_list_entity::RulesListEntity;
use crate::Configuration;
use crate::FLMError;
use crate::FLMResult;
use crate::FilterListMetadata;
use crate::FilterListMetadataWithBody;

/// This module grabs filters metadata by URL
pub(crate) struct FilterMetadataGrabber;

impl FilterMetadataGrabber {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    /// Fetches filter list metadata
    pub(crate) fn fetch_filter_list_metadata(
        &self,
        configuration: &Configuration,
        url: String,
    ) -> FLMResult<FilterListMetadata> {
        let client = BlockingClient::new(configuration)?;
        let mut parser = FilterParser::factory(configuration, &client);

        let download_url = parser
            .parse_from_url(&url)
            .map_err(FLMError::from_parser_error)?;

        let filter_list_metadata: FilterListMetadata = FilterListMetadata {
            title: parser.get_metadata(KnownMetadataProperty::Title),
            description: parser.get_metadata(KnownMetadataProperty::Description),
            time_updated: parser.get_metadata(KnownMetadataProperty::TimeUpdated),
            version: parser.get_metadata(KnownMetadataProperty::Version),
            homepage: parser.get_metadata(KnownMetadataProperty::Homepage),
            license: parser.get_metadata(KnownMetadataProperty::License),
            checksum: parser.get_metadata(KnownMetadataProperty::Checksum),
            url: download_url,
            rules_count: parser.get_rules_count(),
        };

        Ok(filter_list_metadata)
    }

    /// Fetches filter list metadata with body
    pub(crate) fn fetch_filter_list_metadata_with_body(
        &self,
        configuration: &Configuration,
        url: String,
    ) -> FLMResult<FilterListMetadataWithBody> {
        let client = BlockingClient::new(configuration)?;
        let mut parser = FilterParser::factory(configuration, &client);

        let download_url = parser
            .parse_from_url(&url)
            .map_err(FLMError::from_parser_error)?;

        let rule_entity: RulesListEntity = parser.extract_rule_entity(0);

        let filter_list_metadata_with_body: FilterListMetadataWithBody =
            FilterListMetadataWithBody {
                metadata: FilterListMetadata {
                    title: parser.get_metadata(KnownMetadataProperty::Title),
                    description: parser.get_metadata(KnownMetadataProperty::Description),
                    time_updated: parser.get_metadata(KnownMetadataProperty::TimeUpdated),
                    version: parser.get_metadata(KnownMetadataProperty::Version),
                    homepage: parser.get_metadata(KnownMetadataProperty::Homepage),
                    license: parser.get_metadata(KnownMetadataProperty::License),
                    checksum: parser.get_metadata(KnownMetadataProperty::Checksum),
                    url: download_url,
                    rules_count: rule_entity.rules_count,
                },
                filter_body: rule_entity.text,
            };

        Ok(filter_list_metadata_with_body)
    }
}
