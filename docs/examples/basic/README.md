# Basic Example

A minimal Leptos application demonstrating core leptos-next-metadata features.

## Features Demonstrated

- Static metadata with the `metadata!` macro
- Basic Open Graph and Twitter Card setup
- Title templates and inheritance
- Simple favicon configuration

## Running the Example

```bash
# Clone the repository
git clone https://github.com/yourusername/leptos-next-metadata.git
cd leptos-next-metadata/docs/examples/basic

# Install dependencies
cargo build

# Run development server
cargo leptos watch
```

Visit http://localhost:3000 to see the example in action.

## Project Structure

```
src/
├── app.rs              # Main app component with global metadata
├── components/
│   ├── home.rs         # Homepage with static metadata
│   ├── about.rs        # About page with title template
│   └── contact.rs      # Contact page with custom OG image
├── main.rs             # Application entry point
└── lib.rs              # Library exports

public/
├── favicon.ico         # Site favicon
├── og-default.jpg      # Default Open Graph image
└── style.css          # Basic styling

Cargo.toml              # Dependencies and features
```

## Code Highlights

### Global Metadata Setup

```rust
// src/app.rs
#[component]
fn App() -> impl IntoView {
    provide_metadata_context();
    
    // Global metadata applied to all pages
    metadata! {
        title: {
            template: "%s | Basic Example",
            default: "Basic Example - leptos-next-metadata"
        },
        description: "A basic example of leptos-next-metadata",
        openGraph: {
            site_name: "Basic Example",
            locale: "en_US",
            type: "website",
            images: ["/og-default.jpg"],
        },
        twitter: {
            card: "summary_large_image",
        }
    }
    
    view! {
        <Router>
            <Routes>
                <Route path="/" view=HomePage />
                <Route path="/about" view=AboutPage />
                <Route path="/contact" view=ContactPage />
            </Routes>
        </Router>
    }
}
```

### Page-Specific Metadata

```rust
// src/components/home.rs
#[component]
fn HomePage() -> impl IntoView {
    metadata! {
        title: "Welcome",  // Becomes "Welcome | Basic Example"
        description: "Welcome to our basic leptos-next-metadata example",
        keywords: ["leptos", "rust", "metadata", "example"],
    }
    
    view! {
        <main>
            <h1>"Welcome to Basic Example"</h1>
            <p>"This page demonstrates basic metadata features."</p>
        </main>
    }
}
```

## Learning Objectives

After exploring this example, you'll understand:

1. **Basic Setup**: How to configure leptos-next-metadata in a Leptos app
2. **Static Metadata**: Using the `metadata!` macro for static content
3. **Title Templates**: Creating reusable title patterns
4. **Metadata Inheritance**: How child components inherit parent metadata
5. **Basic SEO**: Essential metadata for search engines and social media

## Next Steps

- Explore the [Blog Example](../blog/) for dynamic metadata patterns
- Check out [Custom OG Images](../custom-og-images/) for image generation
- Review [Advanced SEO](../advanced-seo/) for comprehensive optimization

## Generated HTML

When you visit the homepage, leptos-next-metadata generates:

```html
<head>
    <title>Welcome | Basic Example</title>
    <meta name="description" content="Welcome to our basic leptos-next-metadata example">
    <meta name="keywords" content="leptos,rust,metadata,example">
    
    <!-- Open Graph -->
    <meta property="og:title" content="Welcome | Basic Example">
    <meta property="og:description" content="Welcome to our basic leptos-next-metadata example">
    <meta property="og:site_name" content="Basic Example">
    <meta property="og:locale" content="en_US">
    <meta property="og:type" content="website">
    <meta property="og:image" content="/og-default.jpg">
    
    <!-- Twitter Cards -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:title" content="Welcome | Basic Example">
    <meta name="twitter:description" content="Welcome to our basic leptos-next-metadata example">
    
    <!-- Icons -->
    <link rel="icon" href="/favicon.ico">
</head>
```