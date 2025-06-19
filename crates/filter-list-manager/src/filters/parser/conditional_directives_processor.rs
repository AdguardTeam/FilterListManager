use crate::filters::parser::boolean_expression_parser::BooleanExpressionParser;
use crate::filters::parser::{DIRECTIVE_ELSE, DIRECTIVE_ENDIF, DIRECTIVE_IF};
use crate::{Configuration, FilterParserError};
use nom::Slice;

/// if/else/endif nesting level counter
type ConditionalNestingLevel = i16;

pub(crate) struct ConditionalDirectivesProcessor<'c> {
    conditional_nesting_level: ConditionalNestingLevel,
    condition_disabled_at_nesting: ConditionalNestingLevel,
    nesting_stack: Vec<ConditionalNestingLevel>,
    boolean_expression_parser: BooleanExpressionParser<'c>,
}

impl<'c> ConditionalDirectivesProcessor<'c> {
    pub(crate) fn new(conf: &'c Configuration) -> Self {
        Self {
            conditional_nesting_level: 0,
            condition_disabled_at_nesting: 0,
            nesting_stack: vec![],
            boolean_expression_parser: BooleanExpressionParser::new(
                &conf.filters_compilation_policy,
            ),
        }
    }

    /// Processes conditional directives [`DIRECTIVE_IF`], [`DIRECTIVE_ELSE`], ...
    ///
    /// # Returns
    ///
    /// `true` if directive was encountered, `false` otherwise
    pub(crate) fn process(&mut self, line: &str) -> Result<bool, FilterParserError> {
        if line.starts_with(DIRECTIVE_IF) {
            let directive_expression = line.slice(DIRECTIVE_IF.len()..);
            if directive_expression.is_empty() {
                return FilterParserError::EmptyIf.err();
            }

            self.conditional_nesting_level += 1;

            if self.is_capturing_lines() {
                match self.boolean_expression_parser.eval(directive_expression) {
                    None => {
                        return FilterParserError::InvalidBooleanExpression.err();
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
                return FilterParserError::UnbalancedElse.err();
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
                return FilterParserError::UnbalancedEndIf.err();
            }

            return Ok(true);
        }

        Ok(false)
    }

    pub(crate) fn is_capturing_lines(&self) -> bool {
        self.condition_disabled_at_nesting == 0
    }
}

#[cfg(test)]
mod tests {
    use super::ConditionalDirectivesProcessor;
    use crate::{Configuration, FilterParserError};

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
        let conf = Configuration::default();
        let mut parser = ConditionalDirectivesProcessor::new(&conf);

        for (index, line) in test_filter.lines().enumerate() {
            let result = parser.process(line.trim());

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

        let conf = Configuration::default();
        let mut parser = ConditionalDirectivesProcessor::new(&conf);

        let mut error_encountered = false;
        for line in test_filter.lines() {
            if let Err(why) = parser.process(line.trim_start()) {
                error_encountered = true;
                assert_eq!(why, FilterParserError::UnbalancedElse);
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

        let conf = Configuration::default();
        let mut parser = ConditionalDirectivesProcessor::new(&conf);

        let mut error_encountered = false;
        for line in test_filter.lines() {
            if let Err(why) = parser.process(line.trim_start()) {
                error_encountered = true;
                assert_eq!(why, FilterParserError::UnbalancedElse);
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

        let conf = Configuration::default();
        let mut parser = ConditionalDirectivesProcessor::new(&conf);

        let mut error_encountered = false;
        for line in test_filter.lines() {
            if let Err(why) = parser.process(line.trim_start()) {
                error_encountered = true;
                assert_eq!(why, FilterParserError::UnbalancedEndIf);
                break;
            }
        }

        assert!(error_encountered);
    }
}
