# Release Notes - v0.1.0-alpha.1

## 🎉 First Alpha Release

**Release Date:** September 3, 2025  
**Version:** 0.1.0-alpha.1  
**Status:** Alpha Release - Ready for Early Adopters

## ✨ What's New

### Core Features
- **Metadata System**: Complete metadata structures and types for Leptos applications
- **Procedural Macros**: `metadata!` and `generate_metadata!` macros for easy metadata management
- **OpenGraph Support**: Basic OpenGraph and Twitter Card metadata generation
- **JSON-LD Foundation**: Structured data support with conditional compilation
- **File Conventions**: Automatic detection of favicons, manifests, and other web assets

### Developer Experience
- **Type Safety**: Full Rust type safety with comprehensive error handling
- **Builder Pattern**: Fluent API for constructing metadata objects
- **Validation**: Built-in metadata validation with scoring system
- **Integration**: Seamless integration with Leptos 0.8+ signal system

## 🧪 Testing & Quality

- **Unit Tests**: 97 tests passing ✅
- **Documentation Tests**: 4 tests passing ✅
- **E2E Tests**: Cross-browser testing with Playwright ✅
- **Test Server**: Working HTTP server for testing ✅
- **Code Coverage**: Comprehensive test coverage across all modules

## 🚀 Getting Started

### Installation

```toml
[dependencies]
leptos-next-metadata = "0.1.0-alpha.1"
leptos-next-metadata-macros = "0.1.0-alpha.1"
```

### Basic Usage

```rust
use leptos::*;
use leptos_next_metadata::prelude::*;
use leptos_next_metadata_macros::metadata;

#[component]
fn MyPage() -> impl IntoView {
    metadata! {
        title: "My Page",
        description: "Page description",
        keywords: ["rust", "leptos", "metadata"],
        og_type: "website",
    }
    
    view! { <div>"Hello World"</div> }
}
```

### Dynamic Metadata

```rust
use leptos_next_metadata_macros::generate_metadata;

#[generate_metadata]
fn get_metadata() -> Metadata {
    Metadata {
        title: Some(Title::Static("Dynamic Page".into())),
        description: Some("Generated description".into()),
        ..Default::default()
    }
}
```

## 🔧 Features

### Metadata Types
- **Title**: Static and dynamic title support
- **Description**: SEO-optimized descriptions
- **Keywords**: Flexible keyword management
- **Authors**: Author information and profiles
- **OpenGraph**: Social media optimization
- **Twitter Cards**: Twitter-specific metadata
- **Robots**: Search engine directives
- **Viewport**: Mobile optimization settings

### JSON-LD Support
- **Schema.org**: Industry-standard structured data
- **Article**: Blog post and article markup
- **Organization**: Company and business information
- **Person**: Individual profile data
- **WebPage**: Page-specific structured data
- **FAQPage**: Question and answer markup
- **BreadcrumbList**: Navigation structure

### File Conventions
- **Favicon Detection**: Automatic favicon.ico and apple-touch-icon detection
- **Manifest Support**: Web app manifest file detection
- **Icon Management**: Multiple icon format support
- **Robots.txt**: Search engine configuration
- **Sitemap**: XML sitemap detection

## ⚠️ Known Limitations

### Alpha Release Constraints
- **JSON-LD Field Naming**: Some fields use camelCase instead of snake_case (14 warnings)
- **OG Image Generation**: Basic pipeline implemented, advanced features in progress
- **Caching**: Memory caching implemented, disk caching in progress
- **Performance**: Not yet optimized for production workloads

### Browser Support
- **Modern Browsers**: Chrome 90+, Firefox 88+, Safari 14+, Edge 90+
- **Mobile**: iOS Safari 14+, Chrome Mobile 90+
- **Legacy**: Limited support for older browsers

## 🛠️ Development Status

### Completed (95%)
- ✅ Core metadata structures and types
- ✅ Procedural macro system
- ✅ Basic OpenGraph and Twitter Card support
- ✅ JSON-LD foundation
- ✅ File convention scanning
- ✅ Comprehensive testing suite
- ✅ Cross-browser compatibility

### In Progress (5%)
- 🔄 OG image generation pipeline
- 🔄 Advanced caching strategies
- 🔄 Performance optimizations

### Planned for Beta
- 📋 Performance benchmarking
- 📋 Migration tools from Next.js
- 📋 Advanced template system
- 📋 Complete documentation book

## 🔮 Roadmap

### Beta Release (Q1 2026)
- Performance optimizations
- Advanced OG image generation
- Complete caching system
- Migration tools

### 1.0 Release (Q2 2026)
- Production-ready performance
- Complete feature set
- Comprehensive documentation
- Migration guides

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Areas for Contribution
- Performance optimization
- Additional JSON-LD schemas
- Browser compatibility improvements
- Documentation and examples

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/your-repo/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-repo/discussions)
- **Documentation**: [Book](https://your-docs-site.com)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Note**: This is an alpha release intended for early adopters and developers. The API may change before the beta release. Please report any issues or feedback to help us improve the library.
