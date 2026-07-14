# Arquitetura

OpenConnect VPN GUI é organizado em camadas para preservar baixo acoplamento e facilitar contribuições da comunidade.

## Camadas

- `domain`: regras puras de credenciais, servidor, protocolo, estados de conexão e ação primária.
- `application`: portas da aplicação. Define `VpnBackend` e `VpnSession`, contratos que a UI usa sem conhecer detalhes de processo ou sistema operacional.
- `infrastructure`: adaptadores concretos. A implementação atual chama OpenConnect via Polkit no Linux.
- `ui`: composição visual em egui/eframe, tema e widgets reutilizáveis.
- `tests`: cenários BDD com nomes `given_when_then`.

## Padrões Aplicados

- Ports and Adapters: a UI depende de `VpnBackend`, não de `Command`.
- Dependency Inversion: `VpnApp` recebe `Arc<dyn VpnBackend>`.
- Strategy: o protocolo selecionado define o argumento `--protocol` usado pelo OpenConnect.
- Single Responsibility: validação, estado de conexão, processo OpenConnect e renderização estão separados.
- BDD: regras críticas são descritas como comportamento observável em `tests/vpn_behaviour.rs`.

## Fluxo de Conexão

1. A UI coleta servidor, protocolo, usuário e senha.
2. O domínio valida entradas mínimas.
3. A aplicação chama a porta `VpnBackend`.
4. O adaptador OpenConnect executa `pkexec openconnect`.
5. A senha é enviada via `stdin`.
6. O PID do processo é registrado para permitir desconexão segura.

## Segurança

- A senha não é armazenada.
- A senha não aparece nos argumentos do processo.
- A desconexão valida o PID antes de enviar `SIGTERM`.
- O arquivo de PID fica no runtime directory do usuário quando disponível.
