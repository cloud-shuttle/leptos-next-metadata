//! Basic example demonstrating leptos-next-metadata usage
//!
//! This example shows how to:
//! - Set up metadata context
//! - Use static metadata
//! - Generate dynamic metadata
//! - Work with Open Graph images
//! - Use JSON-LD structured data

use leptos::prelude::ElementChild;
use leptos::*;
// use leptos_next_metadata::prelude::*;
use leptos_next_metadata_macros::metadata;

fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            {metadata! {
                title: "Welcome to My Site",
                description: "A blazing fast Leptos application with comprehensive metadata management",
                keywords: ["leptos", "metadata", "rust", "web", "seo"],
                author: "Peter Hanssens",
                openGraph: {
                    title: "Welcome to My Site",
                    description: "A blazing fast Leptos application with comprehensive metadata management",
                    r#type: "website",
                    url: "https://example.com",
                    image: "https://example.com/og-image.jpg"
                },
                twitter: {
                    card: "summary_large_image",
                    title: "Welcome to My Site",
                    description: "A blazing fast Leptos application with comprehensive metadata management",
                    image: "https://example.com/twitter-image.jpg"
                }
            }}

            <h1>"Welcome to My Site"</h1>
            <p>"This is a basic example of leptos-next-metadata usage with macros!"</p>
            <p>"Check the page source to see the generated metadata tags."</p>
        </div>
    }
}
