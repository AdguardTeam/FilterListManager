use self::{
    boolean_expression_parser::BooleanExpressionParser,
    filter_cursor::FilterCursor,
    metadata::{collector::MetadataCollector, KnownMetadataProperty},
    parser_error::FilterParserErrorContext,
    rule_lines_collector::RuleLinesCollector,
};
use crate::filters::parser::checksum_validator::validate_checksum;
use crate::filters::parser::filter_contents_provider::io_provider::IOProvider;
use crate::filters::parser::filter_contents_provider::FilterContentsProvider;
use crate::filters::parser::parser_error::FilterParserError;
use crate::filters::parser::paths::resolve_absolute_uri;
use crate::io::http::blocking_client::BlockingClient;
use crate::io::url_schemes::UrlSchemes;
use crate::io::{get_authority, get_scheme};
use crate::manager::models::FilterId;
use crate::storage::entities::rules_list_entity::RulesListEntity;
use crate::Configuration;
use include_processor::get_include_path;
use nom::Slice;

mod boolean_expression_parser;
mod checksum_validator;
pub(crate) mod diff_updates;
pub(crate) mod filter_contents_provider;
mod filter_cursor;
mod include_processor;
pub(crate) mod metadata;
pub(crate) mod parser_error;
mod paths;
mod rcs_diff;
mod rule_lines_collector;

pub(super) const DIRECTIVE_IF: &str = "!#if";
pub(super) const DIRECTIVE_ELSE: &str = "!#else";
pub(super) const DIRECTIVE_ENDIF: &str = "!#endif";
pub(super) const DIRECTIVE_INCLUDE: &str = "!#include";

/// if/else/endif nesting level counter
type ConditionalNestingLevel = i16;

/// Downloads filter by given url, compiles using directives, and returns metadata and rules lists
/// This parser can recursively include another filters placed with [`DIRECTIVE_INCLUDE`] directive.
/// By default, filter checksum validation will be skipped.
pub(crate) struct FilterParser<'a> {
    conditional_nesting_level: ConditionalNestingLevel,
    condition_disabled_at_nesting: ConditionalNestingLevel,
    nesting_stack: Vec<ConditionalNestingLevel>,
    boolean_expression_parser: BooleanExpressionParser,
    metadata_collector: MetadataCollector,
    rule_lines_collector: RuleLinesCollector,
    filter_downloader: Box<dyn FilterContentsProvider + 'a>,
    filters_cursor: Vec<FilterCursor>,
    should_skip_checksum_validation: bool,
}

impl<'a> FilterParser<'a> {
    pub(crate) fn new(
        boolean_expression_parser: BooleanExpressionParser,
        filter_downloader: Box<dyn FilterContentsProvider + 'a>,
    ) -> Self {
        Self {
            conditional_nesting_level: 0,
            condition_disabled_at_nesting: 0,
            nesting_stack: vec![],
            boolean_expression_parser,
            metadata_collector: MetadataCollector::new(),
            rule_lines_collector: RuleLinesCollector::new(),
            filter_downloader,
            filters_cursor: vec![],
            should_skip_checksum_validation: true,
        }
    }

    /// Basic factory
    pub(crate) fn factory(
        configuration: &Configuration,
        shared_http_client: &'a BlockingClient,
    ) -> Self {
        Self::new(
            BooleanExpressionParser::new(configuration.compiler_conditional_constants.clone()),
            Box::new(IOProvider::new(shared_http_client)),
        )
    }

    /// Constructor for custom [`FilterContentsProvider`]
    pub(crate) fn with_custom_provider(
        filter_downloader: Box<dyn FilterContentsProvider + 'a>,
        configuration: &Configuration,
    ) -> Self {
        Self::new(
            BooleanExpressionParser::new(configuration.compiler_conditional_constants.clone()),
            filter_downloader,
        )
    }

    /// This parser should skip checksum validation
    pub(crate) fn should_skip_checksum_validation(&mut self, value: bool) {
        self.should_skip_checksum_validation = value;
    }

