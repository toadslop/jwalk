use super::{DataSource, DataSourceError};
use crate::{model, unit::Meter};
use gloo_console::log;
use serde::Deserialize;
use std::collections::HashMap;

static MOUNTAINS: &str = include_str!("../../../data/mountains.csv");
static REGIONS: &str = include_str!("../../../data/regions.csv");

fn id_to_data(id: i32) -> Option<&'static str> {
    match id {
        1 => Some(MOUNTAINS),
        _ => None,
    }
}

#[derive(Debug, Default)]
pub struct CsvDataSource;

impl CsvDataSource {
    pub fn new() -> Self {
        Self
    }
}

impl DataSource for CsvDataSource {
    async fn load_list(self, id: i32) -> Result<Vec<model::Mountain>, DataSourceError> {
        log!("running load list");
        let mut reader = csv::Reader::from_reader(REGIONS.as_bytes());
        log!("instantiated regions reader");
        let regions: HashMap<i32, Region> = reader
            .deserialize::<Region>()
            // TODO: partition
            .map(Result::unwrap)
            .map(|region| (region.id, region))
            .collect();
        log!("parsed regions");
        let mountains_csv = id_to_data(id).ok_or(DataSourceError::NotFound).unwrap();
        log!("found mountains csv");
        let mut reader = csv::Reader::from_reader(mountains_csv.as_bytes());

        let mountains: Vec<model::Mountain> = reader
            .deserialize::<Mountain>()
            // TODO: partition
            .map(Result::unwrap)
            .map(|mountain| model::Mountain {
                id: mountain.id,
                name: mountain.name,
                altitude: mountain.altitude,
                region: regions
                    .get(&mountain.region_id)
                    // TODO: handle error properly
                    .map(|a| a.name.clone())
                    .unwrap_or_default()
                    .clone(),
            })
            .collect();
        log!("parsed mountains");
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
}

#[derive(Debug, Clone, Deserialize)]
pub struct Region {
    #[serde(alias = "number")]
    pub id: i32,
    pub name: String,
}
