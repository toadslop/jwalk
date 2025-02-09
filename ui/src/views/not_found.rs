use leptos::prelude::ElementChild;
use leptos::{component, view, IntoView};

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div>
            <h1>{"404 Not Found"}</h1>
            <p>{"The page you are looking for does not exist."}</p>
        </div>
    }
}
