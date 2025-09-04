# 🎭 Macros API Reference

> **Navigation**: [📚 Documentation Index](../index.md) | [🔧 Core API](core.md) | [🚀 Quick Start](../guides/getting-started.md)

## 📖 **Overview**

The macros API provides procedural macros for declarative metadata generation in Leptos applications. This includes the `metadata!` macro for static metadata and the `generate_metadata!` macro for dynamic metadata.

---

## 🚀 **`metadata!` Macro**

The `metadata!` macro provides a declarative way to define static metadata for your pages.

### **Basic Syntax**

```rust
use leptos_next_metadata::prelude::*;

metadata! {
    title: "My Page",
    description: "Page description",
    keywords: ["rust", "leptos", "metadata"],
    og_type: "website",
    og_image: "/og-image.jpg",
}
```

### **Complete Example**

```rust
use leptos::*;
use leptos_next_metadata::prelude::*;

#[component]
fn MyPage() -> impl IntoView {
    metadata! {
        title: "Welcome to My Site",
        description: "A blazing fast Leptos application with comprehensive metadata",
        keywords: ["leptos", "rust", "web", "seo", "metadata"],
        author: "John Doe",
        og_type: "website",
        og_title: "Welcome to My Site",
        og_description: "A blazing fast Leptos application",
        og_image: "/hero-image.jpg",
        og_url: "https://example.com",
        og_site_name: "My Awesome Site",
        og_locale: "en_US",
        twitter_card: "summary_large_image",
        twitter_title: "Welcome to My Site",
        twitter_description: "A blazing fast Leptos application",
        twitter_image: "/twitter-image.jpg",
        robots: {
            index: true,
            follow: true,
            noarchive: false,
        },
        viewport: {
            width: "device-width",
            height: "device-height",
            initial_scale: 1.0,
            user_scalable: true,
        },
        canonical_url: "https://example.com/page",
        alternate_links: [
            { hreflang: "en", href: "https://example.com/en/page" },
            { hreflang: "es", href: "https://example.com/es/page" },
        ],
    }
    
    view! { 
        <div>
            <h1>"Welcome to My Site"</h1>
            <p>"This is a blazing fast Leptos application!"</p>
        </div>
    }
}
```

---

## 🔄 **`generate_metadata!` Macro**

The `generate_metadata!` macro enables dynamic metadata generation based on runtime data.

### **Basic Syntax**

```rust
use leptos_next_metadata_macros::generate_metadata;

#[generate_metadata]
fn get_metadata() -> Metadata {
    Metadata {
        title: Some(Title::Static("Dynamic Page".into())),
        description: Some("Generated description".into()),
        og_type: Some("article".into()),
        ..Default::default()
    }
}
```

### **With Async Data**

```rust
use leptos::*;
use leptos_next_metadata_macros::generate_metadata;
use leptos_next_metadata::metadata::{Metadata, Title};

#[component]
fn BlogPost() -> impl IntoView {
    let post_id = use_params::<BlogPostParams>().get().unwrap().id;
    let post = create_resource(move || post_id, fetch_blog_post);

    generate_metadata! {
        move || async move {
            if let Some(post) = post.get().await {
                Metadata {
                    title: Some(Title::Static(post.title)),
                    description: Some(post.excerpt),
                    keywords: Some(post.tags),
                    author: Some(post.author.name),
                    og_type: Some("article".into()),
                    og_title: Some(post.title),
                    og_description: Some(post.excerpt),
                    og_image: Some(post.featured_image),
                    og_url: Some(format!("https://example.com/blog/{}", post.slug)),
                    twitter_card: Some("summary_large_image".into()),
                    twitter_title: Some(post.title),
                    twitter_description: Some(post.excerpt),
                    twitter_image: Some(post.featured_image),
                    json_ld: Some(vec![
                        JsonLd::Article(Article {
                            headline: post.title,
                            description: post.excerpt,
                            author: Some(post.author.name),
                            date_published: Some(post.published_at),
                            date_modified: Some(post.updated_at),
                            image: Some(post.featured_image),
                            ..Default::default()
                        })
                    ]),
                    ..Default::default()
                }
            } else {
                Metadata::default()
            }
        }
    }

    view! {
        <div>
            <h1>{move || post.get().map(|p| p.title).unwrap_or_default()}</h1>
            <p>{move || post.get().map(|p| p.content).unwrap_or_default()}</p>
        </div>
    }
}
```

---

## 🎨 **Macro Features**

### **Field Types**

The macros support various field types and expressions:

```rust
metadata! {
    // String literals
    title: "My Page",
    
    // Array literals
    keywords: ["rust", "leptos", "metadata"],
    
    // Boolean values
    robots: { index: true, follow: false },
    
    // Numeric values
    viewport: { initial_scale: 1.0, minimum_scale: 0.5 },
    
    // Nested structures
    open_graph: {
        title: "OG Title",
        description: "OG Description",
        image: "/og-image.jpg",
    },
    
    // Complex expressions
    canonical_url: format!("https://example.com/{}", page_slug),
}
```

### **Nested Structures**

Support for complex nested metadata structures:

