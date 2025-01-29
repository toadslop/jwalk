use leptos::{prelude::AddAnyAttr, view, IntoView};
use serde::{Deserialize, Serialize};
use sort_button::SortButton;
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
                <TableHeaderCell resizable=true min_width=80 style:width="80px">
                    Number
                    <SortButton />
                </TableHeaderCell>
                <TableHeaderCell resizable=true>Name</TableHeaderCell>
                <TableHeaderCell resizable=true>Altitude</TableHeaderCell>
            </TableRow>
        }
    }
}
