use crate::{data_source::DataSource, model::Mountain};
use leptos::{
    component,
    prelude::{signal, Effect, For, Get, Set},
    server::OnceResource,
    view, IntoView,
};
use thaw::{Table, TableBody};

#[component]
pub fn MountainTable(data_source: impl DataSource) -> impl IntoView {
    let (mountains, set_mountains) = signal(vec![]);

    let mountains_resource: OnceResource<
        Result<Vec<Mountain>, crate::data_source::DataSourceError>,
    > = OnceResource::new(data_source.load_list(1));

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
