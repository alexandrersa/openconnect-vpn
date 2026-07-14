# Troubleshooting

## The application says OpenConnect was not found

Install OpenConnect using your distribution's package manager.

## The application says pkexec was not found

Install Polkit. On Linux, creating the network interface requires administrative privilege.

## The authorization dialog does not appear

Confirm that a graphical Polkit agent is running. In minimalist sessions, this agent may not start automatically.

## The VPN connects, but internal access does not work

Check:

- selected protocol;
- server address;
- DNS received by the VPN;
- routes installed by the OpenConnect script;
- server policies.

## Credentials work in the terminal, but not in the UI

Compare the equivalent command:

```bash
sudo openconnect --protocol=<protocol> <server> -u <user>
```

In the UI, the password is sent via `stdin`; it does not appear in the command.

## Authentication with MFA or external SSO

The current version covers password via stdin. Environments with MFA, SSO via browser, certificates, or custom forms may require additional support.
