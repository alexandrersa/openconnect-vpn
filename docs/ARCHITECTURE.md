# Architecture

OpenConnect VPN GUI is organized in layers to preserve low coupling and facilitate community contributions.

## Layers

- `domain`: pure rules for credentials, server, protocol, connection states, and primary action.
- `application`: application ports. Defines `VpnBackend` and `VpnSession`, contracts that the UI uses without knowing process or operating system details.
- `infrastructure`: concrete adapters. The current implementation calls OpenConnect via Polkit on Linux.
- `ui`: visual composition in egui/eframe, theme, and reusable widgets.
- `tests`: BDD scenarios with `given_when_then` names.

## Applied Patterns

- Ports and Adapters: the UI depends on `VpnBackend`, not `Command`.
- Dependency Inversion: `VpnApp` receives `Arc<dyn VpnBackend>`.
- Strategy: the selected protocol defines the `--protocol` argument used by OpenConnect.
- Single Responsibility: validation, connection state, OpenConnect process, and rendering are separate.
- BDD: critical rules are described as observable behavior in `tests/vpn_behaviour.rs`.

## Connection Flow

1. The UI collects server, protocol, username, and password.
2. The domain validates minimal inputs.
3. The application calls the `VpnBackend` port.
4. The OpenConnect adapter executes `pkexec openconnect`.
5. The password is sent via `stdin`.
6. The process PID is registered to allow safe disconnection.

## Security

- The password is not stored.
- The password does not appear in the process arguments.
- Disconnection validates the PID before sending `SIGTERM`.
- The PID file is in the user's runtime directory when available.
