use crate::storage::sql_generators::operator::SQLOperator;
use rusqlite::types::Value;
use rusqlite::{params_from_iter, ParamsFromIter};

/// Builds SQL "IN" anonymous placeholders string.
///
/// * `placeholders_count` - How many "?" will be inside IN brackets (?, ?, ...).
///
/// ### Returns
///
/// `field IN(?,?,?...) string` if `placeholders_count` > 0
/// OR
/// `NULL == NULL` for an empty result
///
/// if this value is zero, then instead of the placeholder ID, a `condition will be returned that always gives a zero result`
pub(super) fn build_in_clause(field: &str, placeholders_count: usize) -> String {
    if placeholders_count == 0 {
        // Special hack for empty IN list.
        // If you have an empty vector of entities, this shouldn't match *all entries*.
        // This must match *no entries*
        return String::from("NULL == NULL");
    }

    let mut str = format!("{} IN (", field);
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
    sql: &mut String,
    where_clause: Option<SQLOperator>,
) -> rusqlite::Result<ParamsFromIter<Vec<Value>>> {
    Ok(if let Some(clause) = where_clause {
        let values = SQLOperator::process(clause)?;
        sql.push_str("WHERE ");
        sql.push_str(values.0.as_str());

        params_from_iter(values.1)
    } else {
        params_from_iter(vec![])
    })
}

#[cfg(test)]
mod tests {
    use super::build_in_clause;

    #[test]
    fn test_build_in_clause_empty() {
        let actual = build_in_clause("id", 0);

        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute("create table test (id INTEGER);", ()).unwrap();
        conn.execute("insert into test (id) values (1), (2), (3)", ())
            .unwrap();
        let count: i64 = conn
            .query_row(
                format!("select count(*) from test where {}", actual).as_str(),
                (),
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(count, 0);
        assert_eq!(actual, "NULL == NULL")
    }

    #[test]
    fn test_build_in_clause_is_not_empty() {
        let where_str = build_in_clause("id", 3);

        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute("create table test (id INTEGER);", ()).unwrap();
        conn.execute("insert into test (id) values (1), (2), (3)", ())
            .unwrap();
        let count: i64 = conn
            .query_row(
                format!("select count(*) from test where {}", where_str).as_str(),
                [1, 2, 3],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(count, 3);
        assert_eq!(where_str, "id IN (?,?,?)")
    }
}
