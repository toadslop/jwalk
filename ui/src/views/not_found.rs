use leptos::prelude::ElementChild;
use leptos::{component, view, IntoView};
use rust_i18n::t;

#[component]
pub fn NotFound(
    #[prop(default = "The page you are looking for does not exist.".to_string())] message: String,
) -> impl IntoView {
    view! {
        <div>
            <h1>{"404 Not Found"}</h1>
            <p>{message}</p>
            <p>{t!("oze")}</p>
        </div>
    }
}
