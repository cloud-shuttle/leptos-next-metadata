# Changelog

All notable changes to leptos-next-metadata will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure and design documentation
- Comprehensive documentation framework
- Development setup and contribution guidelines

## [0.1.0-alpha.1] - TBD

### Added
- Core metadata system with type-safe APIs
- Static metadata macro (`metadata!`)
- Dynamic metadata generation (`generate_metadata!`)
- Basic metadata merging and inheritance
- Integration with leptos_meta
- Development-only SEO validation warnings

### Performance
- Sub-millisecond static metadata resolution
- Basic caching for metadata computation

## [0.1.0-alpha.2] - TBD

### Added
- OG image generation engine using resvg + tiny-skia
- SVG template system with Liquid templating
- Basic OG image templates (default, article, product)
- In-memory LRU caching for generated images
- Font loading and management

### Performance
- ~100ms OG image generation (development baseline)
- Basic image caching to avoid regeneration

## [0.1.0-alpha.3] - TBD

### Added
- File-based metadata conventions
- Convention scanner for build-time processing
- Support for favicon.ico, icon files, robots.txt
- Basic opengraph-image and twitter-image file support
- Hot-reload support in development

### Changed
- Improved error messages for metadata validation
- Enhanced development-time warnings

## [0.1.0-beta.1] - TBD

### Added
- JSON-LD support with type-safe Schema.org types
- Builder pattern for complex structured data
- Article, Product, Organization, Person schemas
- JSON-LD validation and optimization
- Structured data component (`<JsonLd>`)

### Changed
- Metadata merging now follows Next.js shallow merge semantics
- Improved TypeScript-like error messages

### Performance
- Compile-time JSON-LD validation
- Zero-runtime-cost for static JSON-LD

## [0.1.0-beta.2] - TBD

### Added
- SSR/CSR context detection and optimization
- Islands architecture support
- Streaming SSR metadata support
- Client-side metadata updates
- Hydration-aware metadata handling

### Changed
- Context-aware metadata resolution
- Optimized for different Leptos rendering modes

### Performance
- Different caching strategies for SSR vs CSR
- Minimal hydration overhead

## [0.1.0] - TBD

### Added
- SEO best practices validation engine
- Automated optimization suggestions
- Complete API documentation
- Migration guides from Next.js and leptos_meta
- Full example applications
- Comprehensive test coverage

### Changed
- Stable API surface
- Production-ready error handling
- Complete documentation

### Performance
- All performance targets met
- Production optimization complete
- Bundle size optimization

### Security
- Security audit complete
- Safe defaults established

## [0.2.0] - TBD (Future)

### Planned
- Custom OG image components with Leptos syntax
- Visual metadata editor/preview tool
- Advanced template system
- Plugin architecture

## [0.3.0] - TBD (Future)

### Planned
- Advanced SEO auditing tools
- Automated performance optimization
- Custom validation rules
- Integration with analytics platforms

## [0.4.0] - TBD (Future)

### Planned
- Integration with other Rust web frameworks (Axum, Warp, etc.)
- Advanced caching strategies
- CDN integration helpers
- Advanced templating system

## [1.0.0] - TBD (Future)

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

| Version | OG Generation | Metadata Resolution | Bundle Size | Build Impact |
|---------|---------------|-------------------|-------------|--------------|
| 0.1.0   | ~100ms        | <1ms / <10ms      | ~200KB      | <5%         |

## Migration Notes

### From leptos_meta
- API compatibility layer provided
- Gradual migration path available
- Performance improvements immediate

### From Next.js
- File convention compatibility
- API surface similarities  
- Migration tool planned for 0.2.0