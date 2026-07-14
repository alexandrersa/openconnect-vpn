# Contributing

Contributions are welcome.

## Recommended Flow

1. Open an issue describing the problem or proposal.
2. Create a short and objective branch.
3. Keep changes small and testable.
4. Run local validation before opening a pull request.

```bash
cargo fmt --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
```

## Code Guidelines

- Pure rules go in `domain`.
- Contracts go in `application`.
- Operating system integrations go in `infrastructure`.
- UI goes in `ui`.
- New rules should have BDD tests in `tests/`.

## Security

Do not log passwords, tokens, authentication cookies, or sensitive data in logs, configuration files, or error messages.
