use crate::filters::indexes::entities::{IndexEntity, IndexI18NEntity};

const FILTERS_JSON_STRING: &str = include_str!("../../tests/fixtures/filters.json");
const FILTERS_JSON_I18N_STRING: &str = include_str!("../../tests/fixtures/filters_i18n.json");

/// Build indices from fixtures
pub(crate) fn build_filters_indices_fixtures() -> serde_json::Result<(IndexEntity, IndexI18NEntity)>
{
    let index = serde_json::from_str::<IndexEntity>(FILTERS_JSON_STRING)?;
    let index_i18n = serde_json::from_str::<IndexI18NEntity>(FILTERS_JSON_I18N_STRING)?;

    Ok((index, index_i18n))
}
