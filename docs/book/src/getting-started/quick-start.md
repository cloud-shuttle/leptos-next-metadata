# Quick Start

Get up and running with leptos-next-metadata in minutes. This guide assumes you have already [installed](installation.md) the library.

## Your First Metadata-Enhanced Component

Let's create a simple page component with metadata:

```rust
use leptos::*;
use leptos_next_metadata::prelude::*;

#[component]
fn HomePage() -> impl IntoView {
    // Add metadata to your component
    metadata! {
        title: "Welcome to My Site",
        description: "A blazingly fast Leptos application with great SEO",
        keywords: ["leptos", "rust", "webassembly", "seo"],
        openGraph: {
            title: "Welcome to My Site",
            description: "Built with Leptos and leptos-next-metadata",
            type: "website",
            url: "https://mysite.com",
            images: [{
                url: "https://mysite.com/og-image.png",
                width: 1200,
                height: 630,
                alt: "My Site Homepage"
            }],
        },
        twitter: {
            card: "summary_large_image",
            site: "@mysite",
            creator: "@myhandle",
        }
    }
    
    view! {
        <main>
            <h1>"Welcome to My Site"</h1>
            <p>"This page has rich metadata for excellent SEO!"</p>
        </main>
    }
}
```

That's it! Your component now has comprehensive metadata that will be automatically injected into the document head.

## Setting Up Your App

To use leptos-next-metadata in your application, wrap your app with the metadata provider:

```rust
use leptos::*;
use leptos_next_metadata::*;

#[component]
fn App() -> impl IntoView {
    // Provide metadata context to your entire app
    provide_metadata_context();
    
    view! {
        <Router>
            <Routes>
                <Route path="/" view=HomePage />
                <Route path="/about" view=AboutPage />
                <Route path="/blog/:slug" view=BlogPost />
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(App)
}
```

## Generated HTML Output

The metadata from your component automatically generates the following HTML in your document head:

```html
<!DOCTYPE html>
<html>
<head>
    <!-- Basic metadata -->
    <title>Welcome to My Site</title>
    <meta name="description" content="A blazingly fast Leptos application with great SEO">
    <meta name="keywords" content="leptos,rust,webassembly,seo">
    
    <!-- Open Graph -->
    <meta property="og:title" content="Welcome to My Site">
    <meta property="og:description" content="Built with Leptos and leptos-next-metadata">
    <meta property="og:type" content="website">
    <meta property="og:url" content="https://mysite.com">
    <meta property="og:image" content="https://mysite.com/og-image.png">
    <meta property="og:image:width" content="1200">
    <meta property="og:image:height" content="630">
    <meta property="og:image:alt" content="My Site Homepage">
    
    <!-- Twitter Cards -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:site" content="@mysite">
    <meta name="twitter:creator" content="@myhandle">
</head>
<body>
    <!-- Your component content -->
</body>
</html>
```

## Dynamic Metadata

For pages that load data dynamically, use the `generate_metadata!` macro:

```rust
use leptos::*;
use leptos_next_metadata::prelude::*;

#[component]
fn BlogPost() -> impl IntoView {
    let params = use_params::<BlogParams>();
    
    // Generate metadata based on loaded data
    generate_metadata! {
        async |params, parent| {
            // Load your data
            let post = fetch_blog_post(&params.slug).await?;
            
            Metadata {
                title: Some(Title::Static(post.title.clone())),
                description: Some(post.excerpt.clone()),
                openGraph: Some(OpenGraph {
                    title: Some(post.title.clone()),
                    description: Some(post.excerpt.clone()),
                    type: Some("article".to_string()),
                    images: vec![OgImage {
                        url: post.featured_image.clone(),
                        width: Some(1200),
                        height: Some(630),
                        alt: Some(format!("Cover image for {}", post.title)),
                    }],
                    ..Default::default()
                }),
                ..parent.await // Inherit from parent metadata
            }
        }
    }
    
    view! {
        <article>
            <h1>{post.title}</h1>
            <p>{post.content}</p>
        </article>
    }
}

async fn fetch_blog_post(slug: &str) -> Result<BlogPost, Error> {
    // Your data fetching logic here
    todo!()
}
```

## Adding JSON-LD Structured Data

Enhance your SEO with structured data (requires `json-ld` feature):

```rust
use leptos::*;
use leptos_next_metadata::prelude::*;

#[component]
fn ArticlePage() -> impl IntoView {
    let article_data = create_resource(|| (), |_| async {
        fetch_article().await
    });
    
    view! {
        {move || article_data.get().map(|article| view! {
            // Your article content
            <article>
                <h1>{&article.title}</h1>
                <p>{&article.content}</p>
            </article>
            
            // Add JSON-LD structured data
            <JsonLd data={
                Article::builder()
                    .headline(&article.title)
                    .description(&article.excerpt)
                    .author(Person::new(&article.author_name))
                    .date_published(&article.published_at)
                    .image(vec![article.featured_image.clone()])
                    .build()
            } />
        })}
    }
}
```

## File-Based Metadata Conventions

leptos-next-metadata automatically detects metadata files in your project (requires `file-conventions` feature):

```
src/
├── app/
│   ├── favicon.ico          # Automatically detected
│   ├── icon.png             # Becomes <link rel="icon">
│   ├── apple-icon.png       # Apple touch icon
│   ├── opengraph-image.png  # Default OG image
│   └── robots.txt           # SEO robots file
│
└── routes/
    ├── blog/
    │   └── opengraph-image.tsx  # Dynamic OG image for blog
    └── products/
        └── twitter-image.png    # Product-specific Twitter image
```

These files are automatically processed and the appropriate metadata is generated.

## Error Handling

leptos-next-metadata provides helpful error messages during development:

```rust
metadata! {
    title: "My Page",
    // This will show a helpful warning in development
    description: "This description is too long and exceeds the recommended 160 character limit for SEO purposes and should be shortened to improve search result snippets",
}
```

## Next Steps

Now that you have the basics working, explore more features:

1. **[Project Setup](project-setup.md)** - Configure for your project structure
2. **[Static Metadata](../guides/static-metadata.md)** - Deep dive into static metadata options
3. **[Dynamic Metadata](../guides/dynamic-metadata.md)** - Advanced dynamic metadata patterns
4. **[OG Images](../guides/og-images.md)** - Generate beautiful Open Graph images
5. **[SEO Optimization](../guides/seo-optimization.md)** - Best practices and validation

## Common Patterns

### Page Templates with Title Templates

```rust
// Root layout component
metadata! {
    title: {
        template: "%s | My Amazing Site",
        default: "My Amazing Site - Welcome"
    },
    description: "Default site description",
}

// Child page - title becomes "About | My Amazing Site"
metadata! {
    title: "About",
}
```

### Conditional Metadata

```rust
#[component]
fn UserProfile(user_id: i32) -> impl IntoView {
    let user_data = create_resource(move || user_id, fetch_user);
    
    view! {
        {move || {
            if let Some(user) = user_data.get() {
                metadata! {
                    title: format!("{}'s Profile", user.name),
                    description: format!("View {}'s profile and posts", user.name),
                }
            }
            
            view! { /* component content */ }
        }}
    }
}
```

---

**Great job!** You now have a working leptos-next-metadata setup. Continue with [Project Setup](project-setup.md) to learn about configuration and advanced patterns.