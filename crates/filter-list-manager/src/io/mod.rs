use crate::io::error::IOError;
use regex::Regex;
use std::fs;

pub mod error;
pub(crate) mod fetch_by_schemes;
pub(crate) mod http;
pub(super) mod url_schemes;

/// Gets scheme from url. Doesn't do trim
///
/// Returns [`None`] if protocol delimiter was not found
pub(crate) fn get_scheme(url: &str) -> Option<&str> {
    if let Some(mut pos) = url.find("://") {
        if url.get(pos..pos + 1) == Some("/") {
            pos += 1
        }

        return Some(&url[0..pos]);
    }

    None
}

/// Gets authority from url. Doesn't do trim
///
/// Returns [`None`] if protocol delimiter was not found
pub(crate) fn get_authority(url: &str) -> Option<&str> {
    if let Some(pos) = url.find("//") {
        let origin = &url[pos + 2..];
        if let Some(slash_pos) = origin.find("/") {
            return Some(&origin[..slash_pos]);
        }

        return Some(origin);
    }

    None
}

/// [`read_filter_file`] error type
pub(crate) enum ReadFilterFileError {
    Io(IOError),
    Other(String),
}

impl From<std::io::Error> for ReadFilterFileError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.into())
    }
}

/// Tries to read filter file by url
pub(crate) fn read_filter_file(url: &String) -> Result<String, ReadFilterFileError> {
    let regex = Regex::new(r"file:///?").map_err(|why| {
        return ReadFilterFileError::Other(why.to_string());
    })?;

    let path = regex.replace(url, "");

    fs::read_to_string(path.to_string()).map_err(ReadFilterFileError::from)
}

/// Gets a #hash value from url
///
/// * `url` - url or path
///
/// Returns a tuple [`Some((path without hash, substring after hash)`] or [`None`] if hash doesn't exist or hash substring is empty
pub(crate) fn get_hash_from_url(url: &String) -> Option<(String, String)> {
    url.find('#').and_then(|index| {
        let (path, mut hash) = url.split_at(index);

        hash = &hash[1..];

        if hash.is_empty() {
            return None;
        }

        Some((String::from(path), String::from(hash)))
    })
}

#[cfg(test)]
mod tests {
    use super::{get_authority, get_hash_from_url, get_scheme};

    #[test]
    fn test_get_scheme() {
        vec![
            ("file:///C:Progra~1", Some("file")),
            (" file:///C:Progra~1", Some(" file")),
            (" https", None),
            (" https", None),
            ("archive-http://fe", Some("archive-http")),
        ]
        .into_iter()
        .for_each(|(absolute_url, expected)| {
            let string = absolute_url.to_string();
            let actual = get_scheme(&string);

            assert_eq!(actual, expected);
        })
    }

    #[test]
    fn test_get_authority() {
        [
            ("//example.com:8080", Some("example.com:8080")),
            ("//example.com:8080/", Some("example.com:8080")),
            ("https://example.com:8080/", Some("example.com:8080")),
            ("https://example.com/", Some("example.com")),
            ("https://example.com", Some("example.com")),
            ("example.com", None),
        ]
        .into_iter()
        .for_each(|(absolute_url, expected)| {
            let actual = get_authority(absolute_url);
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_get_hash_from_url() {
        [
            (
                "http://example.com/#list1",
                Some((String::from("http://example.com/"), String::from("list1"))),
            ),
            (
                "https://example.com/path#list2",
                Some((
                    String::from("https://example.com/path"),
                    String::from("list2"),
                )),
            ),
            ("https://example.com/#", None),
            ("https://example.com#", None),
            (
                "C:\\filters\\1\\patches\\60-m.patch#name",
                Some((
                    String::from("C:\\filters\\1\\patches\\60-m.patch"),
                    String::from("name"),
                )),
            ),
            ("C:\\filters\\1\\patches\\59-m.patch", None),
            (
                "../path.txt#name_name",
                Some((String::from("../path.txt"), String::from("name_name"))),
            ),
        ]
        .into_iter()
        .for_each(|(url, expected)| {
            let invariant = String::from(url);
            let actual = get_hash_from_url(&invariant);

            assert_eq!(actual, expected)
        })
    }
}