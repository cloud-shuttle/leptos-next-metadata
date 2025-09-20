# leptos-next-metadata Agent Guidelines

## Build & Test Commands

- **Build**: `cargo build --all-features` or `make build`
- **Test All**: `cargo test --all-features` or `make test`
- **Test Single**: `cargo test <test_name> --all-features`
- **E2E Tests**: `pnpm test:e2e` or `make e2e` (requires Node.js/Playwright)
- **Quick Check**: `cargo check --all-features` or `make quick-check`
- **Format**: `cargo fmt --all` or `make fmt`
- **Lint**: `cargo clippy --all-features -- -D warnings` or `make clippy`
- **Coverage**: `make coverage` (generates HTML report in target/coverage/)
- **Bench**: `cargo bench --all-features` or `make bench`
- **CI Pipeline**: `make ci` (runs check, test, test-features, audit)

## Architecture & Structure

- **Core Library**: Leptos 0.8+ metadata management with Next.js-style APIs
- **Main Modules**: metadata/, og_image/, json_ld/, conventions/, macros/, utils/
- **Features**: SSR/CSR/hydrate support, OG image generation, JSON-LD, file conventions
- **Macros**: Procedural macros in macros/ subproject for compile-time metadata validation
- **Testing**: Unit tests, integration tests, E2E with Playwright, benchmarks,
  visual regression

## Code Style & Conventions

- **Format**: rustfmt with max_width=100, reorder_imports=true, use_field_init_shorthand=true
- **Lint**: Clippy with -D warnings, cognitive-complexity ≤30, too-many-args ≤8
- **Imports**: Organized with reorder_imports, use prelude for common types
- **Error Handling**: Use thiserror for custom errors, anyhow for context chains
- **Async**: Use async-trait, tokio runtime, prefer ? operator over unwrap()
- **Types**: Strong typing with serde derives, optional fields with Option&lt;T&gt;
- **Documentation**: Module-level docs with //!, examples in doc comments
- **Naming**: snake_case files/modules, PascalCase types, SCREAMING_SNAKE_CASE constants
