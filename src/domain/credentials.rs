use std::{error::Error, fmt};

use zeroize::Zeroizing;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum VpnProtocol {
    #[default]
    AnyConnect,
    CiscoSecure,
    Ocserv,
    PanGp,
    PrismaAccess,
    Pulse,
    Ivanti,
    Juniper,
    F5,
    Fortinet,
    Array,
}

impl VpnProtocol {
    pub const ALL: [Self; 11] = [
        Self::AnyConnect,
        Self::CiscoSecure,
        Self::Ocserv,
        Self::PanGp,
        Self::PrismaAccess,
        Self::Pulse,
        Self::Ivanti,
        Self::Juniper,
        Self::F5,
        Self::Fortinet,
        Self::Array,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::AnyConnect => "Cisco AnyConnect / OpenConnect",
            Self::CiscoSecure => "Cisco Secure Client (AnyConnect)",
            Self::Ocserv => "OpenConnect Server (ocserv)",
            Self::PanGp => "Palo Alto Networks GlobalProtect",
            Self::PrismaAccess => "Prisma Access (GlobalProtect)",
            Self::Pulse => "Pulse Secure",
            Self::Ivanti => "Ivanti Connect Secure",
            Self::Juniper => "Juniper Network Connect",
            Self::F5 => "F5 BIG-IP",
            Self::Fortinet => "Fortinet FortiGate",
            Self::Array => "Array Networks AG SSL VPN",
        }
    }

    pub fn openconnect_name(self) -> &'static str {
        match self {
            Self::AnyConnect | Self::CiscoSecure | Self::Ocserv => "anyconnect",
            Self::PanGp | Self::PrismaAccess => "gp",
            Self::Pulse | Self::Ivanti => "pulse",
            Self::Juniper => "nc",
            Self::F5 => "f5",
            Self::Fortinet => "fortinet",
            Self::Array => "array",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServerAddress(String);

impl ServerAddress {
    pub fn parse(value: &str) -> Result<Self, CredentialError> {
        let value = value.trim();
        let has_whitespace = value.chars().any(char::is_whitespace);

        if value.is_empty() {
            Err(CredentialError::EmptyServer)
        } else if has_whitespace {
            Err(CredentialError::InvalidServer)
        } else {
            Ok(Self(value.to_owned()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Username(String);

impl Username {
    pub fn parse(value: &str) -> Result<Self, CredentialError> {
        let value = value.trim();

        if value.is_empty() {
            Err(CredentialError::EmptyUsername)
        } else {
            Ok(Self(value.to_owned()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct Credentials {
    server: ServerAddress,
    protocol: VpnProtocol,
    username: Username,
    password: Zeroizing<String>,
}

impl Credentials {
    pub fn new(
        server: ServerAddress,
        protocol: VpnProtocol,
        username: Username,
        password: Zeroizing<String>,
    ) -> Result<Self, CredentialError> {
        if password.is_empty() {
            return Err(CredentialError::EmptyPassword);
        }

        Ok(Self {
            server,
            protocol,
            username,
            password,
        })
    }

    pub fn into_parts(self) -> (ServerAddress, VpnProtocol, Username, Zeroizing<String>) {
        (self.server, self.protocol, self.username, self.password)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CredentialError {
    EmptyServer,
    InvalidServer,
    EmptyUsername,
    EmptyPassword,
}

impl fmt::Display for CredentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::EmptyServer => "Informe o servidor VPN.",
            Self::InvalidServer => "Informe um servidor VPN sem espaços.",
            Self::EmptyUsername => "Informe o usuário.",
            Self::EmptyPassword => "Informe a senha.",
        };
        formatter.write_str(message)
    }
}

impl Error for CredentialError {}
