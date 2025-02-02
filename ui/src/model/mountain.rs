use crate::unit::Meter;
use leptos::prelude::IntoAttribute;
use leptos::{
    prelude::{AddAnyAttr, Update, WriteSignal},
    view, IntoView,
};
use serde::{Deserialize, Serialize};
use sort_button::{SortButton, SortOrder};
use thaw::{TableCell, TableHeaderCell, TableRow};

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

    pub fn table_header(data: WriteSignal<Vec<Mountain>>) -> impl IntoView {
        view! {
            <TableRow>
                <TableHeaderCell resizable=true min_width=80 style:width="80px">
                    Number
                    <SortButton on_click=move |(_, sort_order)| {
                        data.update(|rows| {
                            match sort_order {
                                SortOrder::Ascending => rows.sort_by(|a, b| a.id.cmp(&b.id)),
                                SortOrder::Descending => rows.sort_by(|a, b| b.id.cmp(&a.id)),
                            }
                        });
                    } />
                </TableHeaderCell>
                <TableHeaderCell resizable=true>Name</TableHeaderCell>
                <TableHeaderCell resizable=true>Altitude</TableHeaderCell>
            </TableRow>
        }
    }
}
