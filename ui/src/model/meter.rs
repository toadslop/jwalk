use leptos::prelude::IntoRender;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, Default)]
pub struct Meter(u32);

impl Display for Meter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m", self.0)
    }
}

impl IntoRender for Meter {
    type Output = String;

    fn into_render(self) -> Self::Output {
        self.to_string()
    }
}
