/// Entity for filter_tag_localisation table
pub(crate) struct FilterTagLocalisationEntity {
    pub(crate) tag_id: i32,
    pub(crate) lang: String,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
}
