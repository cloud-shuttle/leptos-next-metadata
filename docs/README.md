# 📚 Documentation Overview

Welcome to the `leptos-next-metadata` documentation! This guide will help you navigate through all available resources.

---

## 🚀 **Quick Start**

### **New to the Project?**
1. **[Getting Started](getting-started/)** - Installation and basic setup
2. **[Quick Start Guide](getting-started/quick-start.md)** - Get up and running in minutes
3. **[Basic Examples](examples/basic/)** - Simple usage examples

### **Ready to Build?**
1. **[Components Guide](guides/components.md)** - Learn about all available components
2. **[API Reference](api/)** - Complete API documentation
3. **[Advanced Examples](examples/)** - Complex use cases and patterns

---

## 📖 **Documentation Structure**

### **🚀 Getting Started**
- **[Installation](getting-started/installation.md)** - How to install and configure
- **[Project Setup](getting-started/project-setup.md)** - Setting up your first project
- **[Quick Start](getting-started/quick-start.md)** - Get started in 5 minutes

### **📚 Guides**
- **[Components](guides/components.md)** - Complete component reference
- **[Migration Guide](guides/migration-guide.md)** - Migrating from other solutions
- **[Performance Guide](guides/performance-guide.md)** - Optimization best practices
- **[Troubleshooting](guides/troubleshooting.md)** - Common issues and solutions

### **🔧 API Reference**
- **[Core API](api/core.md)** - Main library functions and types
- **[Macros](api/macros.md)** - Procedural macro documentation

### **💡 Examples**
- **[Basic](examples/basic/)** - Simple usage examples
- **[Advanced SEO](examples/advanced-seo/)** - Complex SEO scenarios
- **[Blog](examples/blog/)** - Blog-specific metadata patterns
- **[E-commerce](examples/ecommerce/)** - E-commerce metadata examples
- **[Custom OG Images](examples/custom-og-images/)** - Dynamic image generation
- **[Internationalization](examples/with-i18n/)** - Multi-language support

### **🛠️ Development**
- **[Design Document](development/design.md)** - Architecture and design decisions
- **[Implementation Plan](development/implementation_plan.md)** - Development roadmap
- **[Testing Strategy](development/testing_strategy.md)** - Testing approach and coverage
- **[Production Roadmap](development/PRODUCTION_ROADMAP.md)** - Release planning
- **[RFCs](development/rfcs/)** - Request for Comments and design proposals

### **👥 Community**
- **[Code of Conduct](community/CODE_OF_CONDUCT.md)** - Community guidelines
- **[Contributing](community/CONTRIBUTING.md)** - How to contribute
- **[Security](community/SECURITY.md)** - Security policies and reporting
- **[Changelog](community/changelog/)** - Release notes and changes

---

## 🎯 **Common Use Cases**

### **Basic Metadata**
```rust
use leptos_next_metadata::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Html lang="en" />
        <Title text="My App" />
        <Meta name="description" content="A great app" />
    }
}
```

### **Advanced SEO**
```rust
use leptos_next_metadata::prelude::*;

#[component]
pub fn BlogPost() -> impl IntoView {
    view! {
        <Html lang="en" />
        <EnhancedTitle 
            text="My Blog Post"
            template="%s | My Blog"
        />
        <MetaTags>
            <Meta name="description" content="An amazing blog post" />
            <Meta property="og:title" content="My Blog Post" />
            <Meta property="og:description" content="An amazing blog post" />
            <Meta property="og:image" content="/images/blog-post.jpg" />
        </MetaTags>
    }
}
```

### **Dynamic OG Images**
```rust
use leptos_next_metadata::prelude::*;

#[component]
pub fn ProductPage() -> impl IntoView {
    let product_title = "Amazing Product";
    let product_price = "$99.99";
    
    view! {
        <Html lang="en" />
        <Title text=product_title />
        <MetaTags>
            <Meta name="description" content="Check out this amazing product" />
            <Meta property="og:title" content=product_title />
            <Meta property="og:image" content=format!("/api/og?title={}&price={}", product_title, product_price) />
        </MetaTags>
    }
}
```

---

## 🔍 **Finding What You Need**

### **By Experience Level**
- **Beginner**: Start with [Getting Started](getting-started/) and [Basic Examples](examples/basic/)
- **Intermediate**: Check out [Components Guide](guides/components.md) and [API Reference](api/)
- **Advanced**: Explore [Advanced Examples](examples/) and [Development Docs](development/)

### **By Use Case**
- **SEO**: [Performance Guide](guides/performance-guide.md) + [Advanced SEO Examples](examples/advanced-seo/)
- **Blog**: [Blog Examples](examples/blog/) + [Components Guide](guides/components.md)
- **E-commerce**: [E-commerce Examples](examples/ecommerce/) + [Migration Guide](guides/migration-guide.md)
- **Multi-language**: [i18n Examples](examples/with-i18n/) + [Troubleshooting](guides/troubleshooting.md)

### **By Problem**
- **Installation Issues**: [Getting Started](getting-started/) + [Troubleshooting](guides/troubleshooting.md)
- **Performance**: [Performance Guide](guides/performance-guide.md) + [Development Docs](development/)
- **Migration**: [Migration Guide](guides/migration-guide.md) + [API Reference](api/)
- **Contributing**: [Community](community/) + [Development Docs](development/)

---

## 🤝 **Getting Help**

### **Documentation Issues**
- Check the [Troubleshooting Guide](guides/troubleshooting.md)
- Review [Common Issues](guides/troubleshooting.md#common-issues)
- Look at [Examples](examples/) for similar use cases

### **Community Support**
- **[GitHub Issues](https://github.com/cloud-shuttle/leptos-next-metadata/issues)** - Bug reports and feature requests
- **[GitHub Discussions](https://github.com/cloud-shuttle/leptos-next-metadata/discussions)** - Questions and community help
- **[Discord](https://discord.gg/leptos)** - Real-time chat and support

### **Contributing**
- **[Contributing Guide](community/CONTRIBUTING.md)** - How to contribute
- **[Code of Conduct](community/CODE_OF_CONDUCT.md)** - Community guidelines
- **[Development Docs](development/)** - Technical implementation details

---

## 📊 **Project Status**

- **Version**: v1.0.0 (Stable)
- **Status**: Production Ready
- **Testing**: 191 comprehensive tests
- **Documentation**: Complete guides and API reference
- **Performance**: 2-7x faster than alternatives

---

**🎯 Ready to get started? [Begin with the Quick Start Guide](getting-started/quick-start.md)!**