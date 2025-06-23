use crate::filters::parser::conditional_directives_processor::ConditionalDirectivesProcessor;
use crate::filters::parser::filter_compiler::CompiledFilterEntities;
use crate::filters::parser::include_processor::get_include_path;
use crate::filters::parser::parser_error::FilterParserErrorContext;
use crate::filters::parser::paths::try_to_resolve_include_path_from_parent_url;
use crate::filters::parser::DIRECTIVE_INCLUDE;
use crate::storage::entities::filter::filter_include_entity::FilterIncludeEntity;
use crate::storage::entities::rules_list::rules_list_entity::RulesListEntity;
use crate::{string, Configuration, FilterParserError};

/// Collects the filter contents from filter-related entities
pub(crate) struct FilterCollector<'c> {
    conditional_directives_processor: ConditionalDirectivesProcessor<'c>,
}

impl<'c> FilterCollector<'c> {
    pub(crate) fn new(configuration: &'c Configuration) -> Self {
        Self {
            conditional_directives_processor: ConditionalDirectivesProcessor::new(configuration),
        }
    }

    /// Builds filter form [`CompiledFilterEntities`]
    pub(crate) fn collect(
        &mut self,
        entities: &CompiledFilterEntities,
        root_url: &str,
    ) -> Result<(String, i32), FilterParserErrorContext> {
        self.collect_from_parts(
            &entities.rules_list_entity,
            root_url,
            Some(&entities.filter_includes_entities),
        )
    }

    /// Builds filter just from filter-related entities
    pub(crate) fn collect_from_parts<I>(
        &mut self,
        rule_entity: &RulesListEntity,
        root_filter_url: &str,
        filter_include_entities: Option<I>,
    ) -> Result<(String, i32), FilterParserErrorContext>
    where
        I: AsRef<Vec<FilterIncludeEntity>>,
    {
        let mut filter_contents = String::with_capacity(rule_entity.text.len());

        let includes = match filter_include_entities {
            Some(ref includes) => includes.as_ref(),
            None => &vec![],
        };

        for (index, line) in rule_entity.text.split_inclusive("\n").enumerate() {
            let trimmed = line.trim();

            // First of all, check if the line is a conditional directive
            let is_conditional_directive = self
                .conditional_directives_processor
                .process(trimmed)
                .map_err(|why| {
                FilterParserErrorContext::new(why, index, string!(root_filter_url))
            })?;

            if is_conditional_directive {
                continue;
            }

            if self.conditional_directives_processor.is_capturing_lines() == false {
                continue;
            }

            // Try to include, if it is a valid include
            let include_contents_optional =
                Self::process_include(trimmed, root_filter_url, includes).map_err(|why| {
                    FilterParserErrorContext::new(why, index, string!(root_filter_url))
                })?;

            if let Some(include_contents) = include_contents_optional {
                filter_contents.push_str(&include_contents);
            } else {
                // Do not collect include directive line itself
                filter_contents.push_str(line);
            }
        }

        let count = includes
            .iter()
            .fold(rule_entity.rules_count, |mut acc, include| {
                acc += include.rules_count;
                acc
            });

        Ok((filter_contents, count))
    }

    /// Processes [`DIRECTIVE_INCLUDE`]
    fn process_include(
        line: &str,
        parent_absolute_url: &str,
        filter_includes: &[FilterIncludeEntity],
    ) -> Result<Option<String>, FilterParserError> {
        if line.starts_with(DIRECTIVE_INCLUDE) {
            let include_path_result = get_include_path(line)?;

            if let Some(include_path) = include_path_result {
                let resolved_path =
                    try_to_resolve_include_path_from_parent_url(parent_absolute_url, include_path)?;

                let current_include = filter_includes
                    .iter()
                    .find(|include| include.absolute_url == resolved_path);

                return if let Some(found_include) = current_include {
                    Ok(Some(found_include.body.clone()))
                } else {
                    FilterParserError::Other(format!(
                        "Couldn't find include {} for {}",
                        include_path, parent_absolute_url
                    ))
                    .err()
                };
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::FilterCollector;
    use crate::storage::entities::filter::filter_include_entity::FilterIncludeEntity;
    use crate::storage::entities::rules_list::rules_list_entity::RulesListEntity;
    use crate::string;
    use crate::{Configuration, FilterParserError};

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

        let conf = Configuration::default();
        let mut collector = FilterCollector::new(&conf);

        let rule = RulesListEntity::new(0, string!(test_filter), 0);
        let (contents, _) = collector
            .collect_from_parts::<Vec<FilterIncludeEntity>>(&rule, "", None)
            .unwrap();

        assert!(contents.contains("catched"));
        assert!(!contents.contains("discarded"));
        assert!(contents.contains("true rule"));
        assert!(contents.contains("this works"));
    }

    #[test]
    fn test_empty_if() {
        let test_filter = "!#if ";

        let conf = Configuration::default();
        let mut collector = FilterCollector::new(&conf);

        let rule = RulesListEntity::new(0, string!(test_filter), 0);
        let error = collector
            .collect_from_parts::<Vec<FilterIncludeEntity>>(&rule, "", None)
            .err()
            .unwrap();

        assert_eq!(error.error, FilterParserError::EmptyIf);
    }

    #[test]
    fn test_invalid_boolean_expression() {
        let test_filter = "!#if (&&";

        let conf = Configuration::default();
        let mut collector = FilterCollector::new(&conf);

        let rule = RulesListEntity::new(0, string!(test_filter), 0);
        let error = collector
            .collect_from_parts::<Vec<FilterIncludeEntity>>(&rule, "", None)
            .err()
            .unwrap();

        assert_eq!(error.error, FilterParserError::InvalidBooleanExpression);
    }
}
