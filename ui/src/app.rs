use leptos::{
    component,
    prelude::{signal, ElementChild, Get},
    server::Resource,
    view, IntoView,
};

async fn load_data(count: i32) -> i32 {
    count
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let async_data = Resource::new(
        move || count.get(),
        // every time `count` changes, this will run
        load_data,
    );

    view! { <p>{move || async_data.get()}</p> }
}
