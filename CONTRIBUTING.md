# Contribuição

Contribuições são bem-vindas.

## Fluxo Recomendado

1. Abra uma issue descrevendo o problema ou proposta.
2. Crie uma branch curta e objetiva.
3. Mantenha mudanças pequenas e testáveis.
4. Rode a validação local antes de abrir pull request.

```bash
cargo fmt --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
```

## Diretrizes de Código

- Regras puras ficam em `domain`.
- Contratos ficam em `application`.
- Integrações com sistema operacional ficam em `infrastructure`.
- UI fica em `ui`.
- Novas regras devem ter testes BDD em `tests/`.

## Segurança

Não registre senha, token, cookie de autenticação ou dados sensíveis em logs, arquivos de configuração ou mensagens de erro.
