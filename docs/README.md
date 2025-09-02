# leptos-next-metadata

[![Crates.io](https://img.shields.io/crates/v/leptos-next-metadata.svg)](https://crates.io/crates/leptos-next-metadata)
[![Documentation](https://docs.rs/leptos-next-metadata/badge.svg)](https://docs.rs/leptos-next-metadata)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Build Status](https://github.com/yourusername/leptos-next-metadata/workflows/CI/badge.svg)](https://github.com/yourusername/leptos-next-metadata/actions)

**Next.js-style metadata management for Leptos** — bringing type-safe, high-performance SEO capabilities to Rust web applications.

## ✨ Features

- **🔒 Type Safety**: Compile-time metadata validation with Rust's type system
- **⚡ Performance**: 100ms OG image generation (7x faster than browser-based)
- **🎯 Developer Experience**: Next.js-compatible API with zero-cost abstractions
- **📱 SSR/CSR Ready**: Automatic optimization for different rendering contexts
- **🎨 OG Images**: Built-in SVG-to-PNG rendering with custom templates
- **📊 JSON-LD**: Type-safe structured data with Schema.org support
- **📁 File Conventions**: Automatic metadata detection from file system
- **🔍 SEO Validation**: Built-in best practices validation and suggestions

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
leptos-next-metadata = "0.1"
leptos = "0.8"
```

Define metadata in your components:

```rust
use leptos::*;
use leptos_next_metadata::prelude::*;

#[component]
fn HomePage() -> impl IntoView {
    metadata! {
        title: "Welcome to My Site",
        description: "A blazingly fast Leptos application",
        openGraph: {
            title: "Welcome",
            type: "website",
            images: ["/og-image.png"],
        },
        twitter: {
            card: "summary_large_image",
            site: "@mysite",
        }
    }
    
    view! {
        <h1>"Welcome!"</h1>
        <p>"Built with Leptos and leptos-next-metadata"</p>
    }
}
```

## 📚 Documentation

- **[Getting Started](book/src/getting-started/installation.md)** - Installation and setup
- **[Guides](book/src/guides/)** - Feature-specific tutorials
- **[API Reference](book/src/reference/api.md)** - Complete API documentation
- **[Cookbook](book/src/cookbook/)** - Real-world examples
- **[Migration](book/src/migration/)** - Migrating from other solutions

## 🏗️ Architecture

leptos-next-metadata is built on five core modules:

| Module | Purpose | Performance |
|--------|---------|-------------|
| **metadata** | Config-based metadata management | <1ms static, <10ms dynamic |
| **og_image** | High-performance image generation | ~100ms generation |
| **json_ld** | Type-safe structured data | Compile-time validation |
| **integrations** | Leptos ecosystem integration | Zero-cost abstractions |
| **conventions** | File-based metadata detection | Build-time processing |

## 📦 Examples

Explore our [examples directory](examples/) for complete applications:

- **[Basic](examples/basic/)** - Simple static site
- **[Blog](examples/blog/)** - Dynamic blog with SEO
- **[E-commerce](examples/ecommerce/)** - Product pages with rich metadata
- **[Advanced SEO](examples/advanced-seo/)** - Complex SEO optimization

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Security Policy](SECURITY.md)
- [Changelog](CHANGELOG.md)

## 📄 License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

## 🎯 Performance Targets

- **OG Image Generation**: 100ms average (vs 800ms browser-based)
- **Metadata Resolution**: <1ms static, <10ms dynamic
- **Build Time**: <5% increase for typical applications
- **Bundle Size**: ~200KB full feature set, tree-shakeable to ~50KB

---

**Built with ❤️ for the Leptos community**