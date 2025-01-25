use crate::model::Mountain;
use leptos::{
    component,
    prelude::{signal, ElementChild, Get},
    server::Resource,
    view, IntoView,
};

async fn load_data(count: Vec<Mountain>) -> Vec<Mountain> {
    count
}

#[component]
pub fn App() -> impl IntoView {
    let (count, _) = signal(vec![]);
    let async_data = Resource::new(
        move || count.get(),
        // every time `count` changes, this will run
        load_data,
    );

    let async_result = move || {
        async_data
            .get()
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
