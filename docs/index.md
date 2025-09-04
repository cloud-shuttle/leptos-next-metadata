# 📚 leptos-next-metadata Documentation

> **Navigation**: [🏠 Home](../../README.md) | [📦 crates.io](https://crates.io/crates/leptos-next-metadata) | [🐙 GitHub](https://github.com/cloud-shuttle/leptos-next-metadata)

## 🎯 **Quick Start**

```bash
cargo add leptos-next-metadata
```

```rust
use leptos::*;
use leptos_next_metadata::prelude::*;

#[component]
fn MyPage() -> impl IntoView {
    metadata! {
        title: "My Page",
        description: "Page description",
        og_image: "/og-image.jpg",
    }
    
    view! { <div>"My Page"</div> }
}
```

---

## 📖 **Documentation Sections**

### 🚀 **Getting Started**
- **[Quick Start Guide](guides/getting-started.md)** - Get up and running in minutes
- **[Installation](guides/installation.md)** - Detailed setup instructions
- **[Basic Examples](examples/basic/README.md)** - Simple use cases to get started

### 📚 **Guides & Tutorials**
- **[Design Document](guides/design.md)** - Architecture and design decisions
- **[Implementation Plan](guides/implementation_plan.md)** - Technical implementation details
- **[Testing Strategy](guides/testing_strategy.md)** - Testing approach and coverage
- **[Production Roadmap](guides/PRODUCTION_ROADMAP.md)** - Path to v1.0.0 stable release

### 🔧 **API Reference**
- **[Core API](api/core.md)** - Main metadata structures and types
- **[Macros API](api/macros.md)** - Procedural macro documentation
- **[JSON-LD API](api/json-ld.md)** - Structured data generation
- **[OG Image API](api/og-image.md)** - Open Graph image generation
- **[File Conventions](api/conventions.md)** - Asset detection and management

### 📝 **Examples & Use Cases**
- **[Basic Examples](examples/basic/README.md)** - Simple metadata setup
- **[Blog Examples](examples/blog/README.md)** - Blog and article metadata
- **[E-commerce Examples](examples/ecommerce/README.md)** - Product and shop metadata
- **[Internationalization](examples/with-i18n/README.md)** - Multi-language support
- **[Advanced SEO](examples/advanced-seo/README.md)** - Complex SEO scenarios
- **[Custom OG Images](examples/custom-og-images/README.md)** - Custom image generation

### 🧪 **Testing & Quality**
- **[Testing Strategy](guides/testing_strategy.md)** - Comprehensive testing approach
- **[Test Coverage](testing/coverage.md)** - Current test coverage status
- **[Performance Benchmarks](testing/benchmarks.md)** - Performance metrics and targets
- **[E2E Testing](testing/e2e/README.md)** - End-to-end testing with Playwright

### 📋 **Project Information**
- **[Contributing Guide](../../CONTRIBUTING.md)** - How to contribute to the project
- **[Code of Conduct](../../CODE_OF_CONDUCT.md)** - Community guidelines
- **[Security Policy](../../SECURITY.md)** - Security reporting and policies
- **[License](../../LICENSE)** - MIT OR Apache-2.0 license

---

## 🗺️ **Current Status**

### **Release Status**: 🚀 **Beta Release v0.1.0-beta.1**
- **Version**: 0.1.0-beta.1
- **Status**: Feature Complete - Ready for Production Testing
- **Published**: ✅ GitHub & crates.io
- **Next Target**: v1.0.0 Stable Release

### **Feature Completeness**: 100% ✅
- ✅ Core metadata system
- ✅ Procedural macros
- ✅ JSON-LD support
- ✅ OG image generation
- ✅ File conventions
- ✅ Advanced caching
- ✅ Performance optimization
- ✅ Comprehensive testing

### **Documentation Status**: 85% 📚
- ✅ Core API documentation
- ✅ Examples and tutorials
- ✅ Testing strategy
- ✅ Design documentation
- 🔄 Production roadmap (in progress)
- 🔄 Migration guides (planned)
- 🔄 Performance guides (planned)

---

## 🎯 **Next Steps**

### **Immediate (This Week)**
1. **Documentation Organization** ✅ Complete
2. **Production Roadmap** ✅ Complete
3. **API Documentation Review** 🔄 In Progress

### **Short Term (Next 2 Weeks)**
1. **Complete API Documentation**
2. **Write Migration Guide**
3. **Create Performance Guide**
4. **Establish CI/CD Pipeline**

### **Medium Term (Next Month)**
1. **Production Readiness Review**
2. **Security Audit**
3. **Performance Optimization**
4. **v1.0.0 Release Preparation**

---

## 🔗 **External Resources**

### **Official Documentation**
- **[Rust Book](https://doc.rust-lang.org/book/)** - Learn Rust
- **[Leptos Book](https://leptos.dev/book/)** - Leptos framework documentation
- **[Cargo Book](https://doc.rust-lang.org/cargo/)** - Rust package manager

### **Community Resources**
- **[Rust Community](https://www.rust-lang.org/community)** - Official Rust community
- **[Leptos Discord](https://discord.gg/leptos)** - Leptos community chat
- **[Rust Users Forum](https://users.rust-lang.org/)** - Rust discussion forum

### **Related Projects**
- **[Next.js Metadata](https://nextjs.org/docs/app/building-your-application/optimizing/metadata)** - Next.js metadata system
- **[Schema.org](https://schema.org/)** - Structured data standards
- **[Open Graph Protocol](https://ogp.me/)** - Social media metadata

---

## 📞 **Getting Help**

### **Documentation Issues**
- [Create a documentation issue](https://github.com/cloud-shuttle/leptos-next-metadata/issues/new?template=documentation.md)
- [Suggest improvements](https://github.com/cloud-shuttle/leptos-next-metadata/discussions)

### **Code Issues**
- [Report a bug](https://github.com/cloud-shuttle/leptos-next-metadata/issues/new?template=bug_report.md)
- [Request a feature](https://github.com/cloud-shuttle/leptos-next-metadata/issues/new?template=feature_request.md)

### **Community Support**
- [GitHub Discussions](https://github.com/cloud-shuttle/leptos-next-metadata/discussions)
- [Stack Overflow](https://stackoverflow.com/questions/tagged/leptos-next-metadata)

---

## 📊 **Documentation Metrics**

- **Total Pages**: 25+
- **Code Examples**: 50+
- **API Endpoints**: 100% documented
- **Test Coverage**: 93 unit tests + 4 doc tests
- **Performance Targets**: All met ✅

---

## 🎉 **Contributing to Documentation**

We welcome contributions to improve our documentation! See our [Contributing Guide](../../CONTRIBUTING.md) for details on:

- Writing new guides and tutorials
- Improving existing documentation
- Adding code examples
- Translating documentation
- Reviewing and editing content

---

*Last Updated: September 4, 2025*  
*Documentation Version: 2.0*  
*Next Review: Weekly during roadmap execution*