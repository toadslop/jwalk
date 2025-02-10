use crate::{
    data_source::DataSource,
    model::{mountain::MountainParams, Mountain},
};
use leptos::{
    component,
    either::Either,
    prelude::{signal, Effect, For, Get, GetUntracked, Set},
    server::OnceResource,
    view, IntoView,
};
use leptos_router::hooks::use_params;
use thaw::{Table, TableBody};

use super::not_found::NotFound;

#[component]
pub fn MountainTable(data_source: impl DataSource) -> impl IntoView {
    let (mountains, set_mountains) = signal(vec![]);
    let params = use_params::<MountainParams>();

    let Ok(params) = params.get_untracked() else {
        return Either::Right(view! { <NotFound />});
    };

    let Some(list_name) = params.list_name else {
        return Either::Right(view! { <NotFound />});
    };

    let mountains_resource: OnceResource<
        Result<Vec<Mountain>, crate::data_source::DataSourceError>,
    > = OnceResource::new(data_source.load_list(list_name));

    Effect::watch(
        move || mountains_resource.get(),
        move |m, _, _| {
            if let Some(m) = m {
                set_mountains.set(m.to_owned().unwrap());
            };
        },
        true,
    );

    Either::Left(view! {
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
    })
}
