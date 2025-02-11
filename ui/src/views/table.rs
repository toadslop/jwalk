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
        navigate("/not-found", NavigateOptions::default());
        unreachable!();
    };

    let Some(lang) = params.lang else {
        navigate("/not-found", NavigateOptions::default());
        unreachable!();
    };

    // TODO: should have a single place to set the locale higher up the component tree
    rust_i18n::set_locale(lang.as_ref());

    let Some(list_name) = params.list_name else {
        navigate("/not-found", NavigateOptions::default());
        unreachable!();
    };

    let mountains_resource: OnceResource<
        Result<Vec<Mountain>, crate::data_source::DataSourceError>,
    > = OnceResource::new(data_source.load_list(list_name, lang));

    Effect::watch(
        move || mountains_resource.get(),
        move |m, _, _| {
            if let Some(m) = m {
                set_mountains.set(m.to_owned().unwrap());
            };
        },
        true,
    );

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
