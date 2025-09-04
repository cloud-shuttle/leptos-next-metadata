# 🔧 Core API Reference

> **Navigation**: [📚 Documentation Index](../index.md) | [🚀 Quick Start](../guides/getting-started.md) | [📋 Production Roadmap](../guides/PRODUCTION_ROADMAP.md)

## 📖 **Overview**

The core API provides the fundamental building blocks for metadata management in Leptos applications. This includes the main `Metadata` struct, supporting types, and utility functions.

---

## 🏗️ **Core Types**

### **`Metadata` Struct**

The central metadata container that holds all page metadata information.

### **Component Types**

#### **`MetaTags` Component**

A component that injects meta tags into the document head during server-side rendering.

```rust
use leptos_next_metadata::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <MetaTags />
        // ... rest of your app
    }
}
```

#### **`Body` Component**

A component to set metadata on the document's `<body>` element from within the application.

```rust
use leptos_next_metadata::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Body class="dark-theme" lang="en" dir="ltr" />
        // ... rest of your app
    }
}
```

#### **`Html` Component**

A component to set metadata on the document's `<html>` element from within the application.

```rust
use leptos_next_metadata::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Html lang="en" dir="ltr" data-theme="dark" />
        // ... rest of your app
    }
}
```

#### **`HashedStylesheet` Component**

A component that injects a hashed stylesheet link into the document head for cargo-leptos integration.

```rust
use leptos_next_metadata::prelude::*;
use leptos::prelude::LeptosOptions;

#[component]
fn App() -> impl IntoView {
    let options = LeptosOptions::builder()
        .output_name("my-app")
        .build();
        
    view! {
        <HashedStylesheet 
            options=options 
            id="main-stylesheet" 
            root="/assets" 
        />
        // ... rest of your app
    }
}
```

#### **`EnhancedTitle` Component**

An enhanced title component with formatter support for dynamic title generation.

```rust
use leptos_next_metadata::prelude::*;

#[component]
fn App() -> impl IntoView {
    // Basic usage
    let _title = view! { <EnhancedTitle text="My Page" /> };
    
    // With formatter
    let formatter = |text: &str| format!("{} | My Site", text);
    let _formatted_title = view! { 
        <EnhancedTitle text="My Page" formatter=formatter />
    };
    
    // With template
    let _template_title = view! { 
        <EnhancedTitle 
            text="My Page" 
            template="{} | My Site"
        />
    };
    
    // With prefix and suffix
    let _prefixed_title = view! { 
        <EnhancedTitle 
            text="My Page" 
            prefix="Welcome to"
            suffix="| My Site"
        />
    };
}
```

```rust
pub struct Metadata {
    pub title: Option<Title>,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub authors: Option<Authors>,
    pub og_type: Option<String>,
    pub og_image: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_url: Option<String>,
    pub twitter_card: Option<TwitterCard>,
    pub twitter_title: Option<String>,
    pub twitter_description: Option<String>,
    pub twitter_image: Option<String>,
    pub robots: Option<Robots>,
    pub viewport: Option<Viewport>,
    pub canonical_url: Option<String>,
    pub alternate_links: Option<Vec<AlternateLink>>,
    pub json_ld: Option<Vec<JsonLd>>,
    // ... additional fields
}
```

#### **Example Usage**

```rust
use leptos_next_metadata::metadata::Metadata;

let metadata = Metadata {
    title: Some(Title::Static("My Page".into())),
    description: Some("Page description".into()),
    keywords: Some(vec!["rust".into(), "leptos".into()]),
    og_type: Some("website".into()),
    og_image: Some("/og-image.jpg".into()),
    ..Default::default()
};
```

---

### **`Title` Enum**

Represents different types of page titles with support for static, dynamic, and template-based titles.

```rust
pub enum Title {
    /// Static title that doesn't change
    Static(String),
    
    /// Dynamic title that can change based on signals
    Dynamic(Signal<String>),
    
    /// Template title with placeholder substitution
    Template(String),
}
```

