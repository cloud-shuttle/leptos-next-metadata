//! Basic example demonstrating leptos-next-metadata usage
//! 
//! This example shows how to:
//! - Set up metadata context
//! - Use static metadata
//! - Generate dynamic metadata
//! - Work with Open Graph images
//! - Use JSON-LD structured data

use leptos::*;
use leptos_next_metadata::prelude::*;

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
    // Set metadata for this page
    let set_metadata = use_set_metadata().unwrap();
    
    // Set static metadata
    set_metadata(Metadata::with_title_and_description(
        "Welcome to My Site",
        "A blazing fast Leptos application with comprehensive metadata management"
    ).open_graph(OpenGraph {
        title: Some("Welcome to My Site".to_string()),
        description: Some("A blazing fast Leptos application with comprehensive metadata management".to_string()),
        r#type: Some("website".to_string()),
        site_name: Some("My Site".to_string()),
        images: vec![OgImage::new("/og-home.png")],
        ..Default::default()
    }).twitter(Twitter {
        card: Some(TwitterCard::SummaryLargeImage),
        site: Some("@mysite".to_string()),
        ..Default::default()
    }).json_ld(json_ld::utils::create_webpage(
        "Welcome to My Site",
        Some("A blazing fast Leptos application with comprehensive metadata management"),
        Some("https://example.com")
    )));
    
    view! {
        <div>
            <h1>"Welcome to My Site"</h1>
            <p>"This is a basic example of leptos-next-metadata usage."</p>
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
    
    // Generate dynamic metadata based on route parameters
    generate_metadata! {
        async |params, parent| {
            // Simulate fetching blog post data
            let post = fetch_blog_post(&params.slug).await?;
            let parent_meta = parent.await;
            
            // Create author schema
            let author = json_ld::utils::create_person(
                &post.author.name,
                Some(&post.author.bio),
                Some(&post.author.job_title),
                None
            );
            
            // Create organization schema
            let publisher = json_ld::utils::create_organization(
                "My Blog",
                Some("A blog about technology and development"),
                Some("https://example.com"),
                Some("https://example.com/logo.png")
            );
            
            // Create article schema
            let article = json_ld::utils::create_article(
                &post.title,
                Some(&post.excerpt),
                Some(author),
                Some(publisher)
            );
            
            // Generate OG image for the blog post
            let og_image_url = generate_blog_og_image(&post).await?;
            
            Metadata {
                title: Title::Template {
                    template: "%s | My Blog".into(),
                    default: "My Blog".into(),
                },
                description: Some(post.excerpt),
                keywords: Some(vec!["blog".to_string(), "technology".to_string(), "development".to_string()]),
                authors: Some(Authors::Single(Author {
                    name: post.author.name.clone(),
                    url: Some(post.author.profile_url.clone()),
                    email: None,
                    image: Some(post.author.avatar.clone()),
                })),
                open_graph: Some(OpenGraph {
                    title: Some(post.title.clone()),
                    description: Some(post.excerpt.clone()),
                    r#type: Some("article".to_string()),
                    images: vec![OgImage::new(&og_image_url)],
                    article: Some(Article {
                        published_time: Some(post.published_at),
                        modified_time: post.updated_at,
                        author: Some(post.author.profile_url),
                        section: Some("Technology".to_string()),
                        tags: Some(post.tags.clone()),
                    }),
                    ..Default::default()
                }),
                twitter: Some(Twitter {
                    card: Some(TwitterCard::SummaryLargeImage),
                    creator: Some(post.author.twitter_handle.clone()),
                    title: Some(post.title.clone()),
                    description: Some(post.excerpt.clone()),
                    image: Some(og_image_url),
                    ..Default::default()
                }),
                json_ld: Some(json_ld::utils::to_json_ld(&json_ld::SchemaOrg::Article(article))?),
                canonical: Some(format!("https://example.com/blog/{}", post.slug)),
                ..parent_meta
            }
        }
    }
    
    view! {
        <div>
            <h1>"Blog Post Title"</h1>
            <p>"This is a blog post with dynamic metadata generation."</p>
            <p>"The metadata is generated based on the route parameters and includes:"</p>
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
    
    // Generate dynamic metadata for product pages
    generate_metadata! {
        async |params, parent| {
            // Simulate fetching product data
            let product = fetch_product(&params.id).await?;
            let reviews = fetch_product_reviews(&params.id).await?;
            let parent_meta = parent.await;
            
            // Calculate aggregate rating
            let rating = if !reviews.is_empty() {
                reviews.iter()
                    .map(|r| r.rating)
                    .sum::<f64>() / reviews.len() as f64
            } else {
                0.0
            };
            
            // Create product schema
            let product_schema = json_ld::utils::create_product(
                &product.name,
                Some(&product.description),
                product.price,
                &product.currency
            );
            
            // Create organization schema for brand
            let brand = json_ld::utils::create_organization(
                &product.brand,
                None,
                None,
                None
            );
            
            // Generate OG image for the product
            let og_image_url = generate_product_og_image(&product).await?;
            
            Metadata {
                title: Title::Template {
                    template: "%s | My Store".into(),
                    default: "My Store".into(),
                },
                description: Some(product.description.clone()),
                keywords: Some(product.categories.clone()),
                open_graph: Some(OpenGraph {
                    title: Some(product.name.clone()),
                    description: Some(product.description.clone()),
                    r#type: Some("product".to_string()),
                    images: vec![OgImage::with_dimensions(&og_image_url, 1200, 630)],
                    ..Default::default()
                }),
                twitter: Some(Twitter {
                    card: Some(TwitterCard::SummaryLargeImage),
                    title: Some(product.name.clone()),
                    description: Some(product.description.clone()),
                    image: Some(og_image_url),
                    ..Default::default()
                }),
                json_ld: Some(json_ld::utils::to_json_ld(&json_ld::SchemaOrg::Product(product_schema))?),
                canonical: Some(format!("https://example.com/product/{}", product.id)),
                ..parent_meta
            }
        }
    }
    
    view! {
        <div>
            <h1>"Product Page"</h1>
            <p>"This is a product page with dynamic metadata generation."</p>
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

// Mock data structures
#[derive(Clone)]
struct BlogPost {
    title: String,
    excerpt: String,
    author: Author,
    published_at: chrono::DateTime<chrono::Utc>,
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
    tags: Vec<String>,
    slug: String,
}

#[derive(Clone)]
struct Author {
    name: String,
    bio: String,
    job_title: String,
    profile_url: String,
    avatar: String,
    twitter_handle: String,
}

#[derive(Clone)]
struct Product {
    id: String,
    name: String,
    description: String,
    price: f64,
    currency: String,
    brand: String,
    categories: Vec<String>,
}

#[derive(Clone)]
struct Review {
    rating: f64,
}

// Mock async functions
async fn fetch_blog_post(slug: &str) -> Result<BlogPost, Box<dyn std::error::Error>> {
    // Simulate async operation
    Ok(BlogPost {
        title: format!("Blog Post: {}", slug),
        excerpt: "This is a sample blog post excerpt.".to_string(),
        author: Author {
            name: "John Doe".to_string(),
            bio: "A passionate developer".to_string(),
            job_title: "Senior Developer".to_string(),
            profile_url: "https://example.com/author/john-doe".to_string(),
            avatar: "https://example.com/avatar/john-doe.jpg".to_string(),
            twitter_handle: "@johndoe".to_string(),
        },
        published_at: chrono::Utc::now(),
        updated_at: None,
        tags: vec!["technology".to_string(), "development".to_string()],
        slug: slug.to_string(),
    })
}

async fn fetch_product(id: &str) -> Result<Product, Box<dyn std::error::Error>> {
    // Simulate async operation
    Ok(Product {
        id: id.to_string(),
        name: format!("Product {}", id),
        description: "This is a sample product description.".to_string(),
        price: 29.99,
        currency: "USD".to_string(),
        brand: "Sample Brand".to_string(),
        categories: vec!["electronics".to_string(), "gadgets".to_string()],
    })
}

async fn fetch_product_reviews(id: &str) -> Result<Vec<Review>, Box<dyn std::error::Error>> {
    // Simulate async operation
    Ok(vec![
        Review { rating: 4.5 },
        Review { rating: 5.0 },
        Review { rating: 4.0 },
    ])
}

async fn generate_blog_og_image(post: &BlogPost) -> Result<String, Box<dyn std::error::Error>> {
    // Simulate OG image generation
    Ok(format!("/og/blog/{}.png", post.slug))
}

async fn generate_product_og_image(product: &Product) -> Result<String, Box<dyn std::error::Error>> {
    // Simulate OG image generation
    Ok(format!("/og/product/{}.png", product.id))
}

fn main() {
    leptos::mount_to_body(App);
}
