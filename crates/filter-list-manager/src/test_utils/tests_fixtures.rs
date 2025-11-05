use crate::test_utils::RAIIFile;
use chrono::Utc;
use std::path::PathBuf;
use std::thread::ThreadId;
use url::Url;

/// Returns path to fixtures folder in the current crate
pub(crate) fn get_tests_fixtures_path() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.join("fixtures")
}

pub(crate) struct TestsFixtures {
    list: Vec<RAIIFile>,
    timestamp: i64,
    thread_span: ThreadId,
}

impl TestsFixtures {
    pub(crate) fn new() -> Self {
        Self {
            timestamp: Utc::now().timestamp(),
            list: vec![],
            thread_span: std::thread::current().id(),
        }
    }

    pub(crate) fn write(&mut self, basename: &str, contents: &str) -> Url {
        let fixtures = get_tests_fixtures_path();
        let path = fixtures.join(format!(
            "{}_{}_{:?}.txt",
            basename, self.timestamp, self.thread_span
        ));
        let url = Url::from_file_path(&path).unwrap();

        self.list.push(RAIIFile::write(&path, contents));

        url
    }
}
