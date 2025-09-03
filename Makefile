# Makefile for leptos-next-metadata development

# Default Rust toolchain
RUST_VERSION ?= stable

# Test configuration
TEST_FEATURES ?= --all-features
TEST_TIMEOUT ?= 300

# Build configuration  
BUILD_FEATURES ?= --all-features
BUILD_MODE ?= debug

# Coverage configuration
COVERAGE_OUTPUT ?= html

.PHONY: help
help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# Development Commands

.PHONY: dev
dev: ## Set up development environment
	@echo "Setting up development environment..."
	rustup toolchain install $(RUST_VERSION)
	rustup component add rustfmt clippy llvm-tools-preview
	cargo install cargo-tarpaulin cargo-criterion divan-cli cargo-audit
	@echo "✅ Development environment ready!"

.PHONY: check
check: ## Run basic checks (format, clippy, test)
	@echo "🔍 Running basic checks..."
	cargo fmt --all -- --check
	cargo clippy $(BUILD_FEATURES) -- -D warnings
	cargo check $(BUILD_FEATURES)
	@echo "✅ Basic checks passed!"

.PHONY: fmt
fmt: ## Format code
	cargo fmt --all

.PHONY: clippy
clippy: ## Run Clippy linter
	cargo clippy $(BUILD_FEATURES) -- -D warnings

.PHONY: fix
fix: ## Fix common issues automatically
	cargo fmt --all
	cargo clippy $(BUILD_FEATURES) --fix --allow-staged --allow-dirty

# Testing Commands

.PHONY: test
test: ## Run all tests
	@echo "🧪 Running all tests..."
	cargo test $(TEST_FEATURES)
	@echo "✅ All tests passed!"

.PHONY: test-unit
test-unit: ## Run unit tests only
	@echo "🧪 Running unit tests..."
	cargo test --lib $(TEST_FEATURES)

.PHONY: test-integration
test-integration: ## Run integration tests only
	@echo "🧪 Running integration tests..."
	cargo test --test '*' $(TEST_FEATURES)

.PHONY: test-doc
test-doc: ## Run documentation tests
	@echo "🧪 Running documentation tests..."
	cargo test --doc $(TEST_FEATURES)

