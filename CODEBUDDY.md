# CODEBUDDY.md

This file provides guidance to CodeBuddy Code when working with code in this repository.

## Project Overview

This is a Tauri 2 + Rust cross-platform GUI application with vanilla HTML/CSS/JS frontend.

## Development Commands

### Setup
```bash
cd src-tauri && npm install
```

### Development
```bash
# Run in development mode with hot reload
npm run tauri dev

# Or directly with cargo
cargo build --manifest-path src-tauri/Cargo.toml
```

### Testing
```bash
# Run Rust unit tests (in src-tauri directory)
cargo test --manifest-path src-tauri/Cargo.toml

# Run specific test
cargo test --manifest-path src-tauri/Cargo.toml greet
```

### Building
```bash
# Build Tauri app for current platform
cd src-tauri && npm run tauri build

# Build for specific target
cd src-tauri && npm run tauri build -- --target x86_64-unknown-linux-gnu
```

### Release
```bash
# Create and push a version tag to trigger GitHub Actions
git tag v0.x.x && git push origin v0.x.x
```

## Architecture

```
rust_demo/
├── src-tauri/              # Tauri application (main workspace)
│   ├── src/main.rs         # Rust backend: Tauri commands, window setup
│   ├── Cargo.toml          # Rust dependencies (Tauri 2, serde)
│   ├── tauri.conf.json     # Tauri configuration (window, bundle settings)
│   ├── package.json        # Node dependencies (@tauri-apps/cli)
│   └── icons/              # App icons (PNG, ICNS, ICO)
├── src/                    # Optional CLI entry point (root level)
├── dist/                   # Frontend assets (HTML/CSS/JS served by Tauri)
└── .github/workflows/       # CI/CD: cross-platform release builds
```

### Key Patterns

- **Tauri commands**: Use `#[tauri::command]` attribute to expose Rust functions to frontend
- **Pure functions**: Business logic separated from `#[tauri::command]` for testability
- **Bundle identifier**: Must use hyphens (not underscores) - e.g., `com.rust-demo.app`

## CI/CD

GitHub Actions builds on tag push for 5 platforms:
- Windows x64 (`x86_64-pc-windows-msvc`)
- macOS x64 (`x86_64-apple-darwin`)
- macOS ARM64 (`aarch64-apple-darwin`)
- Linux x64 (`x86_64-unknown-linux-gnu`)
- Linux ARM64 (`aarch64-unknown-linux-gnu`)

Build artifacts are uploaded and a GitHub Release is created automatically.
