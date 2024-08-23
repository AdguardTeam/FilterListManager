use crate::storage::utils::build_in_clause;
use rusqlite::types::Value;
use rusqlite::Error::InvalidParameterName;
use std::cell::RefCell;
use std::rc::Rc;

/// SQL nested operators representation as indirect enum.
#[cfg_attr(test, derive(Debug))]
pub(crate) enum SQLOperator<'field> {
    /// group_id = [`Value`]
    FieldEqualValue(&'field str, Value),
    #[allow(dead_code)]
    /// group_id IS NULL
    FieldIsNull(&'field str),
    /// group_id < [`Value`]
    FieldLTValue(&'field str, Value),
    #[allow(dead_code)]
    /// group_id > [`Value`]
    FieldGTValue(&'field str, Value),
    #[allow(dead_code)]
    /// ([`SQLOperator`] OR [`SQLOperator`])
    Or(Box<SQLOperator<'field>>, Box<SQLOperator<'field>>),
    /// NOT [`SQLOperator`]
    Not(Box<SQLOperator<'field>>),
    /// ([`SQLOperator`] AND [`SQLOperator`])
    And(Box<SQLOperator<'field>>, Box<SQLOperator<'field>>),
    /// filter_id IN (..,..)
    FieldIn(&'field str, Vec<Value>),
}

impl<'a> SQLOperator<'a> {
    /// Cooks [`SQLOperatorsTreeResult`] for complex SQL query
    ///
    /// * `head_operator` - Head of operators tree.
    ///
    /// # Failure
    ///
    /// Fails, if params reference counting is wrong
    pub(crate) fn process(
        head_operator: SQLOperator<'a>,
    ) -> rusqlite::Result<SQLOperatorsTreeResult> {
        let params_container = Rc::new(RefCell::new(vec![]));

        let where_str = Self::process_operator(head_operator, Rc::clone(&params_container));

        let result = Rc::try_unwrap(params_container).map_err(|_| {
            InvalidParameterName("Got heap error while resolving SQLOperatorsTree".to_string())
        })?;

        Ok(SQLOperatorsTreeResult(where_str, result.into_inner()))
    }

    /// Processes one [`SQLOperator`]
    ///
    /// * `operator` - Current operator
    /// * `container` - Mutable params container
    ///
    /// Returns SQL string portion for concrete `operator`
    fn process_operator(operator: SQLOperator, container: Rc<RefCell<Vec<Value>>>) -> String {
        match operator {
            SQLOperator::FieldEqualValue(field, value) => {
                container.borrow_mut().push(value);

                format!("{} = ?", field)
            }
            SQLOperator::FieldLTValue(field, value) => {
                container.borrow_mut().push(value);

                format!("{} < ?", field)
            }
            SQLOperator::FieldGTValue(field, value) => {
                container.borrow_mut().push(value);

                format!("{} > ?", field)
            }
            SQLOperator::FieldIsNull(field) => {
                format!("{} IS NULL", field)
            }
            SQLOperator::Or(left, right) => {
                let lhs = Self::process_operator(*left, Rc::clone(&container));
                let rhs = Self::process_operator(*right, Rc::clone(&container));

                format!("({} OR {})", lhs, rhs)
            }
            SQLOperator::Not(inversion) => {
                let input = Self::process_operator(*inversion, Rc::clone(&container));

                format!("NOT {}", input)
            }
            SQLOperator::And(left, right) => {
                let lhs = Self::process_operator(*left, Rc::clone(&container));
                let rhs = Self::process_operator(*right, Rc::clone(&container));

                format!("({} AND {})", lhs, rhs)
            }
            SQLOperator::FieldIn(field, mut vec) => {
                let len = vec.len();
                container.borrow_mut().append(&mut vec);

                format!("{}{}", field, build_in_clause(len))
            }
        }
    }
}

/// Container for SQL, prepared for `rusqlite` module.
///
/// 0 - string expression, like (group_id IS NULL or group_id < ?)
/// 1 - [`Vec<Value>`] for [`rusqlite::ParamsFromIter`] binding
pub(crate) struct SQLOperatorsTreeResult(pub(crate) String, pub(crate) Vec<Value>);

#[cfg(test)]
mod tests {
    use super::{SQLOperator, SQLOperator::*, SQLOperatorsTreeResult};
    use crate::utils::memory::heap;
    use rusqlite::types::Value;

    #[test]
    fn test_or_clause() {
        let clause = Or(
            heap(FieldIsNull("group_id")),
            heap(FieldLTValue("group_id", 1.into())),
        );

        match SQLOperator::process(clause).unwrap() {
            SQLOperatorsTreeResult(str, params) => {
                assert_eq!("(group_id IS NULL OR group_id < ?)", str);
                assert_eq!(vec![Value::from(1)], params);
            }
        }
    }

    #[test]
    fn test_nested_or_clause() {
        let clause = Or(
            heap(Not(heap(Or(
                heap(FieldEqualValue("filter_id", 0.into())),
                heap(FieldIsNull("filter_id")),
            )))),
            heap(FieldIsNull("group_id")),
        );

        match SQLOperator::process(clause).unwrap() {
            SQLOperatorsTreeResult(str, params) => {
                assert_eq!(
                    "(NOT (filter_id = ? OR filter_id IS NULL) OR group_id IS NULL)",
                    str
                );
                assert_eq!(vec![Value::from(0)], params);
            }
        }
    }

    #[test]
    fn test_nested_and_operator() {
        let clause = Or(
            heap(Not(heap(And(
                heap(FieldEqualValue("filter_id", 0.into())),
                heap(FieldIsNull("filter_id")),
            )))),
            heap(FieldIsNull("group_id")),
        );

        match SQLOperator::process(clause).unwrap() {
            SQLOperatorsTreeResult(str, params) => {
                assert_eq!(
                    "(NOT (filter_id = ? AND filter_id IS NULL) OR group_id IS NULL)",
                    str
                );
                assert_eq!(vec![Value::from(0)], params);
            }
        }
    }

    #[test]
    fn test_in_clause() {
        let clause = Or(
            heap(FieldIn("filter_id", vec![1.into(), 2.into()])),
            heap(FieldIsNull("group_id")),
        );

        match SQLOperator::process(clause).unwrap() {
            SQLOperatorsTreeResult(str, params) => {
                assert_eq!("(filter_id IN (?,?) OR group_id IS NULL)", str);
                assert_eq!(params, vec![Value::from(1), Value::from(2)]);
            }
        }
    }
}
