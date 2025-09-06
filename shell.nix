{ pkgs ? import <nixpkgs> {} }:

let
  # Rust toolchain with specific version
  rustToolchain = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" "rust-analyzer" "rustfmt" "clippy" ];
    targets = [ "wasm32-unknown-unknown" "x86_64-unknown-linux-gnu" ];
  };

  # Node.js and PNPM
  nodejs = pkgs.nodejs_20;
  pnpm = pkgs.nodePackages.pnpm;

  # Playwright browsers
  playwright = pkgs.playwright.override {
    browsers = [ "chromium" "firefox" "webkit" ];
  };

in pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    rustToolchain

    # Node.js ecosystem
    nodejs
    pnpm

    # Playwright and browsers
    playwright

    # Additional development tools
    cargo-watch
    cargo-edit
    cargo-audit
    cargo-deny

    # System dependencies for image processing
    pkg-config
    openssl
    libpng
    libjpeg
    webp
    freetype
    fontconfig

    # For SVG processing
    librsvg

    # For testing
    wasm-pack
    wasm-bindgen-cli

    # For development
    git
    just
  ];

  shellHook = ''
    echo "🚀 leptos-next-metadata development environment"
    echo "📦 Rust: $(rustc --version)"
    echo "📦 Node.js: $(node --version)"
    echo "📦 PNPM: $(pnpm --version)"
    echo "🎭 Playwright: Available"
    echo ""
    echo "Available commands:"
    echo "  cargo test          - Run Rust tests"
    echo "  pnpm test:e2e      - Run Playwright E2E tests"
    echo "  pnpm install        - Install Node.js dependencies"
    echo "  cargo watch         - Watch for changes and rebuild"
    echo "  just                - Run predefined tasks"
  '';

  # Environment variables for Playwright
  PLAYWRIGHT_BROWSERS_PATH = "${playwright}/share/playwright";

  # For image processing libraries
  PKG_CONFIG_PATH = "${pkgs.lib.makeSearchPath "lib/pkgconfig" [
    pkgs.libpng
    pkgs.libjpeg
    pkgs.webp
    pkgs.freetype
    pkgs.fontconfig
    pkgs.librsvg
  ]}";
}
