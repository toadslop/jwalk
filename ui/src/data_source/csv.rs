use super::{DataSource, DataSourceError};
use crate::{
    error::handle_batch_error,
    model::{
        self, difficulty_rating::DifficultyRating, locale::Locale, meter::Meter,
        translation_key::TranslationKey,
    },
};
use gloo_console::debug;
use serde::Deserialize;
use std::collections::HashMap;

static MOUNTAINS: &str = include_str!("../../../data/mountains.csv");
static REGIONS: &str = include_str!("../../../data/regions.csv");

fn id_to_data(id: &str) -> Option<&'static str> {
    debug!(id);
    match id {
        "100-famous" => Some(MOUNTAINS),
        _ => None,
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CsvDataSource;

unsafe impl Send for CsvDataSource {}

impl CsvDataSource {
    pub fn new() -> Self {
        Self
    }
}

impl DataSource for CsvDataSource {
    async fn load_list(
        self,
        id: String,
        locale: Locale,
    ) -> Result<Vec<model::Mountain>, DataSourceError> {
        let mut reader = csv::Reader::from_reader(REGIONS.as_bytes());

        let deserialized = reader.deserialize::<Region>();

        let regions: HashMap<i32, Region> = handle_batch_error(deserialized)
            .map(|region| (region.id, region))
            .collect();

        let mountains_csv = id_to_data(&id).ok_or(DataSourceError::NotFound)?;
        let mut reader = csv::Reader::from_reader(mountains_csv.as_bytes());

        let deserialized = reader.deserialize::<Mountain>();

        let mountains = handle_batch_error(deserialized)
            .map(|mountain| (regions.get(&mountain.region_id), mountain))
            .map(|(region, mountain)| {
                if let Some(region) = region {
                    Ok(model::Mountain {
                        id: mountain.id,
                        name: mountain.name,
                        altitude: mountain.altitude,
                        region: region.name.as_locale(locale).to_string(),
                        technical_difficulty: mountain.technical_difficulty,
                        physical_difficulty: mountain.physical_difficulty,
                    })
                } else {
                    Err(DataSourceError::NotFound)
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(mountains)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Mountain {
    #[serde(alias = "number")]
    pub id: i32,
    pub name: String,
    pub altitude: Meter,
    pub region_id: i32,
    #[serde(alias = "terrain_diff")]
    pub technical_difficulty: DifficultyRating,
    #[serde(alias = "physical_diff")]
    pub physical_difficulty: DifficultyRating,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Region {
    #[serde(alias = "number")]
    pub id: i32,
    pub name: TranslationKey,
}
