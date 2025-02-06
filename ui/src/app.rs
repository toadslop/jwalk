use crate::{context::Context, data_source::DataSource, model::Mountain};
use leptos::{
    component,
    control_flow::For,
    prelude::{provide_context, signal, Effect, Get, Set},
    server::OnceResource,
    view, IntoView,
};
use thaw::{ConfigProvider, Table, TableBody};

#[component]
pub fn App(data_source: impl DataSource) -> impl IntoView {
    provide_context(Context::init());

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
        <ConfigProvider>
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
        </ConfigProvider>
    }
}
