# RFC 0001: API Design Principles

- **Start Date**: 2024-03-15
- **RFC PR**: [#1](https://github.com/yourusername/leptos-next-metadata/pull/1)
- **Implementation PR**: [#2](https://github.com/yourusername/leptos-next-metadata/pull/2)

## Summary

This RFC establishes the core API design principles for leptos-next-metadata, focusing on developer ergonomics, type safety, and performance while maintaining compatibility with Next.js patterns.

## Motivation

leptos-next-metadata aims to bridge the gap between Next.js's excellent metadata API and Leptos's reactive system. The API must:

1. **Feel Familiar**: Developers migrating from Next.js should recognize patterns
2. **Leverage Rust**: Take advantage of compile-time guarantees and zero-cost abstractions
3. **Stay Performant**: Minimize runtime overhead and bundle size
4. **Remain Flexible**: Support both simple and complex use cases

## Detailed Design

### Core Principles

#### 1. Declarative Over Imperative

**Next.js Style (Declarative)**:
```javascript
export const metadata = {
  title: 'My Page',
  description: 'Page description'
}
```

**leptos-next-metadata (Declarative)**:
```rust
metadata! {
    title: "My Page",
    description: "Page description"
}
```

**Not This (Imperative)**:
```rust
set_title("My Page");
set_description("Page description");
```

**Rationale**: Declarative APIs are easier to reason about, compose, and optimize at compile time.

#### 2. Compile-Time Validation

```rust
// ✅ This validates at compile time
metadata! {
    openGraph: {
        type: "website",  // Valid enum variant
    }
}

// ❌ This won't compile
metadata! {
    openGraph: {
        type: "invalid-type",  // Compile error!
    }
}
```

**Rationale**: Catch metadata errors at compile time rather than runtime or in production.

#### 3. Zero-Cost Abstractions

```rust
// This macro...
metadata! {
    title: "Static Title",
}

// ...should expand to minimal code like:
leptos_meta::Title(|| "Static Title".to_string())
```

**Rationale**: Static metadata should have zero runtime cost beyond what leptos_meta requires.

### API Structure

#### Core Macro: `metadata!`

The primary interface for static metadata:

```rust
metadata! {
    // Basic metadata
    title: "Page Title",
    description: "Page description", 
    keywords: ["rust", "leptos", "web"],
    
    // Open Graph
    openGraph: {
        title: "OG Title",
        type: "website",
        images: ["/og-image.jpg"],
    },
    
    // Twitter Cards  
    twitter: {
        card: "summary_large_image",
        site: "@mysite",
    },
    
    // Icons
    icons: {
        icon: "/favicon.ico",
        apple: "/apple-touch-icon.png",
    }
}
```

#### Dynamic Metadata: `generate_metadata!`

For metadata that depends on async data:

```rust
generate_metadata! {
    async |params, parent| {
        let data = fetch_data(&params.id).await?;
        
        Metadata {
            title: Some(Title::Static(data.title)),
            description: Some(data.description),
            ..parent.await
        }
    }
}
```

#### Template System

Support Next.js-style title templates:

```rust
// Parent component
metadata! {
    title: {
        template: "%s | My Site",
        default: "My Site - Welcome"
    }
}

// Child component  
metadata! {
    title: "About"  // Becomes "About | My Site"
}
```

### Type System Design

#### Core Types

```rust
#[derive(Clone, Debug, Default)]
pub struct Metadata {
    pub title: Option<Title>,
    pub description: Option<String>,
    pub open_graph: Option<OpenGraph>,
    pub twitter: Option<Twitter>,
    pub icons: Option<Icons>,
    // ...
}

#[derive(Clone, Debug)]
pub enum Title {
    Static(String),
    Template { template: String, default: String },
    Absolute(String),
}

#[derive(Clone, Debug, Default)] 
pub struct OpenGraph {
    pub title: Option<String>,
    pub description: Option<String>,
    pub og_type: Option<OpenGraphType>,  // Enum for type safety
    pub images: Vec<OgImage>,
    // ...
}

#[derive(Clone, Debug)]
pub enum OpenGraphType {
    Website,
    Article,
    Profile,
    Book,
    // ... other valid OG types
}
```

#### Builder Pattern Support

For complex metadata construction:

```rust
let metadata = Metadata::builder()
    .title("My Page")
    .description("Page description")
    .open_graph(
        OpenGraph::builder()
            .title("OG Title")
            .og_type(OpenGraphType::Article)
            .images(vec![
                OgImage::builder()
                    .url("/image.jpg")
                    .width(1200)
                    .height(630)
                    .build()
            ])
            .build()
    )
    .build();
```

### Macro Implementation Strategy

#### Parsing Strategy

The `metadata!` macro should:

1. **Parse declarative syntax** into structured data
2. **Validate field names** at compile time
3. **Generate efficient leptos_meta calls**
4. **Optimize static content** for zero-cost abstractions

#### Code Generation

```rust
// Input macro
metadata! {
    title: "My Page",
    description: "Description",
}

// Generated code (conceptual)
{
    use leptos_meta::*;
    
    Title(|| "My Page".to_string());
    Meta()
        .name("description")
        .content("Description");
}
```

### Error Handling

#### Compile-Time Errors

```rust
metadata! {
    title: 123,  // Error: Expected string, found integer
    openGraph: {
        type: "invalid",  // Error: Invalid OpenGraph type
    }
}
```

#### Runtime Errors (Dynamic Metadata)

```rust
generate_metadata! {
    async |params, parent| {
        let data = fetch_data(&params.id).await
            .map_err(MetadataError::DataFetch)?;  // Structured error types
        
        Ok(Metadata {
            title: Some(Title::Static(data.title)),
            ..Default::default()
        })
    }
}
```

### Performance Considerations

#### Static Optimization

```rust
// ✅ Optimal - resolved at compile time
metadata! {
    title: "Static Title",
}

// ❌ Suboptimal - forces runtime resolution
let title = compute_title();
metadata! {
    title: title,
}
```

#### Bundle Size

- Core metadata types: ~5KB
- Static metadata macros: ~0KB (compile time only)  
- Dynamic metadata support: ~15KB
- OG image generation: ~150KB (optional)

## Drawbacks

### Learning Curve

The macro syntax might be unfamiliar to developers who prefer function-based APIs.

**Mitigation**: Provide both macro and builder patterns:

```rust
// Macro style (recommended)
metadata! { title: "My Page" }

// Builder style (alternative)
Metadata::builder().title("My Page").apply();
```

### Compile-Time Dependency

Heavy use of procedural macros increases compile time.

**Mitigation**: 
- Keep macros simple and focused
- Provide opt-out mechanisms for performance-critical builds
- Cache macro expansions where possible

## Rationale and Alternatives

### Alternative 1: Function-Based API

```rust
set_metadata(Metadata {
    title: Some("My Page".to_string()),
    description: Some("Description".to_string()),
    ..Default::default()
});
```

**Pros**: More explicit, no macros
**Cons**: Verbose, runtime overhead, no compile-time validation

### Alternative 2: Derive Macros

```rust
#[derive(Metadata)]
struct PageMetadata {
    title: &'static str,
    description: &'static str,
}

const METADATA: PageMetadata = PageMetadata {
    title: "My Page", 
    description: "Description",
};
```

**Pros**: Type-safe, compile-time
**Cons**: Less flexible, doesn't support dynamic content well

### Chosen Approach: Declarative Macros

The declarative macro approach balances:
- **Familiarity**: Similar to Next.js
- **Performance**: Compile-time optimization
- **Flexibility**: Supports both static and dynamic content
- **Safety**: Compile-time validation

## Prior Art

- **Next.js Metadata API**: Primary inspiration for syntax and patterns
- **leptos_meta**: Foundation for Leptos metadata management
- **sycamore-meta**: Alternative Rust web framework metadata approach

## Unresolved Questions

1. **Macro Hygiene**: How to handle variable capture in macro expansion?
2. **Error Messages**: How to provide helpful error messages for complex macro failures?
3. **IDE Support**: How to ensure good IDE support for macro syntax?
4. **Incremental Compilation**: How to optimize macro compilation for development?

## Future Possibilities

### Advanced Template System

```rust
metadata! {
    title: {
        template: "%s | {siteName}",
        default: "Welcome to {siteName}",
        variables: {
            "siteName": "My Amazing Site"
        }
    }
}
```

### Conditional Metadata

```rust
metadata! {
    title: "My Page",
    #[cfg(feature = "premium")]
    robots: { index: true },
    #[cfg(not(feature = "premium"))]
    robots: { index: false },
}
```

### Metadata Validation Rules

```rust
metadata! {
    title: "My Page",
    #[validate(length(min = 30, max = 60))]
    description: "This description will be validated for SEO length",
}
```

---

**Implementation Timeline**: 2 weeks
**Impact**: High - Foundation for entire library API