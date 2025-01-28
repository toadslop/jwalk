use crate::model::Mountain;
use leptos::{component, control_flow::For, prelude::Get, server::OnceResource, view, IntoView};
use thaw::{ConfigProvider, Table, TableBody};

// TODO: make it possible to use multiple data sources, like static (loaded at compile time), database, and mock
// will have async varieties
#[allow(clippy::unused_async)]
async fn load_data() -> Result<Vec<Mountain>, crate::Error> {
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
    let async_data = OnceResource::new(load_data());

    let async_result = move || {
        let mut it = async_data
            .get()
            .unwrap_or(Ok(Vec::with_capacity(0)))
            .unwrap_or_default();

        it.sort_by(|a, b| a.id.cmp(&b.id));

        it
    };

    view! {
        <ConfigProvider>
            <Table>
                {Mountain::table_header()}
                <TableBody>
                    <For
                        each=move || async_result()
                        key=|mountain| mountain.id
                        children=Mountain::into_table_row
                    />
                </TableBody>
            </Table>
        </ConfigProvider>
    }
}
