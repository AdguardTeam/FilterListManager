pub(crate) mod collector;
pub(crate) mod parsers;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub(crate) enum KnownMetadataProperty {
    Title,
    Description,
    Version,
    Expires,
    Homepage,
    /// Note: Has alias - Last modified
    TimeUpdated,
    License,
    Checksum,
    DiffPath,

    Unknown,
}

impl KnownMetadataProperty {
    pub(self) fn is_known(value: KnownMetadataProperty) -> bool {
        value != KnownMetadataProperty::Unknown
    }
}

impl From<&str> for KnownMetadataProperty {
    fn from(value: &str) -> Self {
        match value {
            "Title" => KnownMetadataProperty::Title,
            "Description" => KnownMetadataProperty::Description,
            "Version" => KnownMetadataProperty::Version,
            "Expires" => KnownMetadataProperty::Expires,
            "Homepage" => KnownMetadataProperty::Homepage,
            "TimeUpdated" | "Last modified" => KnownMetadataProperty::TimeUpdated,
            "Diff-Path" => KnownMetadataProperty::DiffPath,
            "License" => KnownMetadataProperty::License,
            "Checksum" => KnownMetadataProperty::Checksum,
            _ => KnownMetadataProperty::Unknown,
        }
    }
}
