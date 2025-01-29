use rusqlite::{Connection, OptionalExtension};

/// Repository for SQLite maintenance schema - `sqlite_master`
pub(crate) struct DbSchemaRepository;

impl DbSchemaRepository {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    /// Check that table `tbl_name` exists
    pub(crate) fn is_table_exists(
        &self,
        conn: &Connection,
        tbl_name: &str,
    ) -> rusqlite::Result<bool> {
        let mut statement = conn.prepare(
            r#"
            SELECT
                COUNT(tbl_name)
            FROM
                [sqlite_master]
            WHERE
                type="table" AND
                tbl_name=?
            LIMIT 1
        "#,
        )?;

        let result = statement.query([tbl_name]).optional()?;

        if let Some(mut rows) = result {
            if let Some(row) = rows.next()? {
                return Ok(row.get::<usize, i32>(0)? > 0i32);
            }
        }

        Ok(false)
    }
}
