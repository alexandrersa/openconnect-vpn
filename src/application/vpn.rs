use std::{error::Error, fmt};

use crate::domain::Credentials;

pub type VpnBackendResult<T> = Result<T, VpnBackendError>;

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
    message: String,
}

impl VpnBackendError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for VpnBackendError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
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