    /// Parses filter from root url
    ///
    /// * `url` - Absolute filter url. Now supports (https?, file) protocols
    ///
    /// Returns [`Result`] with absolute filter url
    pub(crate) fn parse_from_url(&mut self, url: &str) -> Result<String, FilterParserErrorContext> {
        self.push_file(url, true)?;

        let root_filter_url: String;
        if let Some(root_cursor) = self.filters_cursor.last() {
            root_filter_url = root_cursor.normalized_url.clone();
        } else {
            return self.build_error_with_context(FilterParserError::StackIsCorrupted);
        }

        loop {
            // Gets head of the stack
            if let Some(current_cursor) = self.filters_cursor.last_mut() {
                let current_line = current_cursor.lineno;
                if let Some(next_line) = current_cursor.next_line() {
                    self.parse_line(&next_line, current_line)?;
                } else {
                    self.filters_cursor.pop();
                }

                continue;
            }

            // Before successful return we must check if/endif balance.
            if self.conditional_nesting_level != 0 {
                return self.build_error_with_context(FilterParserError::UnbalancedIf);
            }

            return Ok(root_filter_url);
        }
    }

    /// Downloads filter and pushes it on the parsing stack.
    ///
    /// * `url`         - Absolute filter url
    /// * `from_root`   - Is it root filter or included. Used for stack integrity control
    ///
    /// Returns [`FilterParserErrorContext`] if stack is corrupted or downloading file was unsuccessful
    fn push_file(
        &mut self,
        mut absolute_url: &str,
        from_root: bool,
    ) -> Result<(), FilterParserErrorContext> {
        absolute_url = absolute_url.trim_start();
        let cursor: FilterCursor;

        if from_root {
            if !self.filters_cursor.is_empty() {
                return Err(FilterParserErrorContext {
                    file: absolute_url.to_string(),
                    line: 0,
                    error: FilterParserError::Other(format!(
                        "Trying to include root_filter \"{}\" into another filter \"{}\"",
                        absolute_url,
                        self.filters_cursor.last().unwrap().normalized_url
                    )),
                });
            } else {
                let absolute_url = absolute_url.to_string();
                match self.filter_downloader.get_filter_contents(&absolute_url) {
                    Ok(contents) => {
                        self.filter_downloader
                            .pre_check_filter_contents(contents.as_str())
                            .or_else(|why| self.build_error_with_context(why))?;

                        if !self.should_skip_checksum_validation {
                            validate_checksum(contents.as_str())
                                .or_else(|why| self.build_error_with_context(why))?;
                        }

                        cursor = FilterCursor::new(absolute_url, contents);
                    }
                    Err(why) => {
                        // Here we haven't cursor info, so we haven't context.
                        // Try to build error manually
                        return Err(FilterParserErrorContext {
                            file: absolute_url.to_string(),
                            line: 0,
                            error: why,
                        });
                    }
                }
            }
        } else if self.filters_cursor.last().is_some() {
            let current_scheme = get_scheme(absolute_url);

            let contents = self
                .filter_downloader
                .get_included_filter_contents(absolute_url, current_scheme.into())
                .or_else(|why| self.build_error_with_context(why))?;

            self.filter_downloader
                .pre_check_filter_contents(contents.as_str())
                .or_else(|why| self.build_error_with_context(why))?;

            if !self.should_skip_checksum_validation {
                validate_checksum(contents.as_str())
                    .or_else(|why| self.build_error_with_context(why))?;
            }

            cursor = FilterCursor::new(absolute_url.to_owned(), contents);
        } else {
            // Empty context here, because the stack is corrupted
            return Err(FilterParserErrorContext {
                line: 0,
                file: absolute_url.to_string(),
                error: FilterParserError::StackIsCorrupted,
            });
        }

        self.filters_cursor.push(cursor);

        Ok(())
    }

    #[allow(clippy::bool_comparison)]
    /// Parses line of the current file
    fn parse_line(&mut self, line: &str, lineno: usize) -> Result<(), FilterParserErrorContext> {
        let trimmed = line.trim();

        if self.process_conditional_directive(trimmed)? == true {
            return Ok(());
        }

        if self.is_capturing_lines() == false {
            return Ok(());
        }

        if self.process_include(trimmed)? == true {
            return Ok(());
        }

        if self.metadata_collector.is_reached_eod == false {
            self.metadata_collector.parse_line(trimmed, lineno);
        }

        self.rule_lines_collector.increment_rules_count(trimmed);

        self.rule_lines_collector.push(trimmed.to_string());

        Ok(())
    }

