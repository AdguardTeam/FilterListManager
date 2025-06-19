use crate::io::url_schemes::UrlSchemes;
use crate::io::{get_authority, get_scheme};
use crate::{FilterParserError, IOError};
use std::path::{Path, PathBuf};
use url::{ParseError, Url};

/// Build an absolute path based on another file path -`parent_filter_path` and `child_path`
/// especially for parser logic.
pub(crate) fn to_absolute_path(parent_filter_path: &str, child_path: &str) -> Option<PathBuf> {
    let mut root = PathBuf::from(parent_filter_path);

    root.pop();

    for chunk in child_path.split('/') {
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
    if !child_url.is_empty() && child_url.get(slice.len() - 1..) != Some("/") {
        if let (remainder, "/") = slice.split_at(slice.len() - 1) {
            slice = remainder;
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

    Ok(resolved_url)
}

/// Tries to resolve included path from `parent_url` and `include_path`
///
/// # Failure
///
/// Returns [`FilterParserError::SchemeIsIncorrect`] if parent url and included url have different schemes
pub(super) fn try_to_resolve_include_path_from_parent_url(
    parent_url: &str,
    include_path: &str,
) -> Result<String, FilterParserError> {
    let parent_scheme = get_scheme(parent_url);

    match get_scheme(include_path) {
        // If scheme is found, this is an absolute_path
        Some(current_scheme_raw) => {
            let current_scheme = UrlSchemes::from(current_scheme_raw);
            let parent_scheme = UrlSchemes::from(parent_scheme);

            // Can include only if the schemes match
            if UrlSchemes::File == current_scheme && current_scheme != parent_scheme {
                return Err(FilterParserError::SchemeIsIncorrect(String::from(
                    "\"file\" scheme can be included only from \"file\" scheme",
                )));
            }

            // Authorities must match for web schemes
            if parent_scheme.is_web_scheme() {
                compare_same_origin(parent_url, include_path, parent_scheme, current_scheme)?;
            }

            Ok(include_path.to_string())
        }
        // May be relative path
        None => {
            // Special case - anonymous protocol
            if include_path.starts_with("//") {
                let parent_scheme = parent_scheme.unwrap_or_default();
                // Special-special case - third slash
                let extra_slash = if parent_scheme == "file" && parent_url.starts_with("file:///") {
                    "/"
                } else {
                    ""
                };

                // Parent url always has right scheme
                Ok(format!("{}:{}{}", parent_scheme, extra_slash, include_path))
            } else {
                resolve_absolute_uri(parent_scheme.into(), parent_url, include_path)
            }
        }
    }
}

/// Parent and child must have the same origin.
pub(crate) fn compare_same_origin(
    parent_url: &str,
    child_url: &str,
    parent_scheme: UrlSchemes,
    child_scheme: UrlSchemes,
) -> Result<(), FilterParserError> {
    if parent_scheme == child_scheme && get_authority(parent_url) == get_authority(child_url) {
        return Ok(());
    }

    FilterParserError::other_err_from_to_string(
        "Included filter must have the same origin with the root filter",
    )
}

#[cfg(test)]
mod tests {
    use super::{to_absolute_path, to_absolute_url};
    use crate::filters::parser::paths::try_to_resolve_include_path_from_parent_url;
    use crate::FilterParserError;
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

    #[test]
    fn test_include_path_resolving() {
        [
            (
                "https://example.com/filters/safari/1.txt",
                "https://example.com/filter1.txt",
                Ok("https://example.com/filter1.txt".to_string()),
            ),
            (
                "https://example.com/filters/safari/1.txt",
                "ffwf",
                Ok("https://example.com/filters/safari/ffwf".to_string()),
            ),
            (
                "https://example.com/filters/safari/1.txt",
                "../../global_filter.txt",
                Ok("https://example.com/global_filter.txt".to_string()),
            ),
            (
                "https://example.com/filters/safari/1.txt",
                "../a/./b/c",
                Ok("https://example.com/filters/a/b/c".to_string()),
            ),
            (
                "https://example.com/filters/safari/1.txt",
                "//example.com/filter.txt",
                Ok("https://example.com/filter.txt".to_string()),
            ),
            (
                "file:///C:/filters/safari/1.txt",
                "//C:/same.scheme/filter.txt",
                Ok("file:///C:/same.scheme/filter.txt".to_string()),
            ),
            (
                "https://example.com/filters.txt",
                "file://Volumes/osx/users/user/filters.txt",
                Err(FilterParserError::SchemeIsIncorrect(
                    "\"file\" scheme can be included only from \"file\" scheme".to_string(),
                )),
            ),
            (
                "https://example.com/filters/safari/1.txt",
                "https://adguard.com/filter1.txt",
                FilterParserError::other_err_from_to_string(
                    "Included filter must have the same origin with the root filter",
                ),
            ),
        ]
        .into_iter()
        .for_each(|(base_url, url_like_string, expected_result)| {
            let method_result =
                try_to_resolve_include_path_from_parent_url(base_url, url_like_string);

            assert_eq!(method_result, expected_result);
        })
    }
}
