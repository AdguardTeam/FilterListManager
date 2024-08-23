use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Clone))]
pub(crate) struct GroupLanguageMeta {
    pub(crate) name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Clone))]
pub(crate) struct TagLanguageMeta {
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Clone))]
pub(crate) struct FilterLanguageMeta {
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
}
