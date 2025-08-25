use crate::filters::parser::checksum_validator::validate_checksum;
use crate::filters::parser::conditional_directives_processor::ConditionalDirectivesProcessor;
use crate::filters::parser::filter_contents_provider::io_provider::IOProvider;
use crate::filters::parser::filter_contents_provider::FilterContentsProvider;
use crate::filters::parser::filter_cursor::FilterCursor;
use crate::filters::parser::include_processor::get_include_path;
use crate::filters::parser::is_rule_detector::is_line_is_rule;
use crate::filters::parser::metadata::collector::MetadataCollector;
use crate::filters::parser::metadata::KnownMetadataProperty;
use crate::filters::parser::parser_error::FilterParserErrorContext;
use crate::filters::parser::paths::try_to_resolve_include_path_from_parent_url;
use crate::filters::parser::rule_lines_collector::RuleLinesCollector;
use crate::filters::parser::DIRECTIVE_INCLUDE;
use crate::io::get_scheme;
use crate::io::http::blocking_client::BlockingClient;
use crate::storage::entities::filter::filter_include_entity::FilterIncludeEntity;
use crate::storage::entities::rules_list::rules_list_entity::RulesListEntity;
use crate::{string, Configuration, FilterId, FilterParserError};
use std::fmt::Display;

/// Information about includes collected during main filter text compilation
pub(crate) struct CollectedInclude {
    pub(crate) lines: RuleLinesCollector,
    pub(crate) absolute_url: String,
}

#[derive(Default)]
pub(crate) struct FilterParserResult {
    /// Оригинальный текст фильтра
    pub(crate) original_content: String,
    /// Original filter lines count
    pub(crate) original_lines_count: i32,
    /// Информация о найденных инклюдах
    pub(crate) includes: Vec<CollectedInclude>,
}

/// This is a result of filter compilation, containing main filter as [`RulesListEntity`] and all includes as a list of [`FilterIncludeEntity`]
pub(crate) struct CompiledFilterEntities {
    /// Main filter
    pub(crate) rules_list_entity: RulesListEntity,
    /// List of includes
    pub(crate) filter_includes_entities: Vec<FilterIncludeEntity>,
}

/// Result of file downloading
struct GetFileResult {
    contents: String,
    absolute_url: String,
}

/// Result of main filter file line processing
enum ProcessedMainFilterLine {
    /// This line is an include directive
    Include(CollectedInclude),
    /// This line isn't a directive, may be a rule
    MaybeRule,
    /// Should skip this line
    Skipped,
}

impl Into<FilterCursor> for GetFileResult {
    fn into(self) -> FilterCursor {
        FilterCursor::new(self.absolute_url, self.contents)
    }
}

/// Compiles filter
pub(crate) struct FilterCompiler<'a> {
    /// Conditional directives processor
    conditional_directives_processor: ConditionalDirectivesProcessor<'a>,
    /// Filter metadata collector
    metadata_collector: MetadataCollector,
    /// Filter contents provider
    filter_downloader: Box<dyn FilterContentsProvider + 'a>,
    /// This parser should skip checksum validation
    should_skip_checksum_validation: bool,
    /// This is a stack of filter cursors. Cursor is kind of an iterator over filter lines
    filters_cursor: Vec<FilterCursor>,
    /// Filter compilation result
    filter_parser_result: FilterParserResult,
    /// True if at least one directive was encountered in the filter
    directives_encountered: bool,
}

impl<'a> FilterCompiler<'a> {
    fn new(
        configuration: &'a Configuration,
        filter_downloader: Box<dyn FilterContentsProvider + 'a>,
    ) -> Self {
        Self {
            metadata_collector: MetadataCollector::new(),
            conditional_directives_processor: ConditionalDirectivesProcessor::new(configuration),
            filter_downloader,
            filters_cursor: vec![],
            should_skip_checksum_validation: true,
            filter_parser_result: FilterParserResult {
                original_content: string!(),
                original_lines_count: 0,
                includes: vec![],
            },
            directives_encountered: false,
        }
    }

