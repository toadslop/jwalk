#![warn(clippy::all, clippy::pedantic, clippy::cargo)]

mod app;
mod data_source;
mod error;
mod model;
mod unit;

use app::App;
use data_source::CsvDataSource;
pub use error::Error;
use leptos::view;

fn main() {
    console_error_panic_hook::set_once();
    let data_source = CsvDataSource::new();
    leptos::mount::mount_to_body(move || view! { <App data_source /> });
}
