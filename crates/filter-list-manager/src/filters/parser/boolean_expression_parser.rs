use crate::utils::parsing::is_allowed_space;
use std::cell::Cell;

/// Operators for [`BooleanExpressionParser`]
enum ParserOperator {
    And,
    Or,
}

/// List of possible special tokens for [`BooleanExpressionParser`]
const TOKEN_LIST: [&str; 5] = ["!", "&&", "||", "(", ")"];

/// Parses simple boolean expressions, like `(windows || (mac && gt_catalina))`
pub(crate) struct BooleanExpressionParser {
    /// Brackets balance counter
    bracket_balance: Cell<i16>,

    /// List of "true" tokens
    flags: Vec<String>,
}

impl BooleanExpressionParser {
    /// Ctor
    ///
    /// * `flags` - List of strings-tokens that represents true conditions for compiler. See tests for example
    pub(crate) fn new(flags: Option<Vec<String>>) -> Self {
        Self {
            bracket_balance: Cell::new(0i16),
            flags: flags.unwrap_or_default(),
        }
    }

    /// Runs to next token
    fn next_token(text: &str) -> (&str, usize) {
        let original_len = text.len();
        let mut sample = text.trim_start();
        let mut offset = original_len - sample.len();

        'find_token: {
            for special_token in TOKEN_LIST {
                // Next token is special token
                if sample.starts_with(special_token) {
                    offset += special_token.len();
                    sample = special_token;
                    break 'find_token;
                }
            }

            let iter = sample.char_indices();
            for (idx, c) in iter {
                // Stops search on space
                if is_allowed_space(c) {
                    offset += idx;
                    sample = &sample[..idx];
                    break 'find_token;
                }

                // Try to find special token with lookahead
                let sub = &sample[idx..];
                for special_token in TOKEN_LIST {
                    if sub.starts_with(special_token) {
                        offset += idx;
                        sample = &sample[..idx];

                        break 'find_token;
                    }
                }
            }

            // Nothing found. Consume remaining sample
            offset += sample.len();
        }

        (sample, offset)
    }

    /// Evaluate expression
    pub(crate) fn eval(&mut self, expr: &str) -> Option<bool> {
        self.bracket_balance.set(0i16);

        self.expr(expr).and_then(|(result, _)| {
            if self.bracket_balance.get() != 0 {
                return None;
            }

            Some(result)
        })
    }

    /// Parse expression
    fn expr<'a>(&'a self, expr: &'a str) -> Option<(bool, &'a str)> {
        let (mut left, mut remainder) = self.term(expr)?;

        // TODO: Add Max line length?
        loop {
            let token = self.peek(remainder);

            if token.is_empty() {
                return Some((left, ""));
            }

            if token == ")" {
                self.bracket_balance.set(self.bracket_balance.get() - 1);

                if self.bracket_balance.get() < 0i16 {
                    return None;
                }

                remainder = self.take(remainder).1;

                return Some((left, remainder));
            }

            let (operator, rem) = self.operator(remainder)?;
            remainder = rem;

            match operator {
                ParserOperator::And => {
                    let right = self.term(remainder)?;
                    remainder = right.1;

                    left = left && right.0;

                    continue;
                }
                ParserOperator::Or => {
                    let right = self.expr(remainder)?;
                    return Some((left || right.0, right.1));
                }
            }
        }
    }

    /// Take term
    fn term<'a>(&'a self, expr: &'a str) -> Option<(bool, &'a str)> {
        let (token, remainder) = self.take(expr);

        match token {
            token if token.is_empty() => None,
            "!" => {
                let inversion = self.term(remainder)?;

                Some((!inversion.0, inversion.1))
            }
            "(" => {
                self.bracket_balance.set(self.bracket_balance.get() + 1);

                self.expr(remainder)
            }
            "true" => Some((true, remainder)),
            "false" => Some((false, remainder)),

            // TODO: to_string looks weird
            token => Some((self.flags.contains(&token.to_string()), remainder)),
        }
    }

    /// Take operator
    fn operator<'a>(&'a self, slice: &'a str) -> Option<(ParserOperator, &'a str)> {
        let (token, remainder) = self.take(slice);

        match token {
            "&&" => Some((ParserOperator::And, remainder)),
            "||" => Some((ParserOperator::Or, remainder)),
            _ => None,
        }
    }

    /// Take to the next token
    fn take<'a>(&'a self, slice: &'a str) -> (&'a str, &'a str) {
        let (token, count) = BooleanExpressionParser::next_token(slice);

        (token, &slice[count..])
    }

    /// Just look ahead to the next token
    fn peek<'a>(&'a self, slice: &'a str) -> &'a str {
        let (token, _) = BooleanExpressionParser::next_token(slice);

        token
    }
}

#[cfg(test)]
mod tests {
    use super::BooleanExpressionParser;

    #[test]
    fn tests_take() {
        let object = BooleanExpressionParser::new(Some(vec![]));

        [
            (" true", ("true", "")),
            (" (false ", ("(", "false ")),
            (" t||f", ("t", "||f")),
            (" true || false ", ("true", " || false ")),
        ]
        .iter()
        .for_each(|(line, expected)| {
            let actual = object.take(line);
            assert_eq!(actual, *expected);
        });
    }

    #[test]
    fn tests_eval() {
        let mut object =
            BooleanExpressionParser::new(Some(vec![String::from("windows"), String::from("iOS")]));

        [
            ("(adguard && (adguard_ext_firefox || adguard_app_windows || adguard_app_mac || adguard_app_android))", Some(false)),
            ("", None),
            (" (false ", None),
            (" t|| f", Some(false)),
            // windows is true
            (" mac || windows ", Some(true)),
            ("()", None),
            ("(false || (windows && true) ) ", Some(true)),
            ("(nonexistent || (windows && other) ) ", Some(false)),
            ("nonexistent * windows", None), // Unknown operator
            ("true", Some(true)),
            ("!true", Some(false)),
            ("!(true)", Some(false)),
            ("(!true)", Some(false)),
            ("(some))", None),
        ]
        .iter()
        .for_each(|(line, expected)| {
            let actual = object.eval(line);
            assert_eq!(actual, *expected);
        });
    }
}