    /// Processes conditional directives [`DIRECTIVE_IF`], [`DIRECTIVE_ELSE`], ...
    fn process_conditional_directive(
        &mut self,
        line: &str,
    ) -> Result<bool, FilterParserErrorContext> {
        if line.starts_with(DIRECTIVE_IF) {
            let directive_expression = line.slice(DIRECTIVE_IF.len()..);
            if directive_expression.is_empty() {
                return self.build_error_with_context(FilterParserError::EmptyIf);
            }

            self.conditional_nesting_level += 1;

            if self.is_capturing_lines() {
                match self.boolean_expression_parser.eval(directive_expression) {
                    None => {
                        return self
                            .build_error_with_context(FilterParserError::InvalidBooleanExpression)
                    }

                    Some(false) => {
                        self.condition_disabled_at_nesting = self.conditional_nesting_level;
                    }

                    _ => {}
                }
            }

            return Ok(true);
        } else if line.starts_with(DIRECTIVE_ELSE) {
            // Has no nesting level, or we're trying to process else twice on the same level
            if self.conditional_nesting_level == 0
                || self
                    .nesting_stack
                    .last()
                    .map_or(false, |level| level == &self.conditional_nesting_level)
            {
                return self.build_error_with_context(FilterParserError::UnbalancedElse);
            }

            // We are inside "disabled block" if level not != 0 and equals conditional_nesting_level
            if self.condition_disabled_at_nesting == self.conditional_nesting_level {
                self.condition_disabled_at_nesting = 0;
            } else if self.condition_disabled_at_nesting == 0 {
                // We are inside "enabled block", if level == 0. Should disable if needed
                self.condition_disabled_at_nesting = self.conditional_nesting_level;
            }

            self.nesting_stack.push(self.conditional_nesting_level);

            return Ok(true);
        } else if line.starts_with(DIRECTIVE_ENDIF) {
            if self.condition_disabled_at_nesting == self.conditional_nesting_level {
                self.condition_disabled_at_nesting = 0;
            }

            // Throw this level out of stack
            if self
                .nesting_stack
                .last()
                .map_or(false, |level| level == &self.conditional_nesting_level)
            {
                self.nesting_stack.pop();
            }

            // Reduce the level
            self.conditional_nesting_level -= 1;

            if self.conditional_nesting_level < 0 {
                return self.build_error_with_context(FilterParserError::UnbalancedEndIf);
            }

            return Ok(true);
        }

        Ok(false)
    }

    /// Processes [`DIRECTIVE_INCLUDE`]
    fn process_include(&mut self, line: &str) -> Result<bool, FilterParserErrorContext> {
        if line.starts_with(DIRECTIVE_INCLUDE) {
            let include_path_result =
                get_include_path(line).or_else(|why| self.build_error_with_context(why))?;

            if let Some(include_path) = include_path_result {
                let absolute_path = self
                    .try_to_resolve_include_path(include_path)
                    .or_else(|why| self.build_error_with_context(why))?;

                let cursor = self
                    .filters_cursor
                    .iter()
                    .find(|filter_cursor| absolute_path == filter_cursor.normalized_url);

                if cursor.is_some() {
                    return self.build_error_with_context(FilterParserError::RecursiveInclusion);
                }

                // Stop parsing metadata at the first include directive
                self.metadata_collector.mark_reached_eod();

                return self.push_file(absolute_path.as_str(), false).map(|_| true);
            }
        }

        Ok(false)
    }
}

/// Internal objects getters
impl FilterParser<'_> {
    /// Gets raw value by metadata property
    pub fn get_metadata(&self, prop: KnownMetadataProperty) -> String {
        self.metadata_collector.get(prop)
    }

    pub fn extract_rule_entity(&mut self, id: FilterId) -> RulesListEntity {
        self.rule_lines_collector.extract_rule_entity(id)
    }

    pub fn get_rules_count(&self) -> i32 {
        self.rule_lines_collector.get_rules_count()
    }
}

