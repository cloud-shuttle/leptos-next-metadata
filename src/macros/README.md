# Procedural Macros

This module provides three powerful procedural macros that enable Next.js-style metadata management in Leptos applications.

## `metadata!` Macro

The `metadata!` macro provides a clean, declarative way to set static metadata for your pages.

### Basic Usage

```rust
use leptos_next_metadata::prelude::*;

#[component]
fn MyPage() -> impl IntoView {
    metadata! {
        title: "My Page",
        description: "This is my awesome page",
        keywords: ["rust", "leptos", "metadata"]
    }
    
    view! { <h1>"My Page"</h1> }
}
```

### Advanced Usage with Nested Objects

```rust
metadata! {
    title: "Blog Post",
    description: "An interesting blog post",
    openGraph: {
        title: "Blog Post - Open Graph",
        type: "article",
        images: ["/og-image.png"]
    },
    twitter: {
        card: "summary_large_image",
        site: "@mysite"
    },
    robots: {
        index: true,
        follow: true,
        nocache: false
    }
}
```

### Supported Fields

- **Basic**: `title`, `description`, `keywords`, `authors`
- **Open Graph**: `openGraph` (with nested fields)
- **Twitter**: `twitter` (with nested fields)
- **SEO**: `robots`, `canonical`, `alternates`
- **Viewport**: `viewport`, `themeColor`, `colorScheme`
- **Structured Data**: `jsonLd`

## `generate_metadata!` Macro

The `generate_metadata!` macro enables dynamic metadata generation based on route parameters, API data, or other runtime information.

### Basic Usage

```rust
#[component]
fn BlogPost() -> impl IntoView {
    let params = use_params::<BlogParams>();
    
    generate_metadata! {
        async |params, parent| {
            let post = fetch_post(&params.slug).await?;
            
            Metadata {
                title: Some(post.title),
                description: Some(post.excerpt),
                ..parent.await
            }
        }
    }
    
    view! { <h1>"Blog Post"</h1> }
}
```

### Advanced Usage with Complex Logic

```rust
generate_metadata! {
    async |params, parent| {
        let post = fetch_post(&params.slug).await?;
        let author = fetch_author(&post.author_id).await?;
        let og_image = generate_og_image(&post).await?;
        
        let parent_meta = parent.await;
        
        Metadata {
            title: Title::Template {
                template: "%s | My Blog".into(),
                default: "My Blog".into(),
            },
            description: Some(post.excerpt),
            open_graph: Some(OpenGraph {
                title: Some(post.title.clone()),
                description: Some(post.excerpt.clone()),
                images: vec![og_image],
                article: Some(Article {
                    published_time: Some(post.published_at),
                    author: Some(author.profile_url),
                    section: Some(post.category),
                    tags: Some(post.tags),
                }),
                ..Default::default()
            }),
            json_ld: Some(create_article_schema(&post, &author)?),
            canonical: Some(format!("https://example.com/blog/{}", post.slug)),
            ..parent_meta
        }
    }
}
```

### Parameters

- `params`: Route parameters (automatically available)
- `parent`: Parent metadata from the layout (awaitable)

## `og_image!` Macro

The `og_image!` macro provides a convenient way to generate Open Graph images with custom templates and data.

### Basic Usage

```rust
let og_image_url = og_image! {
    size: (1200, 630),
    template: "blog_post"
};
```

### Advanced Usage with Custom Data

```rust
let og_image_url = og_image! {
    size: (1200, 630),
    template: "blog_post",
    data: {
        title: post.title,
        author: post.author.name,
        date: post.published_at.format("%B %d, %Y"),
        category: post.category
    }
};
```

### Parameters

- `size`: Image dimensions as `(width, height)` tuple
- `template`: Template name or path
- `data`: Data object to inject into the template

## Field Mapping

The macros automatically map field names to their corresponding struct types:

| Macro Field | Rust Struct |
|-------------|-------------|
| `openGraph` | `OpenGraph` |
| `twitter` | `Twitter` |
| `article` | `Article` |
| `robots` | `Robots` |
| `viewport` | `Viewport` |
| `themeColor` | `ThemeColor` |
| `colorScheme` | `ColorScheme` |

## Error Handling

All macros provide compile-time error checking and will fail with helpful error messages if:

- Required fields are missing
- Field types don't match
- Syntax is invalid
- Unsupported fields are used

## Performance

The macros generate optimized Rust code at compile time, ensuring:

- Zero runtime overhead
- Type safety
- Efficient metadata construction
- Proper error handling

## Examples

See the `examples/` directory for complete working examples of all three macros in action.