#### **Examples**

```rust
use leptos_next_metadata::metadata::Title;
use leptos::*;

// Static title
let static_title = Title::Static("My Page".into());

// Dynamic title with signal
let (title, set_title) = create_signal("Dynamic Title".into());
let dynamic_title = Title::Dynamic(title);

// Template title
let template_title = Title::Template("{} - Site Name".into());
```

---

### **`Authors` Enum**

Flexible author representation supporting both single authors and multiple authors.

```rust
pub enum Authors {
    /// Single author
    Single(Author),
    
    /// Multiple authors
    Multiple(Vec<Author>),
}
```

#### **Examples**

```rust
use leptos_next_metadata::metadata::{Authors, Author};

// Single author
let single_author = Authors::Single(Author {
    name: "John Doe".into(),
    email: Some("john@example.com".into()),
    url: Some("https://johndoe.com".into()),
});

// Multiple authors
let multiple_authors = Authors::Multiple(vec![
    Author {
        name: "Jane Smith".into(),
        email: None,
        url: Some("https://janesmith.com".into()),
    },
    Author {
        name: "Bob Johnson".into(),
        email: Some("bob@example.com".into()),
        url: None,
    },
]);
```

---

### **`OpenGraph` Struct**

Open Graph metadata for social media optimization.

```rust
pub struct OpenGraph {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub url: Option<String>,
    pub site_name: Option<String>,
    pub locale: Option<String>,
    pub type: Option<String>,
    pub audio: Option<String>,
    pub video: Option<String>,
    pub determiner: Option<String>,
}
```

#### **Example Usage**

```rust
use leptos_next_metadata::metadata::OpenGraph;

let og = OpenGraph {
    title: Some("OG Title".into()),
    description: Some("OG Description".into()),
    image: Some("/og-image.jpg".into()),
    url: Some("https://example.com/page".into()),
    site_name: Some("My Site".into()),
    locale: Some("en_US".into()),
    type: Some("website".into()),
    ..Default::default()
};
```

---

### **`TwitterCard` Enum**

Twitter Card types for social media sharing.

```rust
pub enum TwitterCard {
    Summary,
    SummaryLargeImage,
    App,
    Player,
}
```

#### **Example Usage**

```rust
use leptos_next_metadata::metadata::TwitterCard;

let twitter_card = TwitterCard::SummaryLargeImage;
```

---

### **`Robots` Struct**

Search engine directives and crawling instructions.

```rust
pub struct Robots {
    pub index: Option<bool>,
    pub follow: Option<bool>,
    pub noarchive: Option<bool>,
    pub nosnippet: Option<bool>,
    pub noimageindex: Option<bool>,
    pub nocache: Option<bool>,
}
```

#### **Example Usage**

```rust
use leptos_next_metadata::metadata::Robots;

let robots = Robots {
    index: Some(true),
    follow: Some(true),
    noarchive: Some(false),
    nosnippet: Some(false),
    noimageindex: Some(false),
    nocache: Some(false),
};
```

---

### **`Viewport` Struct**

Mobile device viewport configuration.

```rust
pub struct Viewport {
    pub width: Option<String>,
    pub height: Option<String>,
    pub initial_scale: Option<f32>,
    pub minimum_scale: Option<f32>,
    pub maximum_scale: Option<f32>,
    pub user_scalable: Option<bool>,
    pub viewport_fit: Option<String>,
}
```

#### **Example Usage**

```rust
use leptos_next_metadata::metadata::Viewport;

let viewport = Viewport {
    width: Some("device-width".into()),
    height: Some("device-height".into()),
    initial_scale: Some(1.0),
    minimum_scale: Some(0.5),
    maximum_scale: Some(2.0),
    user_scalable: Some(true),
    viewport_fit: Some("cover".into()),
};
```

