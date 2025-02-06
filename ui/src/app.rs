use crate::{data_source::DataSource, model::Mountain};
use leptos::{
    component,
    control_flow::For,
    prelude::{signal, Effect, Get, Set},
    server::OnceResource,
    view, IntoView,
};
use thaw::{ConfigProvider, Table, TableBody};
use web_sys::{Navigator, Window};

static DEFAULT_LOCALE: &str = "en-US";
static SUPPORTED_LOCALES: [&str; 2] = ["en-US", "jp-JA"];

#[component]
pub fn App(data_source: impl DataSource) -> impl IntoView {
    let locale = web_sys::window()
        .as_ref()
        .map(Window::navigator)
        .as_ref()
        .and_then(Navigator::language)
        .filter(|a| SUPPORTED_LOCALES.contains(&a.as_str()))
        .unwrap_or(DEFAULT_LOCALE.to_string());

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
