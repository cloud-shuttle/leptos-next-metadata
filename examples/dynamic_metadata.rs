use leptos::*;
use leptos_next_metadata::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <h1>"Dynamic Metadata Example"</h1>
            <p>"This example shows how to use the generate_metadata! macro for dynamic metadata generation."</p>

            <DynamicPage />
        </div>
    }
}

#[component]
fn DynamicPage() -> impl IntoView {
    view! {
        <div>
            {generate_metadata! {
                async || {
                    Metadata {
                        title: Some(Title::Static("Dynamic Blog Post".to_string())),
                        description: Some("This is a dynamically generated blog post with metadata".to_string()),
                        ..Default::default()
                    }
                }
            }}
            
            <h2>"Dynamic Blog Post"</h2>
            <p>"This page demonstrates dynamic metadata generation using the generate_metadata! macro."</p>
            <p>"The metadata is generated at runtime based on the content and can be customized based on user input, API responses, or other dynamic factors."</p>
        </div>
    }
}

#[component]
fn StaticPage() -> impl IntoView {
    view! {
        <div>
            {metadata! {
                title: "Static Page",
                description: "This is a static page with predefined metadata",
                keywords: ["static", "metadata", "example"]
            }}
            
            <h2>"Static Page"</h2>
            <p>"This page uses the static metadata! macro for predefined metadata."</p>
        </div>
    }
}
