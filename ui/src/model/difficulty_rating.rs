use leptos::{
    prelude::{AnyView, IntoAny, IntoRender},
    view,
};
use serde::{Deserialize, Serialize};
use thaw::Icon;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum DifficultyRating {
    Easy,
    Medium,
    Hard,
}

impl IntoRender for DifficultyRating {
    type Output = AnyView;

    fn into_render(self) -> Self::Output {
        match self {
            DifficultyRating::Easy => view! {
                <Icon icon=icondata::BsStarFill /><Icon icon=icondata::BsStar /><Icon icon=icondata::BsStar />
            }.into_any(),
            DifficultyRating::Medium => {
                view! {
                    <Icon icon=icondata::BsStarFill /><Icon icon=icondata::BsStarFill /><Icon icon=icondata::BsStar />
                }
            }
            .into_any(),
            DifficultyRating::Hard => view! {
                <Icon icon=icondata::BsStarFill /><Icon icon=icondata::BsStarFill /><Icon icon=icondata::BsStarFill />
            }.into_any(),
        }
    }
}
