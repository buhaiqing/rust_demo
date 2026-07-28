# Rust Demo

A cross-platform GUI application built with Tauri 2 and Rust.

## Features

- Cross-platform GUI (Windows, macOS, Linux)
- Fast native performance with Rust backend
- Web-based frontend (HTML/CSS/JS)
- GitHub Actions for automated cross-platform builds

## Tech Stack

- **Frontend**: Vanilla HTML/CSS/JS
- **Backend**: Rust + Tauri 2
- **CI/CD**: GitHub Actions

## Development

### Prerequisites

- Rust 1.70+
- Node.js 18+
- npm

### Setup

```bash
# Install frontend dependencies
cd src-tauri && npm install

# Run in development mode
npm run tauri dev
```

### Build

```bash
# Build for current platform
cd src-tauri && npm run tauri build

# Build for release (creates installable bundles)
npm run tauri build -- --release
```

## Release Workflow

Push a tag to trigger cross-platform builds:

```bash
git tag v0.1.0
git push origin v0.1.0
```

This automatically builds for:
- Windows x64 (`x86_64-pc-windows-msvc`)
- macOS x64 (`x86_64-apple-darwin`)
- macOS ARM64 (`aarch64-apple-darwin`)
- Linux x64 (`x86_64-unknown-linux-gnu`)
- Linux ARM64 (`aarch64-unknown-linux-gnu`)

## Project Structure

```
rust_demo/
├── src/                    # CLI entry point (optional)
├── src-tauri/             # Tauri application
│   ├── src/main.rs         # Rust source
│   ├── Cargo.toml          # Rust dependencies
│   ├── tauri.conf.json     # Tauri config
│   ├── package.json        # Node dependencies
│   └── icons/              # App icons
├── dist/                   # Frontend assets
├── .github/workflows/      # CI/CD
└── Cargo.toml              # Workspace manifest
```

## License

MIT
