# Rust Demo

A minimal cross-platform command-line Hello World written in Rust.

## Usage

```bash
cargo run                      # Hello, world!
cargo run -- Alice             # Hello, Alice!
cargo build --release          # ./target/release/rust_demo
```

## Release Workflow

Push a tag to trigger cross-platform builds:

```bash
git tag v0.2.0
git push origin v0.2.0
```

This automatically builds for:
- Windows x64 (`x86_64-pc-windows-msvc`)
- macOS x64 (`x86_64-apple-darwin`)
- macOS ARM64 (`aarch64-apple-darwin`)
- Linux x64 (`x86_64-unknown-linux-gnu`)
- Linux ARM64 (`aarch64-unknown-linux-gnu`)

Each build uploads a tarball (or zip on Windows) containing the single static binary.

## Project Structure

```
rust_demo/
├── src/main.rs             # CLI entry point
├── Cargo.toml              # Manifest
└── .github/workflows/      # CI/CD
```

## License

MIT