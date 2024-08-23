use crate::io::url_schemes::UrlSchemes;
use crate::{FilterParserError, IOError};
use std::path::{Path, PathBuf};
use url::{ParseError, Url};

/// Build an absolute path based on another file path -`parent_filter_path` and `child_path`
/// especially for parser logic.
pub(crate) fn to_absolute_path(parent_filter_path: &str, child_path: &str) -> Option<PathBuf> {
    let mut root = PathBuf::from(parent_filter_path);

    root.pop();

    let mut iter = child_path.split("/");
    while let Some(chunk) = iter.next() {
        match chunk {
            "." => { /* do nothing */ }

            ".." => {
                if !root.pop() {
                    return None;
                }
            }

            value => root.push(Path::new(value)),
        }
    }

    Some(root)
}

/// Build an absolute url based on another file path -`parent_filter_url` and `child_url`
/// especially for parser logic.
pub(crate) fn to_absolute_url(
    parent_filter_url: &str,
    child_url: &str,
) -> Result<String, ParseError> {
    let mut url = Url::parse(parent_filter_url)?;
    url = url.join(child_url)?;

    let mut slice = url.as_str();

    // Url always created with "/" after domain. We need to cut trailing slash
    if child_url.len() > 0 && child_url.get(slice.len() - 1..) != Some("/") {
        match slice.split_at(slice.len() - 1) {
            (remainder, "/") => {
                slice = remainder;
            }
            _ => {}
        }
    }

    Ok(slice.to_string())
}

/// Resolves absolute URI by scheme, absolute url and relative url
///
/// * `scheme` - Absolute url scheme
/// * `parent_url` - Absolute url
/// * `url` - Url, relative to `parent_url`
pub(crate) fn resolve_absolute_uri(
    scheme: UrlSchemes,
    absolute_url: &str,
    relative_url: &str,
) -> Result<String, FilterParserError> {
    let resolved_url: String;

    match scheme {
        UrlSchemes::File => match to_absolute_path(absolute_url, relative_url) {
            None => {
                return Err(FilterParserError::Io(IOError::NotFound(format!(
                    "Can't build absolute url. Tried from: {} and {} ",
                    absolute_url, relative_url
                ))));
            }
            Some(absolute_path) => resolved_url = absolute_path.to_string_lossy().to_string(),
        },
        UrlSchemes::Https | UrlSchemes::Http => match to_absolute_url(absolute_url, relative_url) {
            Ok(absolute_path) => {
                resolved_url = absolute_path;
            }
            Err(why) => {
                return Err(FilterParserError::Io(IOError::NotFound(format!(
                    "Cannot build absolute url from {} and {}. Error is: {}",
                    absolute_url, relative_url, why
                ))));
            }
        },
        _ => {
            return Err(FilterParserError::SchemeIsIncorrect(format!(
                "Parent url has incorrect scheme. Tried url: {}",
                relative_url
            )));
        }
    };

    return Ok(resolved_url);
}

#[cfg(test)]
mod tests {
    use super::{to_absolute_path, to_absolute_url};
    use std::path::PathBuf;

    struct BuildAbsoluteUrlTestStruct(&'static str, &'static str, &'static str);
    impl BuildAbsoluteUrlTestStruct {
        fn get_expected_value(&self) -> Option<String> {
            Some(self.2.to_string())
        }
    }

    #[test]
    fn to_build_absolute_path() {
        vec![
            BuildAbsoluteUrlTestStruct(
                "file:///C:/Users/user/own/own.txt",
                "base.txt#list1",
                "file:///C:/Users/user/own/base.txt#list1",
            ),
            BuildAbsoluteUrlTestStruct(
                "file:///C:/Users/user/up/up.txt",
                "../base.txt",
                "file:///C:/Users/user/base.txt",
            ),
            BuildAbsoluteUrlTestStruct(
                "file:///c:/Users/user/relative/relative.txt",
                "././base.txt#list1",
                "file:///c:/Users/user/relative/base.txt#list1",
            ),
        ]
        .iter()
        .for_each(|test_struct| {
            let actual = to_absolute_path(test_struct.0, test_struct.1);

            match test_struct.get_expected_value() {
                None => assert_eq!(actual, None),
                Some(expected) => assert_eq!(actual.unwrap(), PathBuf::from(expected)),
            }
        })
    }

    #[test]
    fn test_to_absolute_url() {
        let root_filter = String::from("https://example.org/filters/the_filter");

        let test_urls = [
            "../a/./b/c#list1",
            "/c/d",
            "e",
            "./f",
            "../g",
            "http://example.org/other_filters#list1",
            "example.org/yet_other_filters",
            "example.org",
            "http://example.org",
            "http://example.org/",
            "//example.org/other_filters/guess_the_scheme",
        ]
        .map(str::to_string);

        let expected_urls = [
            "https://example.org/a/b/c#list1",
            "https://example.org/c/d",
            "https://example.org/filters/e",
            "https://example.org/filters/f",
            "https://example.org/g",
            "http://example.org/other_filters#list1", // must-use URL from the directive verbatim in this case
            "https://example.org/filters/example.org/yet_other_filters",
            "https://example.org/filters/example.org",
            "http://example.org",
            "http://example.org/",
            "https://example.org/other_filters/guess_the_scheme",
        ]
        .map(str::to_string);

        test_urls.iter().enumerate().for_each(|(index, value)| {
            let actual = to_absolute_url(root_filter.as_str(), value.as_str()).unwrap();
            assert_eq!(actual.to_string(), expected_urls[index])
        });
    }
}