    /// Basic factory
    pub(crate) fn factory(
        configuration: &'a Configuration,
        shared_http_client: &'a BlockingClient,
    ) -> Self {
        Self::new(configuration, Box::new(IOProvider::new(shared_http_client)))
    }

    /// Constructor for custom [`FilterContentsProvider`]
    pub(crate) fn with_custom_provider(
        filter_downloader: Box<dyn FilterContentsProvider + 'a>,
        configuration: &'a Configuration,
    ) -> Self {
        Self::new(configuration, filter_downloader)
    }

    /// This parser should skip checksum validation
    pub(crate) fn should_skip_checksum_validation(&mut self, value: bool) {
        self.should_skip_checksum_validation = value;
    }

    /// This is two-step compilation process:
    /// 1. Run through all lines and collect metadata, count rules, validate condition directives IN CAPTURING MODE and collect all includes
    /// 2. Process all includes, respecting condition directives
    pub(crate) fn compile(&mut self, url: &str) -> Result<String, FilterParserErrorContext> {
        // Get main filter
        let file_info = self
            .get_file(url, true)
            .or_else(|why| self.enrich_error_with_context(why, url))?;

        // TODO: COW say: Mo-Mo!
        // Save original content
        self.filter_parser_result.original_content = file_info.contents.clone();
        // Gets original root url
        let root_url = file_info.absolute_url.clone();

        // Push main filter on the parsing stack
        self.filters_cursor.push(file_info.into());

        loop {
            let stack_length = self.filters_cursor.len();

            // Get head from stack
            if let Some(current_cursor) = self.filters_cursor.last_mut() {
                let lineno = current_cursor.lineno;

                // Get next line from current stack frame
                if let Some(line) = current_cursor.next_line() {
                    let trimmed = line.trim();

                    if stack_length == 1 {
                        // Processing main filter
                        let result = self
                            .process_filter_line(trimmed, lineno)
                            .or_else(|why| self.enrich_error_with_context(why, url))?;

                        match result {
                            ProcessedMainFilterLine::Include(include) => {
                                self.filter_parser_result.includes.push(include);

                                // Stop parsing metadata at the first include directive
                                self.metadata_collector.mark_reached_eod();
                            }
                            ProcessedMainFilterLine::MaybeRule => {
                                self.filter_parser_result.original_lines_count +=
                                    i32::from(is_line_is_rule(trimmed));
                            }
                            ProcessedMainFilterLine::Skipped => {}
                        }
                    } else {
                        // Processing includes
                        let result = self
                            .process_included_line(trimmed, lineno)
                            .or_else(|why| self.enrich_error_with_context(why, url))?;

                        if result {
                            if let Some(collected_include) =
                                self.filter_parser_result.includes.last_mut()
                            {
                                collected_include.lines.increment_rules_count(&line);
                                collected_include.lines.push(line);
                            } else {
                                return self.enrich_error_with_context(
                                    FilterParserError::Other(string!(
                                        "Include could not be collected"
                                    )),
                                    url,
                                );
                            }
                        }
                    }
                } else {
                    self.filters_cursor.pop();
                }
            } else {
                return Ok(root_url);
            }
        }
    }

    /// Gets raw value by metadata property
    pub(crate) fn get_metadata(&self, property: KnownMetadataProperty) -> String {
        self.metadata_collector.get(property)
    }

    /// Get compiled entities
    pub(crate) fn into_entities(self, filter_id: FilterId) -> CompiledFilterEntities {
        let filter_includes_entities = self
            .filter_parser_result
            .includes
            .into_iter()
            .map(|include| {
                FilterIncludeEntity::make(
                    filter_id,
                    include.absolute_url,
                    include.lines.get_rules_count(),
                    include.lines.into_body(),
                )
            })
            .collect();

        let mut rules_list_entity = RulesListEntity::make(
            filter_id,
            self.filter_parser_result.original_content,
            self.filter_parser_result.original_lines_count,
        );

        rules_list_entity.set_has_directives(self.directives_encountered);

        CompiledFilterEntities {
            rules_list_entity,
            filter_includes_entities,
        }
    }

