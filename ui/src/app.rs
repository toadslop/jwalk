use crate::{context::Context, data_source::DataSource, views::table::MountainTable};
use leptos::{component, prelude::provide_context, view, IntoView};
use thaw::ConfigProvider;

#[component]
pub fn App(data_source: impl DataSource) -> impl IntoView {
    provide_context(Context::init());

    view! {
        <ConfigProvider>
            <MountainTable data_source />
        </ConfigProvider>
    }
}
