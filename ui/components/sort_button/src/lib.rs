use leptos::{component, view, IntoView};
use thaw::{Button, ButtonSize};

#[component]
pub fn sort_button() -> impl IntoView {
    view! {
        <Button
            icon=icondata::BsSortUp
            size=ButtonSize::Small />
    }
}