```rust
metadata! {
    open_graph: {
        title: "Open Graph Title",
        description: "Open Graph Description",
        image: "/og-image.jpg",
        url: "https://example.com/page",
        site_name: "My Site",
        locale: "en_US",
        type: "website",
        audio: "/audio.mp3",
        video: "/video.mp4",
        determiner: "the",
    },
    
    twitter: {
        card: "summary_large_image",
        title: "Twitter Title",
        description: "Twitter Description",
        image: "/twitter-image.jpg",
        creator: "@username",
        site: "@site",
    },
    
    robots: {
        index: true,
        follow: true,
        noarchive: false,
        nosnippet: false,
        noimageindex: false,
        nocache: false,
    },
    
    viewport: {
        width: "device-width",
        height: "device-height",
        initial_scale: 1.0,
        minimum_scale: 0.5,
        maximum_scale: 2.0,
        user_scalable: true,
        viewport_fit: "cover",
    },
}
```

---

## 🔧 **Advanced Usage Patterns**

### **Conditional Metadata**

```rust
metadata! {
    title: "My Page",
    description: "Page description",
    og_type: if is_blog_post { "article" } else { "website" },
    og_image: if has_featured_image { featured_image_url } else { "/default-image.jpg" },
}
```

### **Dynamic Values with Signals**

```rust
use leptos::*;

#[component]
fn DynamicPage() -> impl IntoView {
    let (title, set_title) = create_signal("Dynamic Title".into());
    let (description, set_description) = create_signal("Dynamic description".into());

    metadata! {
        title: move || title.get(),
        description: move || description.get(),
        og_title: move || title.get(),
        og_description: move || description.get(),
    }

    view! {
        <div>
            <input 
                placeholder="Enter title"
                on:input=move |ev| set_title.set(event_target_value(&ev))
            />
            <input 
                placeholder="Enter description"
                on:input=move |ev| set_description.set(event_target_value(&ev))
            />
        </div>
    }
}
```

### **Template Titles**

```rust
metadata! {
    title: "{} - Site Name",  // Template with placeholder
    description: "Page description",
}
```

---

## 🚨 **Error Handling**

### **Compile-Time Validation**

The macros provide compile-time validation for common errors:

```rust
// ❌ This will fail to compile - invalid field
metadata! {
    title: "My Page",
    invalid_field: "value",  // Unknown field
}

// ❌ This will fail to compile - wrong type
metadata! {
    title: 42,  // Expected string, got integer
}

// ❌ This will fail to compile - invalid enum value
metadata! {
    twitter_card: "invalid_card_type",  // Not a valid TwitterCard
}
```

### **Runtime Error Handling**

```rust
use leptos_next_metadata::{Error, Result};

fn create_metadata() -> Result<Metadata> {
    let metadata = metadata! {
        title: "My Page",
        description: "Page description",
    }.map_err(|e| {
        eprintln!("Metadata creation failed: {}", e);
        Error::ValidationError("Invalid metadata".into())
    })?;
    
    Ok(metadata)
}
```

---

## 📊 **Performance Characteristics**

| Operation | Time Complexity | Memory Usage |
|-----------|----------------|--------------|
| Static metadata! | O(1) | ~2KB per instance |
| Dynamic generate_metadata! | O(n) | ~4KB for dynamic result |
| Compile-time validation | O(1) | No runtime overhead |
| Template processing | O(n) | ~1KB for template result |

---

## 🧪 **Testing**

### **Unit Testing**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos_next_metadata::metadata::Metadata;

    #[test]
    fn test_metadata_macro() {
        let metadata = metadata! {
            title: "Test Page",
            description: "Test description",
        };
        
        assert_eq!(metadata.title, Some(Title::Static("Test Page".into())));
        assert_eq!(metadata.description, Some("Test description".into()));
    }

    #[test]
    fn test_generate_metadata_macro() {
        #[generate_metadata]
        fn get_metadata() -> Metadata {
            Metadata {
                title: Some(Title::Static("Generated Title".into())),
                description: Some("Generated description".into()),
                ..Default::default()
            }
        }
        
        let metadata = get_metadata();
        assert_eq!(metadata.title, Some(Title::Static("Generated Title".into())));
    }
}
```

### **Integration Testing**

```rust
#[test]
fn test_metadata_integration() {
    let app = leptos::mount_to_body(|cx| {
        view! { cx,
            <div>
                {metadata! {
                    title: "Integration Test",
                    description: "Testing metadata integration",
                }}
                <h1>"Test Page"</h1>
            </div>
        }
    });
    
    // Verify metadata was generated correctly
    // ... test implementation
}
```

---

## 🔗 **Related Documentation**

- **[Core API](core.md)** - Core metadata types and functions
- **[JSON-LD API](json-ld.md)** - Structured data generation
- **[OG Image API](og-image.md)** - Image generation
- **[File Conventions API](conventions.md)** - Asset detection

---

## 📞 **Getting Help**

- **GitHub Issues**: [Report bugs](https://github.com/cloud-shuttle/leptos-next-metadata/issues)
- **GitHub Discussions**: [Ask questions](https://github.com/cloud-shuttle/leptos-next-metadata/discussions)
- **Documentation**: [Index](../index.md)

---

## 🎯 **Best Practices**

### **Do's**
- ✅ Use descriptive titles and descriptions
- ✅ Include relevant keywords
- ✅ Set appropriate Open Graph types
- ✅ Provide high-quality images for social sharing
- ✅ Use canonical URLs for duplicate content

### **Don'ts**
- ❌ Don't use keyword stuffing
- ❌ Don't leave metadata fields empty
- ❌ Don't use misleading titles or descriptions
- ❌ Don't forget mobile viewport settings
- ❌ Don't ignore social media optimization

---

*Last Updated: September 4, 2025*  
*Next: [JSON-LD API](json-ld.md) | [OG Image API](og-image.md)*
