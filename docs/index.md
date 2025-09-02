# leptos-next-metadata Documentation Index

## Quick Start

- **[claude.md](../claude.md)** - Project context and overview for Claude Code assistance
- **[Design Document](design.md)** - Complete technical architecture and design decisions
- **[Implementation Plan](implementation_plan.md)** - Detailed 30-day development roadmap

## Project Overview

`leptos-next-metadata` is a comprehensive Rust crate bringing Next.js-style metadata management to Leptos v0.8.8. It provides type-safe metadata APIs, high-performance OG image generation (2-7x faster), and seamless SSR/CSR handling.

## Core Documentation

### Technical Design
- **[Architecture Overview](design.md#architecture-overview)** - System design and module structure
- **[Config-based Metadata](design.md#config-based-metadata-system)** - Static and dynamic metadata implementation
- **[OG Image Generation](design.md#open-graph-image-generation)** - High-performance image rendering engine
- **[File Conventions](design.md#file-based-metadata-conventions)** - Next.js-compatible file scanning
- **[JSON-LD Support](design.md#json-ld-and-structured-data)** - Type-safe structured data
- **[SSR/CSR Context](design.md#ssrcsr-context-handling)** - Context-aware optimization

### Implementation Details
- **[Project Setup](implementation_plan.md#project-setup-and-initial-structure)** - Workspace configuration and dependencies
- **[Core Modules](implementation_plan.md#core-module-implementation)** - Metadata types and traits
- **[Image Generation](implementation_plan.md#og-image-generation-implementation)** - SVG rendering and templates
- **[Convention Scanner](implementation_plan.md#file-convention-scanner)** - File system scanning
- **[Testing Strategy](implementation_plan.md#testing-strategy)** - Unit and integration tests
- **[Performance Benchmarks](implementation_plan.md#benchmarks-and-performance-testing)** - Optimization targets

## Key Features

### ⚡ Performance
- **100ms** average OG image generation (vs 800ms browser-based)
- **<1ms** static metadata resolution
- **<10ms** dynamic metadata with async data
- **<5%** build time increase
- **~200KB** bundle size (tree-shakeable to 50KB)

### 🔒 Type Safety  
- Compile-time metadata validation
- Type-safe JSON-LD with Schema.org support
- Rust's memory safety guarantees
- Zero-cost abstractions for static metadata

### 🎯 Developer Experience
- Next.js-compatible API surface
- Automatic SSR/CSR optimization
- File-based convention support
- Built-in SEO validation

## Development Timeline

| Week | Focus | Key Deliverables |
|------|-------|------------------|
| 1 | Foundation | Core types, basic metadata, project setup |
| 2 | Core Features | Macro implementation, dynamic metadata |
| 3 | Image Generation | OG image core, SVG templates, caching |
| 4 | Conventions | File scanning, JSON-LD, structured data |
| 5 | Context Handling | SSR/CSR optimization, islands support |
| 6 | Polish | Testing, documentation, release prep |

## Quick Reference

### Essential Commands
```bash
cargo build                    # Build the library
cargo test --all-features     # Run comprehensive tests
cargo bench                   # Performance benchmarks
cargo doc --open             # Generate documentation
```

### Key Dependencies
- `leptos = "0.8.8"`           # Reactive web framework
- `resvg` + `tiny-skia`        # High-performance SVG rendering
- `json-ld = "0.17"`          # W3C-compliant JSON-LD
- `liquid = "0.26"`           # Template engine
- `serde = "1.0"`             # Serialization framework

### Feature Flags
- `default = ["ssr", "og-images", "file-conventions"]`
- `ssr` - Server-side rendering support
- `csr` - Client-side rendering only
- `og-images` - Image generation capabilities
- `file-conventions` - File-based metadata scanning

## Architecture at a Glance

```
leptos-next-metadata/
├── metadata/          # Config-based metadata system
│   ├── config/        # Static & dynamic generation
│   ├── merge/         # Inheritance & merging
│   └── validation/    # SEO best practices
├── og_image/          # Image generation engine
│   ├── generator/     # SVG → PNG rendering
│   ├── templates/     # Liquid template system
│   └── cache/         # Multi-level caching
├── json_ld/           # Structured data support
│   ├── schema/        # Schema.org types
│   └── builder/       # Type-safe builders
├── integrations/      # Ecosystem integration
│   ├── leptos_meta/   # leptos_meta bridge
│   └── server_fn/     # Server function helpers
└── conventions/       # File-based conventions
    ├── scanner/       # File system scanning
    └── resolver/      # Convention resolution
```

## Next Steps

1. **Start with**: [Design Document](design.md) for complete technical understanding
2. **Then review**: [Implementation Plan](implementation_plan.md) for development approach
3. **Use**: [claude.md](../claude.md) when working with Claude Code assistance

---

*This documentation index provides navigation for the leptos-next-metadata project. For specific implementation details, refer to the individual documents linked above.*