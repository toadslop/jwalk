use leptos::{view, IntoView};
use serde::{Deserialize, Serialize};
use thaw::{TableCell, TableHeaderCell, TableRow};

use crate::unit::Meter;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Mountain {
    #[serde(alias = "number")]
    pub id: i32,
    pub name: String,
    pub altitude: Meter,
}

impl PartialEq for Mountain {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Mountain {
    pub fn into_table_row(self) -> impl IntoView {
        view! {
            <TableRow>
                <TableCell>{self.id}</TableCell>
                <TableCell>{self.name}</TableCell>
                <TableCell>{self.altitude}</TableCell>
            </TableRow>
        }
    }

    pub fn table_header() -> impl IntoView {
        view! {
            <TableRow>
                <TableHeaderCell>Number</TableHeaderCell>
                <TableHeaderCell>Name</TableHeaderCell>
                <TableHeaderCell>Altitude</TableHeaderCell>
            </TableRow>
        }
    }
}
