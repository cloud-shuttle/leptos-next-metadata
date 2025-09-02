# leptos-next-metadata - Project Context

> **Navigation**: [📚 Documentation Index](docs/index.md) | [📋 Design Document](docs/design.md) | [🗓️ Implementation Plan](docs/implementation_plan.md)

## Overview

This project is developing `leptos-next-metadata`, a comprehensive Rust crate that brings Next.js-style metadata management to Leptos v0.8.8. The library addresses a critical gap in the Leptos ecosystem by providing production-ready metadata management with type-safe APIs, high-performance OG image generation, and seamless SSR/CSR context handling.

## Core Features

- **Config-based Metadata**: Static and dynamic metadata generation with type safety
- **File-based Conventions**: Next.js-compatible file conventions for metadata
- **OG Image Generation**: 2-7x faster image generation using resvg + tiny-skia
- **JSON-LD Support**: Type-safe structured data with Schema.org compliance
- **SSR/CSR Context Handling**: Automatic optimization for different rendering modes
- **SEO Best Practices**: Built-in validation and optimization recommendations

## Architecture

The library follows a modular, trait-based architecture with five primary modules:

1. **metadata**: Config-based and file-based metadata management
2. **og_image**: High-performance image generation engine
3. **json_ld**: Type-safe Schema.org structured data
4. **integrations**: Leptos ecosystem integration
5. **conventions**: File system scanning and convention resolution

## Key Performance Targets

- **OG Image Generation**: 100ms average (vs 800ms browser-based)
- **Metadata Resolution**: <1ms static, <10ms dynamic
- **Build Time Impact**: <5% increase
- **Runtime Memory**: <1MB overhead
- **Bundle Size**: ~200KB full, tree-shakeable to ~50KB

## Development Status

The project is currently in the design and planning phase, with comprehensive documentation outlining the implementation strategy and technical approach.

## Command Context

When working with this project:
- Focus on Rust/WebAssembly ecosystem patterns
- Prioritize type safety and compile-time guarantees
- Consider both SSR and CSR performance implications
- Follow Leptos v0.8.8 reactive patterns
- Maintain compatibility with Next.js metadata conventions

## Documentation Structure

- `docs/design.md`: Complete technical design and architecture
- `docs/implementation_plan.md`: Detailed 30-day implementation roadmap
- This file serves as the Claude context summary

## Development Commands

Common commands for this project:
- `cargo build`: Build the library
- `cargo test`: Run all tests
- `cargo bench`: Run performance benchmarks
- `cargo doc --open`: Generate and view documentation
- `cargo check --all-features`: Check all feature combinations

## Key Dependencies

- **leptos**: v0.8.8 (reactive web framework)
- **resvg/tiny-skia**: SVG rendering for OG images
- **serde**: Serialization for metadata structures
- **json-ld**: W3C-compliant JSON-LD processing
- **liquid**: Template engine for dynamic content
- **tokio**: Async runtime for server operations