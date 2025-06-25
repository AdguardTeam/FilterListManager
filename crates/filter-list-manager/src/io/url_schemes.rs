#[derive(Eq, PartialEq, Copy, Clone)]
pub(crate) enum UrlSchemes {
    File,
    Https,
    Http,

    Empty,
    Other,
}

impl UrlSchemes {
    pub(crate) fn is_web_scheme(&self) -> bool {
        self == &UrlSchemes::Https || self == &UrlSchemes::Http
    }
}

impl PartialEq<Option<&str>> for UrlSchemes {
    fn eq(&self, other: &Option<&str>) -> bool {
        self == &UrlSchemes::from(*other)
    }
}

impl From<Option<&str>> for UrlSchemes {
    fn from(value: Option<&str>) -> Self {
        match value {
            None => UrlSchemes::Empty,
            Some(str) => Self::from(str),
        }
    }
}

impl From<&str> for UrlSchemes {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "https" => UrlSchemes::Https,
            "http" => UrlSchemes::Http,
            "file" => UrlSchemes::File,
            "" => UrlSchemes::Empty,
            _ => UrlSchemes::Other,
        }
    }
}
