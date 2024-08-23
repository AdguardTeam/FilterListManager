use crate::storage::entities::localisation::filter_tag_localisation_entity::FilterTagLocalisationEntity;
use crate::storage::repositories::Repository;
use rusqlite::{named_params, Error, Transaction};

/// Repository for Tag localisation entities
pub(crate) struct FilterTagLocalisationRepository;

impl FilterTagLocalisationRepository {
    pub(crate) const fn new() -> Self {
        Self {}
    }
}

impl Repository<FilterTagLocalisationEntity> for FilterTagLocalisationRepository {
    const TABLE_NAME: &'static str = "[filter_tag_localisation]";

    fn insert(
        &self,
        transaction: &Transaction<'_>,
        entities: Vec<FilterTagLocalisationEntity>,
    ) -> Result<(), Error> {
        let mut statement = transaction.prepare(
            r"
            INSERT OR REPLACE INTO
                [filter_tag_localisation]
                (
                    tag_id,
                    lang,
                    name,
                    description
                )
            VALUES
                (
                    :tag_id,
                    :lang,
                    :name,
                    :description
                )
            ",
        )?;

        for entity in entities.iter() {
            statement.execute(named_params! {
                ":tag_id": entity.tag_id,
                ":lang": entity.lang,
                ":name": entity.name,
                ":description": entity.description,
            })?;
        }

        Ok(())
    }
}
