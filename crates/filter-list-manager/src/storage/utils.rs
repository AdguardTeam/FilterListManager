use crate::storage::sql_generators::operator::SQLOperator;
use rusqlite::types::Value;
use rusqlite::{params_from_iter, ParamsFromIter};

/// Builds SQL "IN" anonymous placeholders string.
///
/// * `placeholders_count` - How many "?" will be inside IN brackets (?, ?, ...).
///
/// ### Returns
///
/// ` IN(?,?,?...) string` if `placeholders_count` > 0
///
/// if this value is zero, then instead of the placeholder ID, a `condition will be returned that always gives a zero result`
pub(super) fn build_in_clause(placeholders_count: usize) -> String {
    if placeholders_count == 0 {
        // Special hack for empty IN list.
        // If you have an empty vector of entities, this shouldn't match *all entries*.
        // This must match *no entries*
        return String::from(" AND 1 == 2");
    }

    let mut str = String::from(" IN (");
    let mut placeholders_str = "?,".repeat(placeholders_count);
    placeholders_str.pop();

    str += placeholders_str.as_str();
    str += ")";

    str
}

/// Adds to SQL query WHERE string if `where_clause` is some
///
/// Returns modified or original SQL string and [`ParamsFromIter`] value for query methods
pub(super) fn process_where_clause(
    mut sql: String,
    where_clause: Option<SQLOperator>,
) -> rusqlite::Result<(String, ParamsFromIter<Vec<Value>>)> {
    let params = if let Some(clause) = where_clause {
        let values = SQLOperator::process(clause)?;
        sql += "WHERE ";
        sql += values.0.as_str();

        params_from_iter(values.1)
    } else {
        params_from_iter(vec![])
    };

    Ok((sql, params))
}
