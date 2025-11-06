use super::KnownMetadataProperty;
use std::collections::HashMap;

const METADATA_LINE_MARKER: char = '!';

const METADATA_SPLIT_TOKEN: char = ':';

const MAX_METADATA_LINES: usize = 100;

/// Collects metadata from given line
#[derive(Clone)]
pub(crate) struct MetadataCollector {
    values: HashMap<KnownMetadataProperty, String>,
    non_empty_lines: u32,
    pub(crate) is_reached_eod: bool,
}

impl MetadataCollector {
    pub(crate) fn new() -> Self {
        Self {
            values: HashMap::new(),
            non_empty_lines: 0,
            is_reached_eod: false,
        }
    }

    /// Parse metadata line `$Property${METADATA_SPLIT_TOKEN} $Value` and return value for property
    pub(crate) fn parse_line_for(property: KnownMetadataProperty, line: &str) -> Option<String> {
        if !line.starts_with(METADATA_LINE_MARKER) {
            return None;
        }

        line.split_once(METADATA_SPLIT_TOKEN)
            .and_then(|(prop_candidate, value)| {
                if property.equals_str(
                    prop_candidate
                        .trim_start_matches(METADATA_LINE_MARKER)
                        .trim(),
                ) {
                    Some(value.trim().to_string())
                } else {
                    None
                }
            })
    }

    /// Collects metadata from given line if possible
    pub(crate) fn collect_line(&mut self, line: &str, lineno: usize) {
        self.non_empty_lines += line.is_empty() as u32;

        // 1. Max metadata lines exceeded, or
        // 2. Non-empty non-comment line reached, and it is not the first line of the list
        if lineno >= MAX_METADATA_LINES
            || (!line.is_empty()
                && !line.starts_with(METADATA_LINE_MARKER)
                && self.non_empty_lines > 0)
        {
            self.is_reached_eod = true;

            return;
        }

        let (prop, value) = match line.split_once(METADATA_SPLIT_TOKEN) {
            Some(parts) => (
                parts.0.trim_start_matches(METADATA_LINE_MARKER).trim(),
                parts.1.trim(),
            ),
            None => return,
        };

        let property_key: KnownMetadataProperty = prop.into();

        if !KnownMetadataProperty::is_known(property_key) {
            return;
        }

        self.select_known_field(property_key, value.to_string());
    }

    pub(crate) fn get(&self, prop: KnownMetadataProperty) -> String {
        self.values
            .get(&prop)
            .map(ToOwned::to_owned)
            .unwrap_or_default()
    }

    /// If we need to mark collector as "reached eod" from outside
    pub(crate) fn mark_reached_eod(&mut self) {
        self.is_reached_eod = true
    }
}

impl MetadataCollector {
    fn select_known_field(&mut self, prop: KnownMetadataProperty, value: String) {
        if value.is_empty() {
            return;
        }

        self.values.entry(prop).or_insert(value);
    }
}

#[cfg(test)]
mod tests {
    use super::MetadataCollector;
    use crate::filters::parser::metadata::KnownMetadataProperty;

    #[test]
    fn test_parse_line_for() {
        [
            (
                KnownMetadataProperty::DiffPath,
                "! Diff-Path: patches/v1.0.1-m-28334120-60.patch",
                "patches/v1.0.1-m-28334120-60.patch",
            ),
            (
                KnownMetadataProperty::TimeUpdated,
                "!TimeUpdated: 2024-07-31T12:31:19+00:00",
                "2024-07-31T12:31:19+00:00",
            ),
            (
                KnownMetadataProperty::TimeUpdated,
                "! Last modified: 123123",
                "123123",
            ),
        ]
        .into_iter()
        .for_each(|(prop, line, expected)| {
            let actual = MetadataCollector::parse_line_for(prop, line);

            assert_eq!(expected, actual.unwrap());
        });

        let none = MetadataCollector::parse_line_for(
            KnownMetadataProperty::DiffPath,
            "Diff-Path: patches/v1.0.1-m-28334120-60.patch",
        );
        assert!(none.is_none())
    }
}
