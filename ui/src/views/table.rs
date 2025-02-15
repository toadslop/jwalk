use crate::util::navigate1;
use crate::{
    data_source::DataSource,
    model::{mountain::MountainParams, Mountain},
};
use leptos::{
    component,
    prelude::{signal, Effect, For, Get, GetUntracked, Set},
    server::OnceResource,
    view, IntoView,
};
use leptos_router::{
    hooks::{use_navigate, use_params},
    NavigateOptions,
};
use thaw::{Table, TableBody};

#[component]
pub fn MountainTable(data_source: impl DataSource) -> impl IntoView {
    let (mountains, set_mountains) = signal(vec![]);
    let params = use_params::<MountainParams>();
    let navigate = use_navigate();

    // TODO: use value of error to generate more helpful 404 message
    let Ok(params) = params.get_untracked() else {
        navigate1!("/not-found", navigate)
    };

    let Some(lang) = params.lang else {
        navigate1!("/not-found", navigate)
    };

    // TODO: should have a single place to set the locale higher up the component tree
    rust_i18n::set_locale(lang.as_ref());

    let Some(list_name) = params.list_name else {
        navigate1!("/not-found", navigate)
    };

    let mountains_resource: OnceResource<
        Result<Vec<Mountain>, crate::data_source::DataSourceError>,
    > = OnceResource::new(data_source.load_list(list_name, lang));

    Effect::new(move || {
        let mountains = mountains_resource.get();
        if let Some(mountains) = mountains {
            match mountains {
                Ok(mountains) => set_mountains.set(mountains),
                Err(_) => {
                    // TODO: depending on error, route to different error page
                    navigate("/not-found", NavigateOptions::default());
                }
            };
        };
    });

    view! {
        <Table>
            {Mountain::table_header(set_mountains)}
            <TableBody>
                <For
                    each=move || mountains.get()
                    key=|mountain| mountain.id
                    children=Mountain::into_table_row
                />
            </TableBody>
        </Table>
    }
}
