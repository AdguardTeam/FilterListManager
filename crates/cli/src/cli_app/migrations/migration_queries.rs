use rusqlite::Connection;

/// Migration queries executor interface
pub(crate) trait MigrationQueries {
    /// Select current schema version
    fn get_schema_version(&self) -> Result<i32, String>;

    /// Make batch execution of provided queries. Also, wraps them in transaction
    fn execute_migration_queries(&self, queries_str: &str) -> Result<(), String>;

    /// Write new schema version into db
    fn update_schema_version(&self, new_version: i32) -> Result<(), String>;

    /// Rollback migration transaction if failed
    fn rollback_migration(&self) -> Result<(), String>;
}

/// Migration queries executor
pub(crate) struct MigrationQueriesImpl {
    conn: Connection,
}

impl MigrationQueriesImpl {
    pub(crate) fn new(conn: Connection) -> Self {
        Self { conn }
    }
}

impl MigrationQueries for MigrationQueriesImpl {
    fn get_schema_version(&self) -> Result<i32, String> {
        let result = self.conn.query_row(
            "SELECT [schema_version] FROM [version] LIMIT 1",
            (),
            |row| row.get::<_, i32>(0),
        );

        result.map_err(|why| why.to_string())
    }

    fn execute_migration_queries(&self, queries_str: &str) -> Result<(), String> {
        let mut final_sql = String::new();
        final_sql += "BEGIN TRANSACTION;\n";
        final_sql += queries_str;
        final_sql += "\nCOMMIT;";

        self.conn
            .execute_batch(&final_sql)
            .map_err(|why| why.to_string())
    }

    fn update_schema_version(&self, new_version: i32) -> Result<(), String> {
        let result = self
            .conn
            .execute("UPDATE [version] SET schema_version=?1", [new_version]);

        match result {
            Ok(_) => Ok(()),
            Err(why) => Err(why.to_string()),
        }
    }

    fn rollback_migration(&self) -> Result<(), String> {
        let result = self.conn.execute("ROLLBACK;", ());

        match result {
            Ok(_) => Ok(()),
            Err(why) => Err(why.to_string()),
        }
    }
}
