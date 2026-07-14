# Segurança

## Reportar Vulnerabilidades

Abra uma issue privada ou entre em contato com os mantenedores do repositório antes de publicar detalhes técnicos.

## Modelo de Segurança

- Senhas são mantidas somente em memória durante a tentativa de conexão.
- O projeto usa `zeroize` para reduzir permanência da senha em memória.
- Senhas são enviadas ao OpenConnect via `stdin`.
- Senhas não são passadas como argumento de processo.
- A desconexão valida que o PID pertence ao processo esperado antes de encerrar.

## Fora do Escopo Atual

- Armazenamento de credenciais.
- Integração nativa de chaveiro do sistema.
- MFA/SSO externo.
- Política de elevação nativa para Windows/macOS/FreeBSD.