    /// Gets metadata collector clone
    pub(crate) fn clone_metadata(&self) -> MetadataCollector {
        self.metadata_collector.clone()
    }

    /// Gets rules count from entities
    pub(crate) fn get_rules_count(&self) -> i32 {
        self.filter_parser_result.includes.iter().fold(
            self.filter_parser_result.original_lines_count,
            |acc, include| acc + include.lines.get_rules_count(),
        )
    }
}

impl FilterCompiler<'_> {
    fn process_filter_line(
        &mut self,
        line: &str,
        lineno: usize,
    ) -> Result<ProcessedMainFilterLine, FilterParserError> {
        if let Some(include_url) = self.collect_include(line)? {
            self.directives_encountered = true;

            // Push first-level includes as CollectedInclude
            let file_info = self.get_file(include_url.as_str(), false)?;
            self.filters_cursor.push(file_info.into());

            return Ok(ProcessedMainFilterLine::Include(CollectedInclude {
                lines: RuleLinesCollector::new(),
                absolute_url: include_url,
            }));
        }

        if self.conditional_directives_processor.process(line)? {
            self.directives_encountered = true;
            return Ok(ProcessedMainFilterLine::Skipped);
        }

        // We do not need check lines is capturing here, cause all lines should be processed

        if !self.metadata_collector.is_reached_eod {
            self.metadata_collector.parse_line(line, lineno);
        }

        Ok(ProcessedMainFilterLine::MaybeRule)
    }

    #[allow(clippy::bool_comparison)]
    /// Parses line of the current included filter
    ///
    /// Returns `true` if line should be collected
    fn process_included_line(
        &mut self,
        line: &str,
        lineno: usize,
    ) -> Result<bool, FilterParserError> {
        if self.conditional_directives_processor.process(line)? == true {
            return Ok(false);
        }

        if self.conditional_directives_processor.is_capturing_lines() == false {
            return Ok(false);
        }

        if self.process_include(line)? == true {
            // Stop parsing metadata at the first include directive
            self.metadata_collector.mark_reached_eod();
            return Ok(false);
        }

        // Should work at runtime
        if self.metadata_collector.is_reached_eod == false {
            self.metadata_collector.parse_line(line, lineno);
        }

        Ok(true)
    }

    fn collect_include(&self, line: &str) -> Result<Option<String>, FilterParserError> {
        if line.starts_with(DIRECTIVE_INCLUDE) {
            let include_path_result = get_include_path(line)?;

            if let Some(include_path) = include_path_result {
                return if let Some(parent_cursor) = self.filters_cursor.last() {
                    let absolute_path = try_to_resolve_include_path_from_parent_url(
                        &parent_cursor.normalized_url,
                        include_path,
                    )?;

                    let cursor = self
                        .filters_cursor
                        .iter()
                        .find(|filter_cursor| absolute_path == filter_cursor.normalized_url);

                    if cursor.is_some() {
                        return FilterParserError::RecursiveInclusion.err();
                    }

                    Ok(Some(absolute_path))
                } else {
                    FilterParserError::StackIsCorrupted.err()
                };
            }
        }

        Ok(None)
    }

    /// Processes [`DIRECTIVE_INCLUDE`]
    ///
    /// # Returns
    ///
    /// `true` if directive was encountered
    fn process_include(&mut self, line: &str) -> Result<bool, FilterParserError> {
        if line.starts_with(DIRECTIVE_INCLUDE) {
            let include_path_result = get_include_path(line)?;

            if let Some(include_path) = include_path_result {
                return if let Some(parent_cursor) = self.filters_cursor.last() {
                    let absolute_path = try_to_resolve_include_path_from_parent_url(
                        &parent_cursor.normalized_url,
                        include_path,
                    )?;

                    let cursor = self
                        .filters_cursor
                        .iter()
                        .find(|filter_cursor| absolute_path == filter_cursor.normalized_url);

                    if cursor.is_some() {
                        return FilterParserError::RecursiveInclusion.err();
                    }

                    let file_info = self.get_file(absolute_path.as_str(), false)?;
                    self.filters_cursor.push(file_info.into());

                    Ok(true)
                } else {
                    FilterParserError::StackIsCorrupted.err()
                };
            }
        }

        Ok(false)
    }

    /// Downloads filter and returns its contents and calculated absolute url
    ///
    /// * `url`         - Absolute filter url
    /// * `from_root`   - Is it root filter or included. Used for stack integrity control
    ///
    /// Returns [`FilterParserError`] if stack is corrupted or downloading file was unsuccessful
    fn get_file(
        &self,
        mut absolute_url: &str,
        from_root: bool,
    ) -> Result<GetFileResult, FilterParserError> {
        absolute_url = absolute_url.trim_start();

        if from_root {
            if !self.filters_cursor.is_empty() {
                self.raise_root_filter_inclusion_forbidden(absolute_url)
            } else {
                let absolute_url = absolute_url.to_string();
                let contents = self.filter_downloader.get_filter_contents(&absolute_url)?;

                self.filter_downloader
                    .pre_check_filter_contents(contents.as_str())?;

                if !self.should_skip_checksum_validation {
                    validate_checksum(contents.as_str())?;
                }

                Ok(GetFileResult {
                    absolute_url,
                    contents,
                })
            }
        } else if self.filters_cursor.last().is_some() {
            let current_scheme = get_scheme(absolute_url);

            let contents = self
                .filter_downloader
                .get_included_filter_contents(absolute_url, current_scheme.into())?;

            self.filter_downloader
                .pre_check_filter_contents(contents.as_str())?;

            if !self.should_skip_checksum_validation {
                validate_checksum(contents.as_str())?;
            }

            Ok(GetFileResult {
                absolute_url: absolute_url.to_string(),
                contents,
            })
        } else {
            // Empty context here, because the stack is corrupted
            self.raise_stack_is_corrupted()
        }
    }
}

