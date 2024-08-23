use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Struct for temporary tests info
///
/// 0 - DB filename postfix
/// 1 - Vector of created databases
pub(crate) struct TestsDb(pub u64, HashSet<PathBuf>);

impl TestsDb {
    pub(crate) fn new() -> Self {
        let duration = SystemTime::now().duration_since(UNIX_EPOCH);

        let postfix = match duration {
            Ok(dur) => dur.as_secs() + std::process::id() as u64,
            Err(_) => 0,
        };

        Self(postfix, HashSet::new())
    }

    /// Take the path passed into the function, append a prefix to it, and return it as the path for the temporary database.
    ///
    /// * `path_buf` - Base path of DB
    pub(crate) fn build_temporary_db_name(&mut self, path_buf: PathBuf) -> PathBuf {
        if self.1.contains(&path_buf) {
            return path_buf;
        };

        let basename = path_buf.file_stem().unwrap_or_else(|| OsStr::new("db"));

        let mut new_filename = basename.to_os_string();
        new_filename.push(self.0.to_string());
        new_filename.push(path_buf.extension().unwrap_or_default());

        let new_buf = path_buf.with_file_name(new_filename);

        self.1.insert(new_buf.clone());

        new_buf
    }

    /// Tear down implementation
    pub(crate) fn tear_down(&self) {
        self.1
            .iter()
            .for_each(|path| fs::remove_file(path).unwrap_or_else(|why| println!("{}", why)))
    }

    /// Increments postfix
    pub fn increment_postfix(&mut self) {
        self.0 += 1;
    }
}
