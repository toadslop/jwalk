use crate::{
    data_source::DataSource,
    views::{not_found::NotFound, table::MountainTable},
};
use leptos::{component, view, IntoView};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path,
};
use thaw::ConfigProvider;

#[component]
pub fn App(data_source: impl DataSource) -> impl IntoView {
    view! {
        <ConfigProvider>
            <Router>
                <Routes fallback=NotFound>
                    <ParentRoute path=path!("/:lang") view=Outlet>
                        <Route path=path!("/lists/:list_name/table") view=move || view! { <MountainTable data_source /> }/>
                    </ParentRoute>
                    <Route path=path!("/not-found") view=NotFound />
                </Routes>
            </Router>
        </ConfigProvider>
    }
}

// TODO: make a proper 404 page
