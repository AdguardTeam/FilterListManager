use crate::storage::entities::localisation::filter_group_localisation_entity::FilterGroupLocalisationEntity;
use crate::storage::repositories::Repository;
use rusqlite::{named_params, Error, Transaction};

/// Repository for group localisations
pub(crate) struct GroupLocalisationRepository;

impl GroupLocalisationRepository {
    pub(crate) const fn new() -> Self {
        Self {}
    }
}

impl Repository<FilterGroupLocalisationEntity> for GroupLocalisationRepository {
    const TABLE_NAME: &'static str = "[filter_group_localisation]";

    fn insert(
        &self,
        transaction: &Transaction<'_>,
        entities: Vec<FilterGroupLocalisationEntity>,
    ) -> Result<(), Error> {
        let mut statement = transaction.prepare(
            r"
            INSERT OR REPLACE INTO
                [filter_group_localisation]
                (
                    group_id,
                    lang,
                    name
                )
            VALUES
                (
                    :group_id,
                    :lang,
                    :name
                )",
        )?;

        for entity in entities.iter() {
            statement.execute(named_params! {
                ":group_id": entity.group_id,
                ":lang": entity.lang,
                ":name": entity.name
            })?;
        }

        Ok(())
    }
}
