use crate::filters::parser::filter_collector::FilterCollector;
use crate::filters::parser::filter_compiler::FilterCompiler;
use crate::filters::parser::metadata::KnownMetadataProperty;
use crate::io::http::blocking_client::BlockingClient;
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
        let mut compiler = FilterCompiler::factory(configuration, &client);

        let download_url = compiler
            .compile(&url)
            .map_err(FLMError::from_parser_error)?;

        let filter_list_metadata: FilterListMetadata = FilterListMetadata {
            title: compiler.get_metadata(KnownMetadataProperty::Title),
            description: compiler.get_metadata(KnownMetadataProperty::Description),
            time_updated: compiler.get_metadata(KnownMetadataProperty::TimeUpdated),
            version: compiler.get_metadata(KnownMetadataProperty::Version),
            homepage: compiler.get_metadata(KnownMetadataProperty::Homepage),
            license: compiler.get_metadata(KnownMetadataProperty::License),
            checksum: compiler.get_metadata(KnownMetadataProperty::Checksum),
            url: download_url,
            rules_count: compiler.get_rules_count(),
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
        let mut compiler = FilterCompiler::factory(configuration, &client);

        let download_url = compiler
            .compile(&url)
            .map_err(FLMError::from_parser_error)?;

        let metadata = compiler.clone_metadata();
        let compiled_filter_entities = compiler.into_entities(0);
        let mut builder = FilterCollector::new(configuration);

        let (filter_body, rules_count) = builder
            .collect(&compiled_filter_entities, &download_url)
            .map_err(FLMError::from_parser_error)?;

        let filter_list_metadata_with_body: FilterListMetadataWithBody =
            FilterListMetadataWithBody {
                metadata: FilterListMetadata {
                    title: metadata.get(KnownMetadataProperty::Title),
                    description: metadata.get(KnownMetadataProperty::Description),
                    time_updated: metadata.get(KnownMetadataProperty::TimeUpdated),
                    version: metadata.get(KnownMetadataProperty::Version),
                    homepage: metadata.get(KnownMetadataProperty::Homepage),
                    license: metadata.get(KnownMetadataProperty::License),
                    checksum: metadata.get(KnownMetadataProperty::Checksum),
                    url: download_url,
                    rules_count,
                },
                filter_body,
            };

        Ok(filter_list_metadata_with_body)
    }
}
