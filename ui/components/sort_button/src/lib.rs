use icondata::Icon;
use leptos::{
    component,
    ev::MouseEvent,
    prelude::{Get, MaybeProp, RwSignal, Update},
    view, IntoView,
};
use thaw::{Button, ButtonSize};
use thaw_utils::BoxOneCallback;

#[component]
pub fn sort_button(
    #[prop(optional, into)] on_click: Option<BoxOneCallback<(MouseEvent, SortOrder)>>,
) -> impl IntoView {
    let icon_signal = RwSignal::new(SortOrder::default());

    let on_click = move |e| {
        icon_signal.update(|icon| {
            *icon = icon.toggle();
        });

        let Some(on_click) = on_click.as_ref() else {
            return;
        };
        on_click((e, icon_signal.get()));
    };

    let icon = MaybeProp::derive(move || Some(icon_signal.get().icon()));

    view! {
        <Button icon on_click size=ButtonSize::Small />
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    #[default]
    Ascending,
    Descending,
}

impl SortOrder {
    fn icon(&self) -> Icon {
        match self {
            Self::Ascending => icondata::BsSortUp,
            Self::Descending => icondata::BsSortDown,
        }
    }

    fn toggle(&self) -> Self {
        match self {
            Self::Ascending => Self::Descending,
            Self::Descending => Self::Ascending,
        }
    }
}