.PHONY: test-examples
test-examples: ## Test all examples
	@echo "🧪 Testing examples..."
	@if [ -d "examples" ]; then \
		for example in examples/*/; do \
			if [ -f "$$example/Cargo.toml" ]; then \
				echo "Testing $$example"; \
				cargo check --manifest-path "$$example/Cargo.toml"; \
			fi; \
		done; \
	else \
		echo "No examples directory found"; \
	fi

.PHONY: test-watch
test-watch: ## Run tests in watch mode
	cargo watch -x 'test $(TEST_FEATURES)'

.PHONY: test-features
test-features: ## Test different feature combinations
	@echo "🧪 Testing feature combinations..."
	cargo test --no-default-features
	cargo test --no-default-features --features ssr
	cargo test --no-default-features --features csr
	cargo test --no-default-features --features og-images
	cargo test --features ssr,og-images,json-ld
	cargo test --all-features

# Performance & Benchmarking

.PHONY: bench
bench: ## Run benchmarks
	@echo "⚡ Running benchmarks..."
	cargo bench $(BUILD_FEATURES)

.PHONY: bench-compare
bench-compare: ## Run benchmarks and compare with baseline
	@echo "⚡ Running benchmark comparison..."
	cargo bench $(BUILD_FEATURES) -- --save-baseline current
	@if [ -f "target/criterion/baseline/estimates.json" ]; then \
		echo "Comparing with baseline..."; \
		cargo bench $(BUILD_FEATURES) -- --baseline current; \
	else \
		echo "No baseline found, saving current as baseline"; \
		cp -r target/criterion/current target/criterion/baseline; \
	fi

.PHONY: perf
perf: ## Run performance regression tests
	@echo "⚡ Running performance regression tests..."
	cargo test --release --test performance_regression_test

# Coverage & Quality

.PHONY: coverage
coverage: ## Generate test coverage report
	@echo "📊 Generating coverage report..."
	cargo tarpaulin --all-features --out $(COVERAGE_OUTPUT) --output-dir target/coverage
	@echo "📊 Coverage report generated in target/coverage/"

.PHONY: coverage-xml
coverage-xml: ## Generate XML coverage report for CI
	cargo tarpaulin --all-features --out Xml --output-dir target/coverage

.PHONY: audit
audit: ## Run security audit
	@echo "🔒 Running security audit..."
	cargo audit

.PHONY: outdated
outdated: ## Check for outdated dependencies
	@echo "📦 Checking for outdated dependencies..."
	cargo outdated

# E2E Testing

.PHONY: e2e
e2e: ## Run end-to-end tests (requires test server)
	@echo "🌐 Running E2E tests..."
	@if ! command -v npx > /dev/null; then \
		echo "❌ Node.js/npm required for E2E tests"; \
		exit 1; \
	fi
	npx playwright install --with-deps
	cargo test --test e2e $(TEST_FEATURES)

.PHONY: e2e-visual
e2e-visual: ## Run visual regression tests
	@echo "🖼️  Running visual regression tests..."
	cargo test --test visual_regression_test $(TEST_FEATURES)

# Build Commands

.PHONY: build
build: ## Build the project
	@echo "🔨 Building project..."
	cargo build $(BUILD_FEATURES)

.PHONY: build-release
build-release: ## Build in release mode
	@echo "🔨 Building in release mode..."
	cargo build --release $(BUILD_FEATURES)

.PHONY: build-docs
build-docs: ## Build documentation
	@echo "📚 Building documentation..."
	RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc $(BUILD_FEATURES) --no-deps

.PHONY: serve-docs
serve-docs: build-docs ## Build and serve documentation locally
	@echo "📚 Serving documentation at http://localhost:8000"
	python3 -m http.server 8000 -d target/doc

# Release Commands

.PHONY: pre-release
pre-release: ## Run all pre-release checks
	@echo "🚀 Running pre-release checks..."
	$(MAKE) check
	$(MAKE) test
	$(MAKE) test-features
	$(MAKE) audit
	$(MAKE) build-docs
	@echo "✅ Pre-release checks passed!"

.PHONY: release-dry-run
release-dry-run: ## Dry-run cargo publish
	@echo "🚀 Dry-run release..."
	cargo publish --dry-run $(BUILD_FEATURES)

# Development Utilities

.PHONY: clean
clean: ## Clean build artifacts
	@echo "🧹 Cleaning..."
	cargo clean
	rm -rf target/coverage/
	rm -rf tests/fixtures/visual/*-current.png

.PHONY: clean-all
clean-all: clean ## Clean everything including dependencies
	@echo "🧹 Deep cleaning..."
	rm -rf ~/.cargo/registry/cache/
	rm -rf ~/.cargo/git/

.PHONY: deps
deps: ## Update dependencies
	@echo "📦 Updating dependencies..."
	cargo update

.PHONY: tree
tree: ## Show dependency tree
	cargo tree $(BUILD_FEATURES)

# CI/Local Testing

.PHONY: ci
ci: ## Run full CI pipeline locally
	@echo "🤖 Running CI pipeline..."
	$(MAKE) check
	$(MAKE) test
	$(MAKE) test-features
	$(MAKE) audit
	$(MAKE) coverage-xml
	@echo "✅ CI pipeline completed!"

.PHONY: nightly
nightly: ## Run nightly tests locally
	@echo "🌙 Running nightly tests..."
	$(MAKE) perf
	$(MAKE) bench
	$(MAKE) e2e
	@echo "✅ Nightly tests completed!"

# Docker commands (if Dockerfile exists)

.PHONY: docker-build
docker-build: ## Build Docker image
	@if [ -f "Dockerfile" ]; then \
		echo "🐳 Building Docker image..."; \
		docker build -t leptos-next-metadata .; \
	else \
		echo "❌ No Dockerfile found"; \
		exit 1; \
	fi

.PHONY: docker-test
docker-test: ## Run tests in Docker
	@if [ -f "Dockerfile" ]; then \
		echo "🐳 Running tests in Docker..."; \
		docker run --rm leptos-next-metadata cargo test --all-features; \
	else \
		echo "❌ No Dockerfile found"; \
		exit 1; \
	fi

# Environment checks

.PHONY: env-check
env-check: ## Check development environment
	@echo "🔍 Checking development environment..."
	@echo "Rust version: $$(rustc --version)"
	@echo "Cargo version: $$(cargo --version)"
	@echo "Available targets: $$(rustup target list --installed)"
	@echo "Available components: $$(rustup component list --installed)"
	@if command -v node > /dev/null; then \
		echo "Node.js version: $$(node --version)"; \
	else \
		echo "❌ Node.js not found (required for E2E tests)"; \
	fi
	@if command -v npx > /dev/null; then \
		echo "npm version: $$(npm --version)"; \
	else \
		echo "❌ npm not found (required for E2E tests)"; \
	fi

# Installation commands

.PHONY: install-tools
install-tools: ## Install additional development tools
	@echo "🛠️  Installing development tools..."
	cargo install cargo-watch cargo-expand cargo-udeps
	cargo install wasm-pack
	@if command -v npm > /dev/null; then \
		npm install -g @playwright/test; \
		npx playwright install; \
	fi
	@echo "✅ Development tools installed!"

# Quick development commands

.PHONY: quick-test
quick-test: ## Quick test (unit tests only)
	cargo test --lib --quiet

.PHONY: quick-check
quick-check: ## Quick check (no tests)
	cargo check $(BUILD_FEATURES)

.PHONY: watch
watch: ## Watch for changes and run checks
	cargo watch -x 'check --all-features' -x 'test --lib'

# Help for specific testing scenarios

.PHONY: test-help
test-help: ## Show testing help
	@echo "Testing Commands:"
	@echo "  make test          - Run all tests"
	@echo "  make test-unit     - Run unit tests only"
	@echo "  make test-integration - Run integration tests"
	@echo "  make test-features - Test feature combinations"
	@echo "  make e2e           - Run E2E tests (requires Node.js)"
	@echo "  make perf          - Run performance tests"
	@echo "  make coverage      - Generate coverage report"
	@echo ""
	@echo "Environment Variables:"
	@echo "  TEST_FEATURES      - Features to test (default: --all-features)"
	@echo "  COVERAGE_OUTPUT    - Coverage format (default: html)"
	@echo "  BUILD_FEATURES     - Features to build (default: --all-features)"