# Build e Distribuição

## Build Local

Linux:

```bash
cargo build --release
```

O binário é gerado em:

```text
target/release/openconnect-vpn-gui
```

Windows:

```powershell
cargo build --release
```

O binário é gerado em:

```text
target\release\openconnect-vpn-gui.exe
```

macOS universal:

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

O workflow `CI` executa:

- `cargo fmt --check`;
- `cargo test --all-targets`;
- `cargo clippy --all-targets -- -D warnings`;
- build nativo em Linux, Windows e macOS;
- build em containers Ubuntu/Fedora/Arch/Void;
- checagem FreeBSD via `cross`.

O workflow `Release binaries` gera artefatos para distribuição.

## Publicar Release

```bash
git tag v0.1.0
git push origin v0.1.0
```

Depois baixe os artefatos em GitHub Actions e anexe-os a uma release pública.

## Observação Sobre Compatibilidade

Binários Linux são gerados por distribuição para reduzir diferenças de bibliotecas do sistema. Mesmo assim, o usuário final precisa instalar OpenConnect e Polkit.
