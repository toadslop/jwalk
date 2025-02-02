use crate::model::Mountain;
use gloo_console::log;
use leptos::{
    component,
    control_flow::For,
    prelude::{signal, Effect, Get, Set},
    server::OnceResource,
    view, IntoView,
};
use thaw::{ConfigProvider, Table, TableBody};

// TODO: make it possible to use multiple data sources, like static (loaded at compile time), database, and mock
// will have async varieties
#[allow(clippy::unused_async)]
async fn load_data() -> Result<Vec<Mountain>, crate::Error> {
    log!("load data called");
    let csv = include_str!("../../data/mountains.csv");
    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    let data: Vec<Mountain> = reader
        .deserialize::<Mountain>()
        .map(Result::unwrap)
        .collect();

    Ok(data)
}

#[component]
pub fn App() -> impl IntoView {
    log!("rendered");
    let (mountains, set_mountains) = signal(vec![]);
    let mountains_resource = OnceResource::new(load_data());

    let watcher = Effect::watch(
        move || mountains_resource.get(),
        move |m, _, _| {
            if let Some(m) = m {
                set_mountains.set(m.to_owned().unwrap());
            };
        },
        true,
    );

    if !mountains.get().is_empty() {
        watcher.stop();
    }

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
