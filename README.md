# OpenConnect VPN GUI

OpenConnect VPN GUI é um cliente gráfico genérico para conexões VPN compatíveis com OpenConnect. A aplicação permite informar servidor, protocolo, usuário e senha sem expor a senha na linha de comando.

O foco inicial é Linux desktop com OpenConnect e Polkit. Os workflows também geram binários para Windows, macOS e FreeBSD, mas a elevação de privilégio e a integração de rede nesses sistemas dependem de adaptação operacional ou política administrativa local.

## Recursos

- Interface gráfica fixa e simples para conexões OpenConnect.
- Campo de servidor livre, sem servidor pré-configurado.
- Seleção de protocolo: AnyConnect/OpenConnect, GlobalProtect, Pulse, Fortinet, F5, Juniper e Array.
- Senha enviada por `stdin` com `--passwd-on-stdin`.
- Execução em primeiro plano, sem `--background`.
- Controle de desconexão por PID validado.
- Testes BDD para regras de domínio e argumentos do OpenConnect.
- CI e release para múltiplas plataformas e distribuições Linux.

## Requisitos no Linux

É necessário ter OpenConnect, Polkit e um agente gráfico do Polkit rodando na sessão do usuário.

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

Execute o aplicativo como usuário comum. Não use `sudo` para abrir a interface.

## Uso

1. Abra o aplicativo.
2. Informe o servidor VPN, por exemplo `vpn.example.org`.
3. Escolha o protocolo exigido pelo seu provedor.
4. Informe usuário e senha.
5. Clique em `Conectar à VPN` ou pressione `Enter`.
6. Autorize a operação no diálogo do sistema, se solicitado.

Para desconectar, clique em `Desconectar` ou pressione `Enter` quando a conexão estiver ativa.

## Desenvolvimento

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
cargo run --release
```

## Arquitetura

O projeto usa camadas `domain`, `application`, `infrastructure` e `ui`, com portas e adaptadores. A documentação técnica está em [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Documentação

- [Guia do usuário](docs/USER_GUIDE.md)
- [Build e distribuição](docs/BUILD_AND_RELEASE.md)
- [Solução de problemas](docs/TROUBLESHOOTING.md)
- [Arquitetura](docs/ARCHITECTURE.md)
- [Contribuição](CONTRIBUTING.md)
- [Segurança](SECURITY.md)

## Binários

O workflow `.github/workflows/release.yml` gera artefatos para:

- Linux Ubuntu x86_64;
- Linux Fedora x86_64;
- Linux Arch Linux x86_64;
- Linux Void Linux x86_64;
- Windows x86_64;
- macOS universal;
- FreeBSD x86_64.

Acione manualmente em GitHub Actions ou publique uma tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

O binário local Linux fica em:

```text
target/release/openconnect-vpn-gui
```

## Limitações conhecidas

- A automação de conexão privilegiada está implementada para Linux com Polkit.
- Windows, macOS e FreeBSD compilam e são distribuídos, mas exigem integração de privilégio e OpenConnect adequada ao ambiente.
- Autenticações com MFA, SSO externo, formulários customizados e certificados podem exigir extensões futuras da UI.

## Licença

Distribuído sob licença MIT. Veja [LICENSE](LICENSE).

## Créditos

- O fundo usa uma foto de Abdul Ghofur disponibilizada pela [Unsplash](https://unsplash.com/photos/a-river-running-through-a-lush-green-forest-S6AdzlVncNc) sob a [licença Unsplash](https://unsplash.com/license).
- A fonte [JetBrains Mono](https://github.com/JetBrains/JetBrainsMono) é distribuída sob a SIL Open Font License 1.1, incluída em `assets/fonts/OFL.txt`.