/// Utility methods
impl FilterParser<'_> {
    /// Tries to resolve included path from parent [`FilterCursor`] and passed `include_path`
    ///
    /// # Failure
    ///
    /// Returns [`FilterParserError::StackIsCorrupted`] unless parent cursor exist
    /// Returns [`FilterParserError::SchemeIsIncorrect`] if parent url and included url have different schemes
    fn try_to_resolve_include_path(&self, include_path: &str) -> Result<String, FilterParserError> {
        if let Some(parent_cursor) = self.filters_cursor.last() {
            let parent_scheme = get_scheme(parent_cursor.normalized_url.as_str());

            match get_scheme(include_path) {
                // If scheme is found, this is an absolute_path
                Some(current_scheme_raw) => {
                    let current_scheme = UrlSchemes::from(current_scheme_raw);
                    let parent_scheme: UrlSchemes =
                        UrlSchemes::from(get_scheme(parent_cursor.normalized_url.as_str()));

                    // Can include only if the schemes match
                    if UrlSchemes::File == current_scheme && current_scheme != parent_scheme {
                        return Err(FilterParserError::SchemeIsIncorrect(String::from(
                            "\"file\" scheme can be included only from \"file\" scheme",
                        )));
                    }

                    // Authorities must match for web schemes
                    if parent_scheme.is_web_scheme() {
                        self.compare_same_origin(
                            parent_cursor.normalized_url.as_str(),
                            include_path,
                            parent_scheme,
                            current_scheme,
                        )?;
                    }

                    Ok(include_path.to_string())
                }
                // May be relative path
                None => {
                    // Special case - anonymous protocol
                    if include_path.starts_with("//") {
                        let parent_scheme = parent_scheme.unwrap_or_default();
                        // Special-special case - third slash
                        let extra_slash = if parent_scheme == "file"
                            && parent_cursor.normalized_url.starts_with("file:///")
                        {
                            "/"
                        } else {
                            ""
                        };

                        // Parent url always has right scheme
                        Ok(format!("{}:{}{}", parent_scheme, extra_slash, include_path))
                    } else {
                        resolve_absolute_uri(
                            parent_scheme.into(),
                            &parent_cursor.normalized_url,
                            include_path,
                        )
                    }
                }
            }
        } else {
            Err(FilterParserError::StackIsCorrupted)
        }
    }

    /// Parent and child must have the same origin.
    #[inline]
    fn compare_same_origin(
        &self,
        parent_url: &str,
        child_url: &str,
        parent_scheme: UrlSchemes,
        child_scheme: UrlSchemes,
    ) -> Result<(), FilterParserError> {
        if parent_scheme == child_scheme && get_authority(parent_url) == get_authority(child_url) {
            return Ok(());
        }

        FilterParserError::other_err_from_to_string(
            "Included filter must have the same origin with the root filter",
        )
    }

    /// Enriches error with `filename:lineno`, if stack isn't empty
    fn build_error_with_context<R>(
        &self,
        error: FilterParserError,
    ) -> Result<R, FilterParserErrorContext> {
        if let Some(current) = self.filters_cursor.last() {
            return Err(FilterParserErrorContext::new(
                error,
                current.lineno,
                current.normalized_url.clone(),
            ));
        }

        Err(FilterParserErrorContext::new(error, 0, String::new()))
    }

    /// Do we are capturing input lines right now?
    /// Usable for (if/else/endif)-like blocks
    fn is_capturing_lines(&self) -> bool {
        self.condition_disabled_at_nesting == 0
    }
}

#[cfg(test)]
mod tests {
    use super::{
        filter_contents_provider::string_provider::StringProvider, filter_cursor::FilterCursor,
        metadata::KnownMetadataProperty, parser_error::FilterParserError, FilterParser,
    };
    use crate::test_utils::SHARED_TEST_BLOCKING_HTTP_CLIENT;
    use crate::utils::memory::heap;
    use crate::Configuration;

