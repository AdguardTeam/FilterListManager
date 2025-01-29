use crate::manager::models::configuration::Locale;
use crate::storage::entities::filter_entity::FilterEntity;
use crate::storage::entities::localisation::filter_localisation_entity::FilterLocalisationEntity;
use crate::storage::repositories::Repository;
use crate::storage::utils::build_in_clause;
use crate::FilterId;
use rusqlite::types::Value;
use rusqlite::{named_params, params_from_iter, Connection, Transaction};
use std::collections::HashMap;

/// Repository for filters localisations
pub(crate) struct FilterLocalisationRepository;

impl FilterLocalisationRepository {
    pub(crate) const fn new() -> Self {
        Self {}
    }

    pub(crate) fn select_available_locales(
        &self,
        connection: &Connection,
    ) -> rusqlite::Result<Vec<Locale>> {
        let mut statement = connection.prepare(
            r"
            SELECT
                DISTINCT lang
            FROM
                [filter_localisation]
        ",
        )?;

        let mut out: Vec<Locale> = vec![];
        let mut rows = statement.query(())?;
        while let Some(row) = rows.next()? {
            out.push(row.get(0)?);
        }

        Ok(out)
    }

    pub(crate) fn enrich_filter_lists_with_localisation(
        &self,
        connection: &Connection,
        filters: &mut [FilterEntity],
        locale: &Locale,
    ) -> rusqlite::Result<()> {
        let mut sql = String::from(
            r"
            SELECT
                filter_id,
                name,
                description
            FROM
               [filter_localisation]
            WHERE ",
        );
        sql += build_in_clause("filter_id", filters.len()).as_str();
        sql += " AND lang = ?";

        let mut statement = connection.prepare(sql.as_str())?;

        let mut params = filters
            .iter()
            .filter_map(|filter| filter.filter_id)
            .map(|filter_id| filter_id.into())
            .collect::<Vec<Value>>();

        params.push(locale.to_string().into());

        let mut rows = statement.query(params_from_iter(params))?;
        let mut map: HashMap<FilterId, (Option<String>, Option<String>)> = HashMap::new();
        while let Some(row) = rows.next()? {
            map.insert(row.get(0)?, (row.get(1)?, row.get(2)?));
        }

        filters.iter_mut().for_each(|filter| {
            // We have localisations only for index filters
            if !filter.is_custom() {
                match filter.filter_id {
                    None => {}
                    Some(filter_id) => {
                        if let Some((maybe_title, maybe_description)) = map.remove(&filter_id) {
                            if let Some(title) = maybe_title {
                                filter.title = title;
                            }

                            if let Some(description) = maybe_description {
                                filter.description = description;
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }
}

impl Repository<FilterLocalisationEntity> for FilterLocalisationRepository {
    const TABLE_NAME: &'static str = "[filter_localisation]";

    fn insert(
        &self,
        transaction: &Transaction<'_>,
        entities: &[FilterLocalisationEntity],
    ) -> rusqlite::Result<()> {
        let mut statement = transaction.prepare(
            r"
            INSERT OR REPLACE INTO
                [filter_localisation]
                (
                    filter_id,
                    lang,
                    name,
                    description
                )
            VALUES
                (
                    :filter_id,
                    :lang,
                    :name,
                    :description
                )
            ",
        )?;

        for entity in entities.iter() {
            statement.execute(named_params! {
                ":filter_id": entity.filter_id,
                ":lang": entity.lang,
                ":name": entity.name,
                ":description": entity.description,
            })?;
        }

        Ok(())
    }
}
