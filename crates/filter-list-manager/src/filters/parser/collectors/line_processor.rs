use crate::filters::parser::conditional_directives_processor::ConditionalDirectivesProcessor;
use crate::filters::parser::include_processor::get_include_path;
use crate::filters::parser::DIRECTIVE_INCLUDE;
use crate::Configuration;
use crate::FilterParserError;

/// Result of processing a single line through the directive pipeline.
pub(crate) enum ProcessedLine<'line> {
    /// Line is a conditional directive itself, or not captured — skip it.
    Skip,
    /// Line is an `!#include` directive with the extracted (unresolved) path.
    Include(&'line str),
    /// Line is a regular rule that should be recorded.
    Rule,
}

/// Shared per-line processing logic used by both Default and Streaming collectors.
/// Handles conditional directives (`!#if`/`!#else`/`!#endif`) and `!#include` detection.
pub(crate) struct LineProcessor<'conf> {
    conditional_directives_parser: ConditionalDirectivesProcessor<'conf>,
}

impl<'conf> LineProcessor<'conf> {
    pub(crate) fn new(configuration: &'conf Configuration) -> Self {
        Self {
            conditional_directives_parser: ConditionalDirectivesProcessor::new(configuration),
        }
    }

    /// Processes a single line through the directive pipeline.
    ///
    /// Returns:
    /// - `ProcessedLine::Skip` if the line is a conditional directive or not currently captured
    /// - `ProcessedLine::Include(path)` if the line is an `!#include` directive
    /// - `ProcessedLine::Rule` this line must be written into output
    pub(crate) fn process<'line>(
        &mut self,
        line: &'line str,
    ) -> Result<ProcessedLine<'line>, FilterParserError> {
        let trimmed = line.trim();

        // 1. Check conditional directives
        let is_conditional = self.conditional_directives_parser.process(trimmed)?;

        if is_conditional {
            return Ok(ProcessedLine::Skip);
        }

        // 2. Check if we are capturing lines
        if !self.conditional_directives_parser.is_capturing_lines() {
            return Ok(ProcessedLine::Skip);
        }

        // 3. Check for !#include directive
        if trimmed.starts_with(DIRECTIVE_INCLUDE) {
            if let Some(include_path) = get_include_path(trimmed)? {
                return Ok(ProcessedLine::Include(include_path));
            }
        }

        // 4. Regular rule
        Ok(ProcessedLine::Rule)
    }
}
