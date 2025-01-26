use crate::model::Mountain;
use leptos::{
    component,
    prelude::{ElementChild, Get},
    server::OnceResource,
    view, IntoView,
};

// TODO: make it possible to use multiple data sources, like static (loaded at compile time), database, and mock
// will have async varieties
#[allow(clippy::unused_async)]
async fn load_data() -> Result<Vec<Mountain>, crate::Error> {
    let csv = include_str!("../../data/mountains.csv");
    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    let it: Vec<Mountain> = reader
        .deserialize::<Mountain>()
        .map(Result::unwrap)
        .collect();

    Ok(it)
}

#[component]
pub fn App() -> impl IntoView {
    let async_data = OnceResource::new(load_data());

    let async_result = move || {
        async_data
            .get()
            .unwrap_or(Ok(Vec::with_capacity(0)))
            .unwrap_or_default()
            .first()
            .unwrap_or(&Mountain {
                id: 0,
                name: "hi".into(),
            })
            .name
            .to_string()
    };

    view! { <p>{move || async_result }</p> }
}
