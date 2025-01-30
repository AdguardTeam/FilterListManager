use std::cell::Cell;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::thread::current;
use std::time::{SystemTime, UNIX_EPOCH};

/// Struct for temporary databases path info
pub(crate) struct TestsDb(u64, HashSet<PathBuf>);

thread_local! {
    /// This will be incremented every time you build a path
    static TESTS_COUNTER: Cell<u64> = Cell::new(1);
}

impl TestsDb {
    pub(crate) fn new() -> Self {
        let duration = SystemTime::now().duration_since(UNIX_EPOCH);

        let postfix = match duration {
            Ok(dur) => dur.as_secs(),
            Err(_) => 0,
        };

        Self(postfix, HashSet::new())
    }

    /// Take the path passed into the function, append a prefix to it, and return it as the path for the temporary database.
    ///
    /// * `path_buf` - Base path of DB
    ///
    /// # Safety
    ///
    /// This function must be used inside mutex.
    pub(crate) unsafe fn build_temporary_db_name(&mut self, path_buf: PathBuf) -> PathBuf {
        let basename = path_buf.file_stem().unwrap_or_else(|| OsStr::new("db"));

        let current_thread_local_count = TESTS_COUNTER.get();
        TESTS_COUNTER.set(current_thread_local_count + 1);

        let mut new_filename = basename.to_os_string();

        /*
            New filename is a combination of:
            1. Run tests timestamp
            2. Current thread id
            3. Current thread local counter, which will increment every time you build db name
        */
        new_filename.push(format!(
            "_{}_{:?}_{}",
            self.0,
            current().id(),
            current_thread_local_count
        ));

        // Add extension
        new_filename.push(
            path_buf
                .extension()
                .map(|str| format!(".{}", str.to_string_lossy().to_string()))
                .unwrap_or_default(),
        );

        let new_buf = path_buf.with_file_name(new_filename);

        self.1.insert(new_buf.clone());

        new_buf
    }

    /// Tear down implementation
    pub(crate) fn tear_down(&self) {
        self.1.iter().for_each(|path| {
            fs::remove_file(path).unwrap_or_else(|why| println!("Teardown: {}", why))
        })
    }
}
