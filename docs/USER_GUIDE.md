# Guia do Usuário

## Antes de Começar

Tenha em mãos:

- endereço do servidor VPN;
- protocolo exigido pelo provedor;
- usuário;
- senha;
- OpenConnect instalado no sistema.

No Linux, também é necessário um agente gráfico do Polkit. Exemplos comuns incluem `polkit-gnome`, `lxqt-policykit`, `mate-polkit` ou o agente já integrado ao ambiente desktop.

## Campos da Interface

- `Servidor VPN`: endereço informado pelo administrador, como `vpn.example.org`.
- `Protocolo`: tipo de VPN usado pelo servidor.
- `Usuário`: login da conta VPN.
- `Senha`: senha da conta VPN.

## Conectar

1. Preencha os campos.
2. Clique em `Conectar à VPN` ou pressione `Enter`.
3. Autorize a operação no diálogo do sistema.
4. Aguarde o status ficar verde.

## Desconectar

Com a VPN ativa, clique em `Desconectar` ou pressione `Enter`.

## Protocolos

- `Cisco AnyConnect / OpenConnect`: padrão OpenConnect mais comum.
- `GlobalProtect`: usado por gateways Palo Alto.
- `Pulse Connect Secure`: usado por Pulse/Ivanti.
- `Fortinet`: usado por FortiGate.
- `F5 BIG-IP`: usado por F5.
- `Juniper Network Connect`: usado por Juniper.
- `Array Networks`: usado por Array.

Se o administrador não informou o protocolo, teste primeiro `Cisco AnyConnect / OpenConnect`.
