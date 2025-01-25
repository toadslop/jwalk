mod app;
mod model;

use app::App;
use leptos::view;

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(move || view! { <App/> })
}
