# Build and Release

## Local Build

Linux:

```bash
cargo build --release
```

The binary is generated at:

```text
target/release/openconnect-vpn-gui
```

Windows:

```powershell
cargo build --release
```

The binary is generated at:

```text
target\release\openconnect-vpn-gui.exe
```

Universal macOS:

```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
mkdir -p dist
lipo -create -output dist/openconnect-vpn-gui \
  target/aarch64-apple-darwin/release/openconnect-vpn-gui \
  target/x86_64-apple-darwin/release/openconnect-vpn-gui
```

## GitHub Actions

The `CI` workflow executes:

- `cargo fmt --check`;
- `cargo test --all-targets`;
- `cargo clippy --all-targets -- -D warnings`;
- native build on Linux, Windows, and macOS;
- build in Ubuntu/Fedora/Arch/Void containers;
- FreeBSD check via `cross`.

The `Release packages` workflow verifies formatting, tests, and Clippy before creating platform archives. It publishes `.tar.gz` packages for Unix-like platforms, a `.zip` package for Windows, and a `SHA256SUMS` file in the GitHub Release.

## Publish Release

```bash
git tag v0.1.0
git push origin v0.1.0
```

The workflow publishes the GitHub Release and its packages automatically. For a manual release, run **Release packages** in GitHub Actions and enter the intended tag.

## Compatibility Note

Linux binaries are generated per distribution to reduce system library differences. Even so, the end-user needs to install OpenConnect and Polkit.
