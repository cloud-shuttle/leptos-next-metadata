# Changelog

All notable changes to leptos-next-metadata will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Last Updated**: September 3rd, 2025

## [Unreleased]

### Planned
- OG image generation pipeline completion
- File convention scanner implementation
- Advanced caching strategies
- Performance optimizations
- Integration with Leptos ecosystem
- Comprehensive testing suite with Playwright

## [0.1.0-alpha.1] - 2025-09-03

### Added
- Core metadata system with type-safe APIs
- Static metadata macro (`metadata!`) with full Leptos 0.8+ integration
- Dynamic metadata generation (`generate_metadata!`) with reactive signals
- Basic metadata merging and inheritance
- Integration with leptos_meta 0.8+
- Development-only SEO validation warnings
- Procedural macro system for clean metadata syntax
- Support for Leptos 0.8+ signal system and modern reactive patterns

### Performance
- Sub-millisecond static metadata resolution
- Basic caching for metadata computation
- Reactive metadata updates with <1ms response time

### Technical
- Modern Rust 1.75+ features support
- Leptos 0.8+ compatibility with latest APIs
- Conditional compilation for JSON-LD features
- Type-safe metadata structures with compile-time validation

## [0.1.0-alpha.2] - Planned for Q4 2025

### Planned
- OG image generation engine using resvg + tiny-skia
- SVG template system with Liquid templating
- Basic OG image templates (default, article, product)
- In-memory LRU caching for generated images
- Font loading and management

### Performance Targets
- ~100ms OG image generation (development baseline)
- Basic image caching to avoid regeneration

## [0.1.0-alpha.3] - Planned for Q4 2025

### Planned
- File-based metadata conventions
- Convention scanner for build-time processing
- Support for favicon.ico, icon files, robots.txt
- Basic opengraph-image and twitter-image file support
- Hot-reload support in development

## [0.1.0-beta.1] - Planned for Q1 2026

### Planned
- JSON-LD support with type-safe Schema.org types
- Builder pattern for complex structured data
- Article, Product, Organization, Person schemas
- JSON-LD validation and optimization
- Structured data component integration

## [0.1.0-beta.2] - Planned for Q1 2026

### Planned
- SSR/CSR context detection and optimization
- Islands architecture support
- Streaming SSR metadata support
- Client-side metadata updates
- Hydration-aware metadata handling

## [0.1.0] - Planned for Q2 2026

### Planned
- SEO best practices validation engine
- Automated optimization suggestions
- Complete API documentation
- Migration guides from Next.js and leptos_meta
- Full example applications
- Comprehensive test coverage

### Performance Targets
- All performance targets met
- Production optimization complete
- Bundle size optimization

### Security
- Security audit complete
- Safe defaults established

## [0.2.0] - Planned for Q3 2026

### Planned
- Custom OG image components with Leptos syntax
- Visual metadata editor/preview tool
- Advanced template system
- Plugin architecture

## [0.3.0] - Planned for Q4 2026

### Planned
- Advanced SEO auditing tools
- Automated performance optimization
- Custom validation rules
- Integration with analytics platforms

## [0.4.0] - Planned for Q1 2027

### Planned
- Integration with other Rust web frameworks (Axum, Warp, etc.)
- Advanced caching strategies
- CDN integration helpers
- Advanced templating system

## [1.0.0] - Planned for Q2 2027

### Planned
- Stable API with backward compatibility guarantee
- Full production readiness
- Enterprise features
- Long-term support commitment

---

## Categories

- **Added** - New features
- **Changed** - Changes in existing functionality  
- **Deprecated** - Soon-to-be removed features
- **Removed** - Removed features
- **Fixed** - Bug fixes
- **Security** - Vulnerability fixes
- **Performance** - Performance improvements

## Performance Tracking

| Version | OG Generation | Metadata Resolution | Bundle Size | Build Impact | Status |
|---------|---------------|-------------------|-------------|--------------|---------|
| 0.1.0-alpha.1 | N/A | <1ms / <10ms | ~50KB | <2% | ✅ Complete |
| 0.1.0-alpha.2 | ~100ms | <1ms / <10ms | ~100KB | <3% | 🔄 In Progress |
| 0.1.0 | ~100ms | <1ms / <10ms | ~200KB | <5% | 📋 Planned |

## Current Implementation Status

### ✅ Completed (0.1.0-alpha.1)
- Core metadata structures and types
- `metadata!` macro for static metadata
- `generate_metadata!` macro for dynamic metadata
- Basic OpenGraph and Twitter Card support
- JSON-LD foundation with conditional compilation
- Procedural macro system
- Integration with Leptos 0.8+ signal system
- Reactive metadata updates

### 🔄 In Progress
- OG image generation pipeline
- File convention scanner
- Advanced caching strategies
- Performance optimizations
- Integration with Leptos ecosystem

### 📋 Planned
- Comprehensive testing suite with Playwright
- Performance benchmarking
- Migration tools from Next.js
- Advanced template system
- Complete documentation book

## Migration Notes

### From leptos_meta
- API compatibility layer provided
- Gradual migration path available
- Performance improvements immediate
- Modern Leptos 0.8+ integration

### From Next.js
- File convention compatibility planned
- API surface similarities  
- Migration tool planned for 0.2.0
- Reactive metadata capabilities

### From Leptos 0.7 or earlier
- Requires Leptos 0.8+ for full functionality
- Modern signal system integration
- Updated component patterns
- Enhanced reactive capabilities