# CODEBUDDY.md

This file provides guidance to CodeBuddy Code when working with code in this repository.

## Project Overview

A minimal cross-platform command-line Hello World written in Rust. No GUI, no frontend — single binary, single source file.

## Development Commands

### Build & Run
```bash
cargo run                       # prints "Hello, world!"
cargo run -- Alice              # prints "Hello, Alice!"
cargo build --release           # ./target/release/rust_demo
```

### Testing
```bash
cargo test
```

### Release
```bash
git tag v0.x.x && git push origin v0.x.x   # triggers CI for 5 platforms
```

## Architecture

```
rust_demo/
├── src/main.rs          # Single-file CLI: reads optional argv[1], prints greeting
├── Cargo.toml           # Manifest (release profile: lto, opt-level=z, strip)
└── .github/workflows/   # CI/CD: cross-platform release builds
```

### Key Patterns

- **Stdlib only**: no external crates — uses `std::env::args()` for input
- **Single source file**: all logic in `src/main.rs`
- **Release profile**: aggressively optimized (`lto`, `codegen-units=1`, `opt-level="z"`, `strip`)

## CI/CD

GitHub Actions builds on tag push for 4 platforms:
- Windows x64 (`x86_64-pc-windows-msvc`)
- macOS x64 (`x86_64-apple-darwin`)
- macOS ARM64 (`aarch64-apple-darwin`)
- Linux x64 (`x86_64-unknown-linux-gnu`)

Each job runs natively on its runner's architecture — no cross-compilation. (Linux ARM64 omitted; cross-compiling to aarch64 from an x86_64 runner requires extra toolchain setup that's overkill for this demo.)

Each job runs `cargo build --release --target <triple>`, packages the binary into a tarball (or zip on Windows), and uploads as an artifact. Once all jobs succeed, `create-release` aggregates artifacts and publishes a GitHub Release via `softprops/action-gh-release`.