---

## 🔧 **Core Functions**

### **`merge_metadata`**

Merges two metadata objects, with the second taking precedence over the first.

```rust
pub fn merge_metadata(
    base: &Metadata,
    override_metadata: &Metadata
) -> Result<Metadata, Error>
```

#### **Example Usage**

```rust
use leptos_next_metadata::metadata::{Metadata, merge_metadata, Title};

let base_metadata = Metadata {
    title: Some(Title::Template("{} - Site Name".into())),
    description: Some("Default description".into()),
    og_type: Some("website".into()),
    ..Default::default()
};

let page_metadata = Metadata {
    title: Some(Title::Static("Page Title".into())),
    og_image: Some("/page-image.jpg".into()),
    ..Default::default()
};

let merged = merge_metadata(&base_metadata, &page_metadata)?;
// Result: "Page Title - Site Name" with page-specific OG image
```

---

### **`validate_metadata`**

Validates metadata for SEO best practices and returns a validation score.

```rust
pub fn validate_metadata(metadata: &Metadata) -> ValidationResult
```

#### **Example Usage**

```rust
use leptos_next_metadata::metadata::{Metadata, validate_metadata, Title};

let metadata = Metadata {
    title: Some(Title::Static("My Page".into())),
    description: Some("Page description".into()),
    ..Default::default()
};

let validation = validate_metadata(&metadata);
println!("Validation score: {}", validation.score);
println!("Issues: {:?}", validation.issues);
```

---

## 📝 **Builder Pattern**

The `Metadata` struct supports a fluent builder pattern for easy construction.

```rust
use leptos_next_metadata::metadata::Metadata;

let metadata = Metadata::new()
    .title("My Page")
    .description("Page description")
    .keywords(vec!["rust", "leptos"])
    .og_type("website")
    .og_image("/og-image.jpg")
    .twitter_card(TwitterCard::SummaryLargeImage)
    .build();
```

---

## 🔍 **Error Handling**

All core functions return `Result<T, Error>` for robust error handling.

```rust
use leptos_next_metadata::{Error, Result};

fn create_metadata() -> Result<Metadata> {
    let metadata = Metadata {
        title: Some(Title::Static("My Page".into())),
        description: Some("Page description".into()),
        ..Default::default()
    };
    
    // Validate the metadata
    let validation = validate_metadata(&metadata)?;
    if validation.score < 80 {
        return Err(Error::ValidationError("Metadata score too low".into()));
    }
    
    Ok(metadata)
}
```

---

## 📊 **Performance Characteristics**

| Operation | Time Complexity | Memory Usage |
|-----------|----------------|--------------|
| Metadata Creation | O(1) | ~2KB per instance |
| Metadata Merge | O(n) | ~4KB for merged result |
| Validation | O(n) | ~1KB for validation result |
| Serialization | O(n) | ~2KB for JSON output |

---

## 🧪 **Testing**

All core types and functions include comprehensive unit tests.

```bash
# Run core API tests
cargo test metadata

# Run specific test modules
cargo test metadata::tests
cargo test metadata::merge::tests
cargo test metadata::validation::tests
```

---

## 🔗 **Related Documentation**

- **[Macros API](macros.md)** - Procedural macro documentation
- **[JSON-LD API](json-ld.md)** - Structured data generation
- **[OG Image API](og-image.md)** - Image generation
- **[File Conventions API](conventions.md)** - Asset detection

---

## 📞 **Getting Help**

- **GitHub Issues**: [Report bugs](https://github.com/cloud-shuttle/leptos-next-metadata/issues)
- **GitHub Discussions**: [Ask questions](https://github.com/cloud-shuttle/leptos-next-metadata/discussions)
- **Documentation**: [Index](../index.md)

---

*Last Updated: September 4, 2025*  
*Next: [Macros API](macros.md) | [JSON-LD API](json-ld.md)*
