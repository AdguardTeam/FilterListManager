use crate::filters::parser::collectors::line_processor::{LineProcessor, ProcessedLine};
use crate::filters::parser::filter_compiler::CompiledFilterEntities;
use crate::filters::parser::parser_error::FilterParserErrorContext;
use crate::filters::parser::paths::try_to_resolve_include_path_from_parent_url;
use crate::storage::entities::filter::filter_include_entity::FilterIncludeEntity;
use crate::storage::entities::rules_list::rules_list_entity::RulesListEntity;
use crate::{string, Configuration, FilterParserError};

/// Collects the filter contents from in-memory entities.
pub(crate) struct DefaultFilterCollector<'c> {
    configuration: &'c Configuration,
}

impl<'c> DefaultFilterCollector<'c> {
    pub(crate) fn new(configuration: &'c Configuration) -> Self {
        Self { configuration }
    }

    /// Builds filter from [`CompiledFilterEntities`]
    pub(crate) fn collect(
        &self,
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
        &self,
        rule_entity: &RulesListEntity,
        root_filter_url: &str,
        filter_include_entities: Option<I>,
    ) -> Result<(String, i32), FilterParserErrorContext>
    where
        I: AsRef<Vec<FilterIncludeEntity>>,
    {
        let mut line_processor = LineProcessor::new(self.configuration);
        let mut filter_contents = String::with_capacity(rule_entity.text.len());

        let includes = match filter_include_entities {
            Some(ref includes) => includes.as_ref(),
            None => &Vec::new(),
        };

        for (index, line) in rule_entity.text.split_inclusive("\n").enumerate() {
            match line_processor.process(line).map_err(|why| {
                FilterParserErrorContext::new(why, index, string!(root_filter_url))
            })? {
                ProcessedLine::Skip => {}
                ProcessedLine::Include(include_path) => {
                    let include_contents =
                        Self::resolve_include(include_path, root_filter_url, includes).map_err(
                            |why| {
                                FilterParserErrorContext::new(why, index, string!(root_filter_url))
                            },
                        )?;
                    filter_contents.push_str(include_contents);
                }
                ProcessedLine::Rule => {
                    filter_contents.push_str(line);
                }
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

    /// Resolves an include path and returns the include body
    fn resolve_include<'body>(
        include_path: &str,
        parent_absolute_url: &str,
        filter_includes: &'body [FilterIncludeEntity],
    ) -> Result<&'body str, FilterParserError> {
        let resolved_path =
            try_to_resolve_include_path_from_parent_url(parent_absolute_url, include_path)?;

        let current_include = filter_includes
            .iter()
            .find(|include| include.absolute_url == resolved_path);

        if let Some(found_include) = current_include {
            Ok(found_include.body.as_str())
        } else {
            FilterParserError::Other(format!(
                "Couldn't find include {} for {}",
                include_path, parent_absolute_url
            ))
            .err()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DefaultFilterCollector;
    use crate::filters::parser::parser_error::FilterParserErrorContext;
    use crate::storage::entities::filter::filter_include_entity::FilterIncludeEntity;
    use crate::storage::entities::rules_list::rules_list_entity::RulesListEntity;
    use crate::Configuration;

    fn collect_filter(filter_text: &str) -> Result<(String, i32), FilterParserErrorContext> {
        let conf = Configuration::default();
        let collector = DefaultFilterCollector::new(&conf);
        let rules_entity = RulesListEntity::make(0, filter_text.to_string(), 0);

        collector.collect_from_parts(
            &rules_entity,
            "https://example.com/filter.txt",
            Option::<&Vec<FilterIncludeEntity>>::None,
        )
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

        let (filter_body, _) = collect_filter(test_filter).unwrap();

        assert!(filter_body.contains("catched"));
        assert!(!filter_body.contains("discarded"));
        assert!(filter_body.contains("true rule"));
        assert!(filter_body.contains("this works"));
    }
}
