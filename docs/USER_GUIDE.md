# User Guide

## Before You Start

Have on hand:

- VPN server address;
- protocol required by the provider;
- username;
- password;
- OpenConnect installed on the system.

On Linux, a graphical Polkit agent is also required. Common examples include `polkit-gnome`, `lxqt-policykit`, `mate-polkit`, or the agent already integrated into the desktop environment.

## Language

Use the language selector in the upper right corner of the window to change the
interface. Portuguese, English, French, Spanish, German, Japanese, Mandarin
(simplified Chinese), Dutch, and Esperanto are available. English is the
default. The choice applies immediately to the application's labels, states,
validations, and messages during the current session.

## Interface Fields

- `VPN Server`: address provided by the administrator, such as `vpn.example.org`.
- `Protocol`: type of VPN used by the server.
- `Username`: VPN account login.
- `Password`: VPN account password.

## Connect

1. Fill in the fields.
2. Click `Connect to VPN` or press `Enter`.
3. Authorize the operation in the system dialog.
4. Wait for the status to turn green.

## Disconnect

With the VPN active, click `Disconnect` or press `Enter`.

## Protocols

- `Cisco AnyConnect / OpenConnect`, `Cisco Secure Client`, and `OpenConnect Server (ocserv)`: use the `anyconnect` protocol.
- `Palo Alto Networks GlobalProtect` and `Prisma Access`: use the `gp` protocol.
- `Pulse Secure` and `Ivanti Connect Secure`: use the `pulse` protocol.
- `Fortinet FortiGate`: uses the `fortinet` protocol.
- `F5 BIG-IP`: used by F5.
- `Juniper Network Connect`: used by Juniper.
- `Array Networks AG SSL VPN`: uses the `array` protocol.

If the administrator did not provide the protocol, first test `Cisco AnyConnect / OpenConnect`.
