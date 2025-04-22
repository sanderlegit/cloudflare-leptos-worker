use leptos::*;

use crate::components::show_data_from_api::ShowDataFromApi;

use leptos_meta::{provide_meta_context, Title}; // Title component for setting title

// #[component]
// pub fn App() -> impl IntoView {
//     view! {}
// }
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    // provide_meta_context still needed for MetaTags in HtmlShell
    provide_meta_context();

    view! {
        // Set the document title using the Title component
        <Title text="Welcome to Leptos" />

        <h1>"Hello world!"</h1>
        <ShowDataFromApi />
    }
}