    impl FilterParser<'_> {
        pub(crate) fn test_factory() -> Self {
            let conf = Configuration::default();

            Self::factory(&conf, &SHARED_TEST_BLOCKING_HTTP_CLIENT)
        }
    }

    #[test]
    fn test_process_conditional_directives() {
        let test_filter = "!#if true
            true rule
            !#if false
                adguard discarded rule
            !#else
                adguard catched rule
                next catched rule
            !#endif

        !this works
        !#endif
        ";
        let mut parser = FilterParser::test_factory();

        for (index, line) in test_filter.lines().enumerate() {
            let result = parser.process_conditional_directive(line.trim());

            match index {
                0 => {
                    // If true
                    assert_eq!(parser.conditional_nesting_level, 1);
                    assert!(result.unwrap());
                    assert!(parser.is_capturing_lines());
                }
                1 => {
                    // true rule
                    assert_eq!(parser.conditional_nesting_level, 1);
                    assert_eq!(result.unwrap(), false);
                    assert!(parser.is_capturing_lines());
                }
                2 => {
                    // if false
                    assert_eq!(parser.conditional_nesting_level, 2);
                    assert!(result.unwrap());
                    assert_eq!(parser.is_capturing_lines(), false);
                }
                3 => {
                    // adguard discarded rule
                    assert_eq!(parser.conditional_nesting_level, 2);
                    assert_eq!(result.unwrap(), false);
                    assert_eq!(parser.is_capturing_lines(), false);
                }
                4 => {
                    // !#else
                    assert_eq!(parser.conditional_nesting_level, 2);
                    assert!(result.unwrap());
                    assert_eq!(parser.is_capturing_lines(), true);
                }
                5 => {
                    // adguard catched rule
                    assert_eq!(parser.conditional_nesting_level, 2);
                    assert_eq!(result.unwrap(), false);
                    assert_eq!(parser.is_capturing_lines(), true);
                }
                6 => {
                    // next catched rule
                    assert_eq!(parser.conditional_nesting_level, 2);
                    assert_eq!(result.unwrap(), false);
                    assert_eq!(parser.is_capturing_lines(), true);
                }
                7 => {
                    // !#endif
                    assert_eq!(parser.conditional_nesting_level, 1);
                    assert!(result.unwrap());
                    assert_eq!(parser.is_capturing_lines(), true);
                }
                8 => {
                    // <empty line> with new_line
                    assert_eq!(parser.conditional_nesting_level, 1);
                    assert_eq!(result.unwrap(), false);
                    assert_eq!(parser.is_capturing_lines(), true);
                }
                9 => {
                    // !this works
                    assert_eq!(parser.conditional_nesting_level, 1);
                    assert_eq!(result.unwrap(), false);
                    assert_eq!(parser.is_capturing_lines(), true);
                }
                10 => {
                    // !#endif
                    assert_eq!(parser.conditional_nesting_level, 0);
                    assert!(result.unwrap());
                    assert_eq!(parser.is_capturing_lines(), true);
                }
                11 => {
                    // <empty line>
                    assert_eq!(parser.conditional_nesting_level, 0);
                    assert_eq!(result.unwrap(), false);
                    assert_eq!(parser.is_capturing_lines(), true);
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_catch_unbalanced_else() {
        let test_filter = "! comment
        !#if (false)
        abc
        !#else
        def
        !#else
ghj
        !#endif";

        let mut parser = FilterParser::test_factory();

        let mut error_encountered = false;
        for (idx, line) in test_filter.lines().enumerate() {
            if let Err(why) = parser.parse_line(line, idx) {
                error_encountered = true;
                assert_eq!(why.error, FilterParserError::UnbalancedElse);
                break;
            }
        }

        assert!(error_encountered);
    }

    #[test]
    fn test_else_without_if() {
        let test_filter = "! comment
        !#else
def
        !#endif";

        let mut parser = FilterParser::test_factory();

        let mut error_encountered = false;
        for (idx, line) in test_filter.lines().enumerate() {
            if let Err(why) = parser.parse_line(line, idx) {
                error_encountered = true;
                assert_eq!(why.error, FilterParserError::UnbalancedElse);
                break;
            }
        }

        assert!(error_encountered);
    }

    #[test]
    fn test_unbalanced_endif() {
        let test_filter = "! comment
!#if (false)
abc
    !#endif
!#endif";

        let mut parser = FilterParser::test_factory();

        let mut error_encountered = false;
        for (idx, line) in test_filter.lines().enumerate() {
            if let Err(why) = parser.parse_line(line, idx) {
                error_encountered = true;
                assert_eq!(why.error, FilterParserError::UnbalancedEndIf);
                break;
            }
        }

        assert!(error_encountered);
    }

    #[test]
    fn test_disabled_condition_inside_enabled() {
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

        let mut parser = FilterParser::test_factory();

        for (idx, line) in test_filter.lines().enumerate() {
            parser.parse_line(line, idx).unwrap();
        }

        let entity = parser.rule_lines_collector.extract_rule_entity(0);

        assert!(entity.text.contains("catched"));
        assert!(!entity.text.contains("discarded"));
        assert!(entity.text.contains("true rule"));
        assert!(entity.text.contains("this works"));
    }

    #[test]
    fn test_empty_if() {
        let test_filter = "!#if ";

        let mut parser = FilterParser::test_factory();

        let result = parser.parse_line(test_filter, 0).err().unwrap();

        assert_eq!(result.error, FilterParserError::EmptyIf)
    }

    #[test]
    fn test_invalid_boolean_expression() {
        let test_filter = "!#if (&&";

        let mut parser = FilterParser::test_factory();

        let result = parser.parse_line(test_filter, 0).err().unwrap();
        assert_eq!(result.error, FilterParserError::InvalidBooleanExpression);
    }

    #[test]
    fn test_rules_count() {
        let filter = include_str!("../../tests/fixtures/small_pseudo_custom_filter.txt");
        let provider = StringProvider::factory_test(filter.into());
        let url = "we don't care".to_string();

        let conf = Configuration::default();
        let mut parser = FilterParser::with_custom_provider(heap(provider), &conf);
        parser.parse_from_url(&url).unwrap();

        assert_eq!(parser.rule_lines_collector.get_rules_count(), 6);
    }

    #[test]
    fn test_metadata_fields_must_be_grabbed_only_first_time() {
        let filter = include_str!("../../tests/fixtures/small_pseudo_custom_filter.txt");
        let provider = StringProvider::factory_test(filter.into());
        let url = "we don't care".to_string();

        let conf = Configuration::default();
        let mut parser = FilterParser::with_custom_provider(heap(provider), &conf);
        parser.parse_from_url(&url).unwrap();

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
            parser.get_metadata(KnownMetadataProperty::Description),
            "Pseudo Custom Filter Description"
        );
        assert_eq!(
            parser.get_metadata(KnownMetadataProperty::TimeUpdated),
            "2024-05-28T13:31:01+00:00"
        );
        assert_eq!(
            parser.get_metadata(KnownMetadataProperty::Version),
            "2.0.91.12"
        );
        assert_eq!(
            parser.get_metadata(KnownMetadataProperty::Expires),
            "5 days (update frequency)",
        );
    }

    #[test]
    fn test_metadata_fields_aliases() {
        let filter =
            include_str!("../../tests/fixtures/small_pseudo_custom_filter_with_aliases.txt");
        let provider = StringProvider::factory_test(filter.into());
        let url = "we don't care".to_string();

        let conf = Configuration::default();
        let mut parser = FilterParser::with_custom_provider(heap(provider), &conf);
        parser.parse_from_url(&url).unwrap();

        // Last modified
        assert_eq!(
            parser.get_metadata(KnownMetadataProperty::TimeUpdated),
            "2024-06-24T12:01:21.959Z",
        );
    }

    #[test]
    fn test_include_path_resolving() {
        let conf = Configuration::default();

        [
            (
                "https://example.com/filters/safari/1.txt",
                "https://example.com/filter1.txt",
                Ok("https://example.com/filter1.txt".to_string()),
            ),
            (
                "https://example.com/filters/safari/1.txt",
                "ffwf",
                Ok("https://example.com/filters/safari/ffwf".to_string()),
            ),
            (
                "https://example.com/filters/safari/1.txt",
                "../../global_filter.txt",
                Ok("https://example.com/global_filter.txt".to_string()),
            ),
            (
                "https://example.com/filters/safari/1.txt",
                "../a/./b/c",
                Ok("https://example.com/filters/a/b/c".to_string()),
            ),
            (
                "https://example.com/filters/safari/1.txt",
                "//example.com/filter.txt",
                Ok("https://example.com/filter.txt".to_string()),
            ),
            (
                "file:///C:/filters/safari/1.txt",
                "//C:/same.scheme/filter.txt",
                Ok("file:///C:/same.scheme/filter.txt".to_string()),
            ),
            (
                "https://example.com/filters.txt",
                "file://Volumes/osx/users/user/filters.txt",
                Err(FilterParserError::SchemeIsIncorrect(
                    "\"file\" scheme can be included only from \"file\" scheme".to_string(),
                )),
            ),
            (
                "https://example.com/filters/safari/1.txt",
                "https://adguard.com/filter1.txt",
                FilterParserError::other_err_from_to_string(
                    "Included filter must have the same origin with the root filter",
                ),
            ),
        ]
        .into_iter()
        .for_each(|(base_url, url_like_string, expected_result)| {
            let mut parser = FilterParser::with_custom_provider(
                heap(StringProvider::factory_test(String::new())),
                &conf,
            );

            parser
                .filters_cursor
                .push(FilterCursor::new(String::from(base_url), String::new()));

            let method_result = parser.try_to_resolve_include_path(url_like_string);
            assert_eq!(method_result, expected_result);
        })
    }
}
