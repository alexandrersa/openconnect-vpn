# Solução de Problemas

## O aplicativo diz que OpenConnect não foi encontrado

Instale OpenConnect pelo gerenciador de pacotes da distribuição.

## O aplicativo diz que pkexec não foi encontrado

Instale Polkit. No Linux, a criação da interface de rede exige privilégio administrativo.

## O diálogo de autorização não aparece

Confirme que existe um agente gráfico do Polkit em execução. Em sessões minimalistas, esse agente pode não iniciar automaticamente.

## A VPN conecta, mas o acesso interno não funciona

Verifique:

- protocolo selecionado;
- endereço do servidor;
- DNS recebido pela VPN;
- rotas instaladas pelo script do OpenConnect;
- políticas do servidor.

## Credenciais funcionam no terminal, mas não na UI

Compare o comando equivalente:

```bash
sudo openconnect --protocol=<protocolo> <servidor> -u <usuario>
```

Na UI, a senha é enviada por `stdin`; ela não aparece no comando.

## Autenticação com MFA ou SSO externo

A versão atual cobre senha via stdin. Ambientes com MFA, SSO via navegador, certificados ou formulários customizados podem exigir suporte adicional.
