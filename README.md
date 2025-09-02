# leptos-next-metadata

[![Crates.io](https://img.shields.io/crates/v/leptos-next-metadata.svg)](https://crates.io/crates/leptos-next-metadata)
[![Documentation](https://docs.rs/leptos-next-metadata/badge.svg)](https://docs.rs/leptos-next-metadata)
[![CI](https://github.com/cloud-shuttle/leptos-next-metadata/workflows/CI/badge.svg)](https://github.com/cloud-shuttle/leptos-next-metadata/actions)
[![Coverage](https://codecov.io/gh/cloud-shuttle/leptos-next-metadata/branch/main/graph/badge.svg)](https://codecov.io/gh/cloud-shuttle/leptos-next-metadata)
[![License](https://img.shields.io/crates/l/leptos-next-metadata.svg)](https://github.com/cloud-shuttle/leptos-next-metadata#license)

Next.js-style metadata management for Leptos v0.8.8+ with type-safe APIs, blazing-fast OG image generation, and comprehensive SEO optimization.

## ✨ Features

- 🚀 **2-7x faster** OG image generation than browser-based solutions
- 🦀 **Type-safe** metadata with compile-time validation
- 🎯 **Next.js compatible** API for easy migration
- 🖼️ **Dynamic OG images** with SVG templates and custom fonts
- 📊 **JSON-LD support** with Schema.org types
- 🔍 **SEO validation** with best practices enforcement
- ⚡ **SSR/CSR/Islands** - works with all Leptos rendering modes
- 📁 **File conventions** - automatic favicon, robots.txt, sitemap detection
- 🎨 **Template system** - Liquid templates for OG images
- 💾 **Smart caching** - multi-level caching for optimal performance

## 🚀 Quick Start

```rust
use leptos::*;
use leptos_next_metadata::prelude::*;

#[component]
fn HomePage() -> impl IntoView {
    // Static metadata
    metadata! {
        title: "Welcome to My Site",
        description: "A blazing fast Leptos application",
        openGraph: {
            title: "Welcome",
            type: "website",
            images: ["/og-home.png"],
        }
    }
    
    view! {
        <h1>"Welcome to My Site"</h1>
    }
}

#[component] 
fn BlogPost() -> impl IntoView {
    let params = use_params::<BlogParams>();
    
    // Dynamic metadata with async data loading
    generate_metadata! {
        async |params, parent| {
            let post = load_post(&params.slug).await?;
            
            Metadata {
                title: Title::Template {
                    template: "%s | My Blog".into(),
                    default: "My Blog".into(),
                },
                description: Some(post.excerpt),
                openGraph: Some(OpenGraph {
                    title: Some(post.title),
                    images: vec![
                        generate_og_image(&post).await?,
                    ],
                    ..Default::default()
                }),
                ..parent.await
            }
        }
    }
    
    view! {
        // Your component
    }
}
```

## 📦 Installation

```toml
[dependencies]
leptos-next-metadata = "0.1"
```

With specific features:

```toml
[dependencies]
leptos-next-metadata = { 
    version = "0.1",
    features = ["ssr", "og-images", "file-conventions"] 
}
```

## 📚 Documentation

- [**Getting Started Guide**](https://docs.rs/leptos-next-metadata)
- [**API Reference**](https://docs.rs/leptos-next-metadata)
- [**Examples**](./examples)
- [**Migration from Next.js**](./docs/migration.md)

## 🎯 Why leptos-next-metadata?

| Feature | leptos-next-metadata | leptos_meta | Manual Implementation |
|---------|---------------------|-------------|----------------------|
| Static Metadata | ✅ Macro-based | ✅ Component-based | ⚠️ Verbose |
| Dynamic Metadata | ✅ Async with caching | ❌ | ⚠️ Complex |
| OG Image Generation | ✅ 100ms avg | ❌ | ⚠️ 800ms+ with Puppeteer |
| JSON-LD | ✅ Type-safe | ❌ | ⚠️ Error-prone |
| File Conventions | ✅ Automatic | ❌ | ⚠️ Manual setup |
| SEO Validation | ✅ Built-in | ❌ | ❌ |
| Next.js Compatibility | ✅ Drop-in replacement | ❌ | ❌ |

## 🔧 Minimum Requirements

- Rust 1.75+ (for async traits)
- Leptos 0.8.8+
- Nightly Rust (optional, for advanced features)

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🔗 Links

- [Repository](https://github.com/cloud-shuttle/leptos-next-metadata)
- [Issues](https://github.com/cloud-shuttle/leptos-next-metadata/issues)
- [Discussions](https://github.com/cloud-shuttle/leptos-next-metadata/discussions)
- [Leptos Framework](https://leptos.dev/)
