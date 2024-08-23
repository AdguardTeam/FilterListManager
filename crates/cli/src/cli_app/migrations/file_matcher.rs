use regex::{Error, Regex};

/// Regex for matching migration files
const FILE_MATCHING_REGEX: &str = r"(\d+)-migration.sql";

/// Filename matcher
/// Matching based on migration index in filename
pub(crate) struct FileMatcher {
    regex: Regex,
}

impl FileMatcher {
    /// Creates matcher regexp object
    pub(crate) fn create_matcher() -> Result<Self, Error> {
        match Regex::new(FILE_MATCHING_REGEX) {
            Ok(regex) => Ok(Self { regex }),
            Err(why) => Err(why),
        }
    }

    /// Gets migration index from filename
    pub(crate) fn extract_migration_index(&self, file_name: &str) -> Result<i32, String> {
        if let Some(captures) = self.regex.captures(file_name) {
            if captures.len() > 0 {
                let index = captures[1].parse::<i32>().unwrap();
                return Ok(index);
            }
        }

        Err(format!(
            "File {} does not match pattern {}",
            file_name, FILE_MATCHING_REGEX
        ))
    }
}
