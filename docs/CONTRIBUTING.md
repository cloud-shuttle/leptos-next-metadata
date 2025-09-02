# Contributing to leptos-next-metadata

Thank you for your interest in contributing to leptos-next-metadata! This document outlines the process for contributing to the project and helps ensure a smooth collaboration experience.

## 🚀 Getting Started

### Prerequisites

- **Rust**: Latest stable version (1.70+)
- **Node.js**: For building documentation (18+)
- **Git**: For version control

### Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/yourusername/leptos-next-metadata.git
   cd leptos-next-metadata
   ```

2. **Install Dependencies**
   ```bash
   cargo build
   cargo install mdbook  # For documentation
   ```

3. **Run Tests**
   ```bash
   cargo test --all-features
   cargo bench --no-run  # Verify benchmarks compile
   ```

4. **Documentation**
   ```bash
   cd docs/book
   mdbook serve  # Serve documentation locally
   ```

## 🎯 Ways to Contribute

### Bug Reports
- Use the [bug report template](https://github.com/yourusername/leptos-next-metadata/issues/new?template=bug_report.md)
- Include minimal reproduction steps
- Provide environment details (OS, Rust version, Leptos version)

### Feature Requests
- Use the [feature request template](https://github.com/yourusername/leptos-next-metadata/issues/new?template=feature_request.md)
- Explain the use case and expected behavior
- Consider starting a discussion first for major features

### Code Contributions
- Start with issues labeled `good-first-issue` or `help-wanted`
- Discuss major changes in an issue first
- Follow our coding standards and testing requirements

## 📋 Development Guidelines

### Code Standards

1. **Formatting**
   ```bash
   cargo fmt --all
   ```

2. **Linting**
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```

3. **Testing**
   ```bash
   cargo test --all-features
   cargo test --doc  # Test documentation examples
   ```

### Commit Messages

We follow [Conventional Commits](https://conventionalcommits.org/):

```
feat: add support for custom OG image templates
fix: resolve metadata inheritance bug in nested routes
docs: update installation guide with feature flags
perf: optimize image generation caching layer
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`

### Branch Naming

- `feature/description` - New features
- `fix/description` - Bug fixes  
- `docs/description` - Documentation changes
- `refactor/description` - Code improvements

### Testing Requirements

All contributions must include appropriate tests:

1. **Unit Tests** - For individual functions and modules
   ```rust
   #[test]
   fn test_metadata_merge() {
       // Test implementation
   }
   ```

2. **Integration Tests** - For component interactions
   ```rust
   #[tokio::test]
   async fn test_og_image_generation() {
       // Test implementation
   }
   ```

3. **Documentation Tests** - Examples in docs must work
   ```rust
   /// # Example
   /// ```
   /// use leptos_next_metadata::prelude::*;
   /// // Working example code
   /// ```
   ```

### Performance Considerations

- **Benchmarks**: Include benchmarks for performance-critical changes
- **Memory Usage**: Profile memory usage for large operations
- **Build Times**: Avoid increasing compilation time significantly
- **Bundle Size**: Consider impact on final bundle size

## 📊 Pull Request Process

### Before Submitting

1. **Rebase** your branch on latest main
2. **Run** full test suite locally
3. **Update** documentation if needed
4. **Add** changelog entry if applicable

### PR Checklist

- [ ] Tests pass locally
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated
- [ ] Changelog entry added (if applicable)
- [ ] Benchmarks included (for performance changes)

### PR Template

```markdown
## Summary
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update
- [ ] Performance improvement

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed

## Performance Impact
- Benchmarks: [link to benchmark results]
- Bundle size: [impact on bundle size]
- Memory usage: [memory usage impact]

## Breaking Changes
List any breaking changes and migration steps

## Additional Notes
Any additional context or considerations
```

## 🏗️ Project Structure

```
leptos-next-metadata/
├── src/                    # Main library code
├── macros/                 # Procedural macros
├── tests/                  # Integration tests
├── benches/                # Performance benchmarks
├── examples/               # Example applications
├── docs/                   # Documentation
│   ├── book/              # mdBook documentation
│   └── rfcs/              # Design RFCs
└── .github/               # GitHub workflows
```

### Module Guidelines

- **Public API**: Keep API surface minimal and focused
- **Error Handling**: Use `thiserror` for structured errors
- **Async Code**: Support both sync and async where beneficial
- **Feature Flags**: Use features to make functionality optional

## 📚 Documentation Standards

### Code Documentation

```rust
/// Brief one-line description
///
/// Longer description explaining the purpose and behavior.
/// Include important details about parameters and return values.
///
/// # Examples
///
/// ```rust
/// use leptos_next_metadata::prelude::*;
/// 
/// let metadata = Metadata::builder()
///     .title("My Page")
///     .build();
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// - Invalid metadata format provided
/// - Required fields are missing
///
/// # Panics
///
/// This function panics if called without proper context setup.
pub fn example_function() -> Result<Metadata, Error> {
    // Implementation
}
```

### Documentation Writing

- Use clear, concise language
- Include practical examples
- Test all code examples
- Update related documentation when making changes

## 🔍 Code Review Process

### As a Reviewer

- **Focus on**: Logic, performance, maintainability, documentation
- **Be constructive**: Provide specific suggestions for improvements  
- **Ask questions**: If something isn't clear, ask for clarification
- **Test locally**: Run tests and try examples when possible

### As an Author

- **Be responsive**: Address feedback promptly
- **Explain decisions**: If you disagree, explain your reasoning
- **Update docs**: Keep documentation in sync with code changes
- **Test thoroughly**: Ensure your changes work as intended

## 🌟 Recognition

Contributors are recognized in several ways:

- **Authors file**: Added to AUTHORS.md
- **Changelog**: Contributions noted in release notes
- **GitHub**: Recognition in release announcements
- **Documentation**: Contributors acknowledged in docs

## 🆘 Getting Help

- **Discord**: Join our development chat
- **GitHub Discussions**: For design discussions
- **Issues**: For bugs and feature requests
- **Email**: maintainer@leptos-next-metadata.dev

## 📜 License

By contributing to leptos-next-metadata, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).

---

Thank you for contributing to leptos-next-metadata! 🎉