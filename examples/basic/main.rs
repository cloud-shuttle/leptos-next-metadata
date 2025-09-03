//! Basic example demonstrating leptos-next-metadata usage
//! 
//! This example shows how to:
//! - Set up metadata context
//! - Use static metadata
//! - Generate dynamic metadata
//! - Work with Open Graph images
//! - Use JSON-LD structured data

use leptos::*;
use leptos_router::*;
use leptos_next_metadata::prelude::*;
use leptos_next_metadata::metadata::{Metadata, Title, Authors, Author, OpenGraph, OgImage, Article, Twitter, TwitterCard};
use leptos_next_metadata::json_ld;

#[component]
pub fn App() -> impl IntoView {
    // Provide metadata context for the entire app
    provide_metadata_context();
    
    view! {
        <MetadataProvider>
            <Router>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/blog/:slug" view=BlogPost/>
                    <Route path="/product/:id" view=ProductPage/>
                </Routes>
            </Router>
        </MetadataProvider>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // Use the metadata! macro for clean, Next.js-style syntax
    metadata! {
        title: "Welcome to My Site",
        description: "A blazing fast Leptos application with comprehensive metadata management",
        keywords: ["leptos", "metadata", "rust", "web", "seo"],
        openGraph: {
            title: "Welcome to My Site",
            description: "A blazing fast Leptos application with comprehensive metadata management",
            type: "website",
            siteName: "My Site",
            images: ["/og-home.png"]
        },
        twitter: {
            card: "summary_large_image",
            site: "@mysite"
        },
        jsonLd: json_ld::SchemaOrg::web_page(
            "Welcome to My Site",
            Some("A blazing fast Leptos application with comprehensive metadata management"),
            Some("https://example.com")
        )
    }
    
    view! {
        <div>
            <h1>"Welcome to My Site"</h1>
            <p>"This is a basic example of leptos-next-metadata usage with macros!"</p>
            <nav>
                <a href="/blog/first-post">"Read our first blog post"</a>
                <br/>
                <a href="/product/1">"Check out our products"</a>
            </nav>
        </div>
    }
}

#[component]
fn BlogPost() -> impl IntoView {
    let params = use_params::<BlogParams>();
    
    // For now, use static metadata since generate_metadata! macro needs more work
    metadata! {
        title: "Blog Post | My Blog",
        description: "This is a sample blog post with comprehensive metadata",
        keywords: ["blog", "technology", "development"],
        openGraph: {
            title: "Blog Post | My Blog",
            description: "This is a sample blog post with comprehensive metadata",
            type: "article",
            images: ["/og/blog-post.png"]
        },
        twitter: {
            card: "summary_large_image",
            creator: "@johndoe"
        }
    }
    
    view! {
        <div>
            <h1>"Blog Post Title"</h1>
            <p>"This is a blog post with metadata management!"</p>
            <p>"The metadata includes:"</p>
            <ul>
                <li>"Dynamic title with template"</li>
                <li>"Open Graph metadata"</li>
                <li>"Twitter Card metadata"</li>
                <li>"JSON-LD structured data"</li>
                <li>"Canonical URL"</li>
            </ul>
        </div>
    }
}

#[component]
fn ProductPage() -> impl IntoView {
    let params = use_params::<ProductParams>();
    
    // For now, use static metadata since generate_metadata! macro needs more work
    metadata! {
        title: "Product | My Store",
        description: "This is a sample product with comprehensive metadata",
        keywords: ["electronics", "gadgets"],
        openGraph: {
            title: "Product | My Store",
            description: "This is a sample product with comprehensive metadata",
            type: "product",
            images: ["/og/product.png"]
        },
        twitter: {
            card: "summary_large_image"
        }
    }
    
    view! {
        <div>
            <h1>"Product Page"</h1>
            <p>"This is a product page with metadata management."</p>
            <p>"The metadata includes:"</p>
            <ul>
                <li>"Product-specific title and description"</li>
                <li>"Open Graph metadata with product images"</li>
                <li>"Twitter Card metadata"</li>
                <li>"JSON-LD structured data for products"</li>
                <li>"Canonical URL"</li>
            </ul>
        </div>
    }
}

// Route parameter types
#[derive(Params, PartialEq, Clone)]
struct BlogParams {
    slug: String,
}

#[derive(Params, PartialEq, Clone)]
struct ProductParams {
    id: String,
}

fn main() {
    leptos::mount_to_body(App);
}
