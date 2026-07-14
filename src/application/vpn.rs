use std::{error::Error, fmt};

use crate::domain::Credentials;

pub type VpnBackendResult<T> = Result<T, VpnBackendError>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendErrorMessage {
    UnsupportedPlatform,
    SystemAuthorizationUnavailable,
    OpenConnectNotFound,
    PkexecNotFound,
    AuthorizationCancelled,
    ConnectionUnavailable,
    PolkitStartFailed,
    DisconnectFailed,
    DisconnectStillActive,
    ProcessMonitoringFailed,
    StateStoreFailed,
    ConnectionStartFailed,
}

pub trait VpnBackend: Send + Sync {
    fn preflight(&self) -> VpnBackendResult<()>;
    fn is_connected(&self) -> bool;
    fn active_pid(&self) -> Option<u32>;
    fn is_managed_process(&self, pid: u32) -> bool;
    fn connect(&self, credentials: Credentials) -> VpnBackendResult<Box<dyn VpnSession>>;
    fn disconnect(&self, pid: u32) -> VpnBackendResult<()>;
    fn clear_state(&self);
}

pub trait VpnSession: Send {
    fn pid(&self) -> u32;
    fn verified(&self) -> bool;
    fn poll(&mut self) -> VpnBackendResult<SessionStatus>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionStatus {
    Active,
    Exited,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VpnBackendError {
    content: VpnBackendErrorContent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum VpnBackendErrorContent {
    Plain(String),
    Localized {
        message: BackendErrorMessage,
        detail: Option<String>,
    },
}

impl VpnBackendError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            content: VpnBackendErrorContent::Plain(message.into()),
        }
    }

    pub fn localized(message: BackendErrorMessage) -> Self {
        Self {
            content: VpnBackendErrorContent::Localized {
                message,
                detail: None,
            },
        }
    }

    pub fn localized_with_detail(message: BackendErrorMessage, detail: impl Into<String>) -> Self {
        Self {
            content: VpnBackendErrorContent::Localized {
                message,
                detail: Some(detail.into()),
            },
        }
    }

    pub fn localized_message(&self) -> Option<BackendErrorMessage> {
        match &self.content {
            VpnBackendErrorContent::Plain(_) => None,
            VpnBackendErrorContent::Localized { message, .. } => Some(*message),
        }
    }

    pub fn detail(&self) -> Option<&str> {
        match &self.content {
            VpnBackendErrorContent::Plain(_) => None,
            VpnBackendErrorContent::Localized { detail, .. } => detail.as_deref(),
        }
    }
}

fn portuguese_backend_message(message: BackendErrorMessage) -> &'static str {
    match message {
        BackendErrorMessage::UnsupportedPlatform => {
            "Este binário foi gerado para este sistema, mas a conexão automática com privilégio administrativo ainda está implementada apenas no Linux."
        }
        BackendErrorMessage::SystemAuthorizationUnavailable => {
            "Não foi possível abrir a autorização do sistema. Verifique o Polkit."
        }
        BackendErrorMessage::OpenConnectNotFound => {
            "O OpenConnect não foi encontrado. Instale o pacote openconnect."
        }
        BackendErrorMessage::PkexecNotFound => {
            "O pkexec não foi encontrado. Instale o Polkit antes de conectar."
        }
        BackendErrorMessage::AuthorizationCancelled => "A autorização do sistema foi cancelada.",
        BackendErrorMessage::ConnectionUnavailable => {
            "Não foi possível estabelecer conexão com o servidor informado. Verifique servidor, protocolo, credenciais, autorização do sistema e acesso à internet."
        }
        BackendErrorMessage::PolkitStartFailed => "Não foi possível iniciar o Polkit",
        BackendErrorMessage::DisconnectFailed => "Não foi possível encerrar a VPN",
        BackendErrorMessage::DisconnectStillActive => {
            "O pedido de desconexão foi enviado, mas o túnel ainda está ativo. Tente novamente."
        }
        BackendErrorMessage::ProcessMonitoringFailed => {
            "Não foi possível acompanhar o processo do OpenConnect."
        }
        BackendErrorMessage::StateStoreFailed => "Não foi possível gravar o estado local da VPN",
        BackendErrorMessage::ConnectionStartFailed => "Não foi possível iniciar a VPN",
    }
}

impl fmt::Display for VpnBackendError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.content {
            VpnBackendErrorContent::Plain(message) => formatter.write_str(message),
            VpnBackendErrorContent::Localized { message, detail } => {
                formatter.write_str(portuguese_backend_message(*message))?;
                if let Some(detail) = detail
                    && !detail.trim().is_empty()
                {
                    write!(formatter, ": {detail}")?;
                }
                Ok(())
            }
        }
    }
}

impl Error for VpnBackendError {}

impl From<String> for VpnBackendError {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for VpnBackendError {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}
