use super::KnownMetadataProperty;
use std::collections::HashMap;

const METADATA_LINE_MARKER: char = '!';

const METADATA_SPLIT_TOKEN: char = ':';

const MAX_METADATA_LINES: usize = 100;

/// Collects metadata from given line
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

    pub(crate) fn parse_line(&mut self, line: &str, lineno: usize) {
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
