# Security

## Reporting Vulnerabilities

Open a private issue or contact the repository maintainers before publishing technical details.

## Security Model

- Passwords are kept only in memory during the connection attempt.
- The project uses `zeroize` to reduce password persistence in memory.
- Passwords are sent to OpenConnect via `stdin`.
- Passwords are not passed as process arguments.
- Disconnection validates that the PID belongs to the expected process before terminating.

## Out of Current Scope

- Credential storage.
- Native system keychain integration.
- External MFA/SSO.
- Native elevation policy for Windows/macOS/FreeBSD.
