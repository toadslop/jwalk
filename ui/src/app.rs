use crate::{context::Context, data_source::DataSource, views::table::MountainTable};
use leptos::{component, prelude::provide_context, view, IntoView};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path,
};
use thaw::ConfigProvider;

#[component]
pub fn App(data_source: impl DataSource) -> impl IntoView {
    provide_context(Context::init());

    view! {
        <ConfigProvider>
            <Router>
                <Routes fallback=|| "Not found.">
                <ParentRoute path=path!("/:lang") view=Outlet>
                    <Route path=path!("/lists/:list_name/table") view=move || view! { <MountainTable data_source /> }/>
                </ParentRoute>

                </Routes>
            </Router>
        </ConfigProvider>
    }
}