/// Errors
impl FilterCompiler<'_> {
    /// Enriches error with `filename:lineno`, if stack isn't empty
    fn enrich_error_with_context<R>(
        &self,
        error: FilterParserError,
        default_url: impl ToString,
    ) -> Result<R, FilterParserErrorContext> {
        if let Some(current) = self.filters_cursor.last() {
            return Err(FilterParserErrorContext::new(
                error,
                current.lineno,
                current.normalized_url.clone(),
            ));
        }

        Err(FilterParserErrorContext::new(
            error,
            0,
            default_url.to_string(),
        ))
    }

    /// Raises [`FilterParserError::StackIsCorrupted`] error with passed `lineno` and `absolute_url`
    #[cold]
    fn raise_stack_is_corrupted<T>(&self) -> Result<T, FilterParserError> {
        FilterParserError::StackIsCorrupted.err()
    }

    /// Raises [`FilterParserError::Other`] error while trying to include root filter into another filter.
    /// Technically, it's a stack corruption
    #[cold]
    fn raise_root_filter_inclusion_forbidden<T>(
        &self,
        absolute_url: impl Display,
    ) -> Result<T, FilterParserError> {
        let url = self
            .filters_cursor
            .last()
            .map(|v| v.normalized_url.to_string())
            .unwrap_or_default();

        Err(FilterParserError::Other(format!(
            "Trying to include root_filter \"{}\" into another filter \"{}\"",
            absolute_url, url
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::{CompiledFilterEntities, FilterCompiler};
    use crate::filters::parser::filter_contents_provider::string_provider::StringProvider;
    use crate::filters::parser::metadata::KnownMetadataProperty;
    use crate::utils::memory::heap;
    use crate::{Configuration, FilterParserError};

    impl<'c> FilterCompiler<'c> {
        pub(crate) fn test_factory_with_string(
            configuration: &'c Configuration,
            body: &str,
        ) -> Self {
            Self::with_custom_provider(
                Box::new(StringProvider::factory_test(body.to_string())),
                configuration,
            )
        }
    }

    #[test]
    fn test_conditional_filters_are_equal() {
        let test_filter = "
        !#if true
            true rule
            !#if true
                catched rule
            !#else
                discarded
            !#endif

            !this works
        !#endif
        ";

        let conf = Configuration::default();
        let mut compiler = FilterCompiler::test_factory_with_string(&conf, test_filter);

        compiler.compile("").unwrap();

        let CompiledFilterEntities {
            rules_list_entity, ..
        } = compiler.into_entities(0);

        assert_eq!(rules_list_entity.text, test_filter);
    }

    #[test]
    fn test_empty_if() {
        let test_filter = "!#if ";

        let conf = Configuration::default();
        let mut compiler = FilterCompiler::test_factory_with_string(&conf, test_filter);

        let error = compiler.compile("").err().unwrap();

        assert_eq!(error.error, FilterParserError::EmptyIf);
    }

    #[test]
    fn test_invalid_boolean_expression() {
        let test_filter = "!#if (&&";

        let conf = Configuration::default();
        let mut compiler = FilterCompiler::test_factory_with_string(&conf, test_filter);

        let error = compiler.compile("").err().unwrap();

        assert_eq!(error.error, FilterParserError::InvalidBooleanExpression);
    }

    #[test]
    fn test_rules_count() {
        let filter = include_str!("../../../tests/fixtures/small_pseudo_custom_filter.txt");
        let conf = Configuration::default();
        let mut compiler = FilterCompiler::test_factory_with_string(&conf, filter);
        let url = "we don't care".to_string();
        compiler.compile(&url).unwrap();

        let CompiledFilterEntities {
            rules_list_entity, ..
        } = compiler.into_entities(0);

        assert_eq!(rules_list_entity.rules_count, 6);
    }

    #[test]
    fn test_metadata_fields_must_be_grabbed_only_first_time() {
        let filter = include_str!("../../../tests/fixtures/small_pseudo_custom_filter.txt");
        let provider = StringProvider::factory_test(filter.into());
        let url = "we don't care".to_string();

        let conf = Configuration::default();
        let mut compiler = FilterCompiler::with_custom_provider(heap(provider), &conf);
        compiler.compile(&url).unwrap();

        let mut description_count = 0;
        let mut expires_count = 0;
        let mut version_count = 0;
        let mut time_updated_count = 0;
        filter.lines().for_each(|line| {
            if line.starts_with("! Description:") {
                description_count += 1;
            } else if line.starts_with("! Expires:") {
                expires_count += 1;
            } else if line.starts_with("! Version:") {
                version_count += 1;
            } else if line.starts_with("! TimeUpdated:") {
                time_updated_count += 1;
            }
        });

        // Testing fields are duplicated
        assert!(
            description_count == expires_count
                && expires_count == version_count
                && version_count == time_updated_count
                && time_updated_count == 2
        );

        // Check right values
        assert_eq!(
            compiler.get_metadata(KnownMetadataProperty::Description),
            "Pseudo Custom Filter Description"
        );
        assert_eq!(
            compiler.get_metadata(KnownMetadataProperty::TimeUpdated),
            "2024-05-28T13:31:01+00:00"
        );
        assert_eq!(
            compiler.get_metadata(KnownMetadataProperty::Version),
            "2.0.91.12"
        );
        assert_eq!(
            compiler.get_metadata(KnownMetadataProperty::Expires),
            "5 days (update frequency)",
        );
    }

    #[test]
    fn test_metadata_fields_aliases() {
        let filter =
            include_str!("../../../tests/fixtures/small_pseudo_custom_filter_with_aliases.txt");
        let provider = StringProvider::factory_test(filter.into());
        let url = "we don't care".to_string();

        let conf = Configuration::default();
        let mut compiler = FilterCompiler::with_custom_provider(heap(provider), &conf);
        compiler.compile(&url).unwrap();

        // Last modified
        assert_eq!(
            compiler.get_metadata(KnownMetadataProperty::TimeUpdated),
            "2024-06-24T12:01:21.959Z",
        );
    }
}
