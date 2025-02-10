use leptos::prelude::IntoAttribute;
use leptos::Params;
use leptos::{
    prelude::{AddAnyAttr, Update, WriteSignal},
    view, IntoView,
};
use leptos_router::params::Params;
use serde::{Deserialize, Serialize};
use sort_button::{SortButton, SortOrder};
use thaw::{TableCell, TableHeaderCell, TableRow};

use super::difficulty_rating::DifficultyRating;
use super::locale::SupportedLocale;
use super::meter::Meter;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mountain {
    #[serde(alias = "number")]
    pub id: i32,
    pub name: String,
    pub altitude: Meter,
    pub region: String,
    pub technical_difficulty: DifficultyRating,
    pub physical_difficulty: DifficultyRating,
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
                <TableCell>{self.region}</TableCell>
                <TableCell>{self.physical_difficulty}</TableCell>
                <TableCell>{self.technical_difficulty}</TableCell>
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
                <TableHeaderCell resizable=true>
                    Name
                    <SortButton on_click=move |(_, sort_order)| {
                        data.update(|rows| {
                            match sort_order {
                                SortOrder::Ascending => rows.sort_by(|a, b| a.name.cmp(&b.name)),
                                SortOrder::Descending => rows.sort_by(|a, b| b.name.cmp(&a.name)),
                            }
                        });
                    } />
                </TableHeaderCell>
                <TableHeaderCell resizable=true>
                    Altitude
                    <SortButton on_click=move |(_, sort_order)| {
                        data.update(|rows| {
                            match sort_order {
                                SortOrder::Ascending => rows.sort_by(|a, b| a.altitude.cmp(&b.altitude)),
                                SortOrder::Descending => rows.sort_by(|a, b| b.altitude.cmp(&a.altitude)),
                            }
                        });
                    } />
                </TableHeaderCell>
                <TableHeaderCell resizable=true>
                    Region
                    <SortButton on_click=move |(_, sort_order)| {
                        data.update(|rows| {
                            match sort_order {
                                SortOrder::Ascending => rows.sort_by(|a, b| a.region.cmp(&b.region)),
                                SortOrder::Descending => rows.sort_by(|a, b| b.region.cmp(&a.region)),
                            }
                        });
                    } />
                </TableHeaderCell>
                <TableHeaderCell resizable=true>
                    Physical Difficulty
                    <SortButton on_click=move |(_, sort_order)| {
                        data.update(|rows| {
                            match sort_order {
                                SortOrder::Ascending => rows.sort_by(|a, b| a.physical_difficulty.cmp(&b.physical_difficulty)),
                                SortOrder::Descending => rows.sort_by(|a, b| b.physical_difficulty.cmp(&a.physical_difficulty)),
                            }
                        });
                    } />
                </TableHeaderCell>
                <TableHeaderCell resizable=true>
                    Technical Difficulty
                    <SortButton on_click=move |(_, sort_order)| {
                        data.update(|rows| {
                            match sort_order {
                                SortOrder::Ascending => rows.sort_by(|a, b| a.technical_difficulty.cmp(&b.technical_difficulty)),
                                SortOrder::Descending => rows.sort_by(|a, b| b.technical_difficulty.cmp(&a.technical_difficulty)),
                            }
                        });
                    } />
                </TableHeaderCell>
            </TableRow>
        }
    }
}

#[derive(Params, PartialEq, Clone)]
pub struct MountainParams {
    pub list_name: Option<String>,
    pub lang: Option<SupportedLocale>,
}
