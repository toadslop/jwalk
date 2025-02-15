use crate::model::{self, locale::Locale};
pub mod csv;
pub use csv::CsvDataSource;
use serde::{Deserialize, Serialize};

pub trait DataSource: Send + Sync + Clone + Copy + 'static {
    fn load_list(
        self,
        list_id: String,
        locale: Locale,
    ) -> impl std::future::Future<Output = Result<Vec<model::Mountain>, DataSourceError>>
           + std::marker::Send
           + 'static;
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataSourceError {
    #[error("Requested data was not found.")]
    NotFound,
    #[error("An error occurred while loading data.")]
    InvalidData,
}
