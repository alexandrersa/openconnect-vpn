mod connection;
mod credentials;

pub use connection::{ConnectionState, PrimaryAction, primary_action_for};
pub use credentials::{CredentialError, Credentials, ServerAddress, Username, VpnProtocol};
