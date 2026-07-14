# OpenConnect VPN GUI

[![CI](https://github.com/alexandrersa/openconnect-vpn/actions/workflows/ci.yml/badge.svg)](https://github.com/alexandrersa/openconnect-vpn/actions/workflows/ci.yml)
[![Release](https://github.com/alexandrersa/openconnect-vpn/actions/workflows/release.yml/badge.svg)](https://github.com/alexandrersa/openconnect-vpn/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Secure, cross-platform graphical VPN client for OpenConnect, Cisco AnyConnect, Cisco Secure Client, Palo Alto GlobalProtect, Prisma Access, Ivanti Connect Secure, Pulse Secure, Juniper, F5, Fortinet, Array Networks, and ocserv.

OpenConnect VPN GUI lets you enter a server, protocol profile, username, and password without exposing the password on the command line. It is built with Rust and egui for Linux, Windows, macOS, and FreeBSD.

The initial focus is on Linux desktop with OpenConnect and Polkit. The workflows also generate binaries for Windows, macOS, and FreeBSD, but privilege elevation and network integration on these systems depend on operational adaptation or local administrative policy.

## Features

- Simple, fixed graphical interface for OpenConnect connections.
- Free server field, with no pre-configured server.
- Eleven provider profiles mapped to the seven OpenConnect VPN protocols.
- Password sent via `stdin` with `--passwd-on-stdin`.
- Foreground execution, without `--background`.
- Disconnection control via validated PID.
- BDD tests for domain rules and OpenConnect arguments.
- CI and release for multiple platforms and Linux distributions.

## Supported VPN profiles

OpenConnect currently implements seven wire protocols. The GUI exposes provider-specific profiles so the correct OpenConnect protocol is easier to identify.

| Profile | OpenConnect protocol |
| --- | --- |
| Cisco AnyConnect / Cisco Secure Client / ocserv | `anyconnect` |
| Palo Alto GlobalProtect / Prisma Access | `gp` |
| Pulse Secure / Ivanti Connect Secure | `pulse` |
| Juniper Network Connect | `nc` |
| F5 BIG-IP | `f5` |
| Fortinet FortiGate | `fortinet` |
| Array Networks AG SSL VPN | `array` |

## Requirements on Linux

You need to have OpenConnect, Polkit, and a graphical Polkit agent running in the user session.

Ubuntu/Debian:

```bash
sudo apt install openconnect policykit-1
```

Fedora:

```bash
sudo dnf install openconnect polkit
```

Arch Linux:

```bash
sudo pacman -S openconnect polkit
```

Void Linux:

```bash
sudo xbps-install -S openconnect polkit
```

Run the application as a regular user. Do not use `sudo` to open the interface.

## Usage

1. Open the application.
2. Enter the VPN server, for example `vpn.example.org`.
3. Choose the protocol required by your provider.
4. Enter your username and password.
5. Click `Connect to VPN` or press `Enter`.
6. Authorize the operation in the system dialog, if requested.

To disconnect, click `Disconnect` or press `Enter` when the connection is active.

## Development

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
cargo run --release
```

## Architecture

The project uses `domain`, `application`, `infrastructure`, and `ui` layers, with ports and adapters. The technical documentation is in [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Documentation

- [User Guide](docs/USER_GUIDE.md)
- [Build and Release](docs/BUILD_AND_RELEASE.md)
- [Troubleshooting](docs/TROUBLESHOOTING.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Contributing](CONTRIBUTING.md)
- [Security](SECURITY.md)

## Binaries

The `.github/workflows/release.yml` workflow verifies the source, packages the binaries, generates `SHA256SUMS`, and publishes a GitHub Release for:

- Linux Ubuntu x86_64;
- Linux Fedora x86_64;
- Linux Arch Linux x86_64;
- Linux Void Linux x86_64;
- Windows x86_64;
- macOS universal;
- FreeBSD x86_64.

Trigger it manually in GitHub Actions or publish a tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The tag-triggered workflow creates the public release automatically. To run it manually, use **Actions → Release packages** and provide the release tag.

The local Linux binary is located at:

```text
target/release/openconnect-vpn-gui
```

## Known Limitations

- Privileged connection automation is implemented for Linux with Polkit.
- Windows, macOS, and FreeBSD compile and are distributed, but require privilege integration and OpenConnect appropriate to the environment.
- Authentications with MFA, external SSO, custom forms, and certificates may require future UI extensions.

## License

Distributed under the MIT license. See [LICENSE](LICENSE).

## Credits

- The security background is an original SVG included with the application.
- The [Lato](https://www.latofonts.com/) font is distributed under the SIL Open Font License 1.1, included in `assets/fonts/OFL.txt`.
- The Noto Sans CJK font has been subset to the glyphs used by the interface to support Japanese and Mandarin. It is distributed under the SIL Open Font License 1.1; see `assets/fonts/NotoSansCJK-NOTICE.txt` and `assets/fonts/OFL.txt`.
