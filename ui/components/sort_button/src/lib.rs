use leptos::{
    component,
    prelude::{RwSignal, Update},
    view, IntoView,
};
use thaw::{Button, ButtonSize};

#[component]
pub fn sort_button() -> impl IntoView {
    let icon = RwSignal::new(icondata::BsSortUp);

    let on_click = move |_| {
        icon.update(|icon| {
            *icon = if *icon == icondata::BsSortUp {
                icondata::BsSortDown
            } else {
                icondata::BsSortUp
            }
        });
    };

    view! {
        <Button icon on_click size=ButtonSize::Small />
    }
}
