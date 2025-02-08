use crate::model;
pub mod csv;
pub use csv::CsvDataSource;
use serde::{Deserialize, Serialize};

pub trait DataSource: Send + Sync + Clone + Copy + 'static {
    fn load_list(
        self,
        list_id: String,
    ) -> impl std::future::Future<Output = Result<Vec<model::Mountain>, DataSourceError>>
           + std::marker::Send
           + 'static;
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataSourceError {
    #[error("Requested data was not found.")]
    NotFound,
}
