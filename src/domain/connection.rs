#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
    Error,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimaryAction {
    Connect,
    Disconnect,
}

impl ConnectionState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Disconnected => "Desconectada",
            Self::Connecting => "Conectando…",
            Self::Connected => "Conectada",
            Self::Disconnecting => "Desconectando…",
            Self::Error => "Atenção necessária",
        }
    }

    pub fn signal_index(self) -> usize {
        match self {
            Self::Disconnected | Self::Error => 0,
            Self::Connecting | Self::Disconnecting => 1,
            Self::Connected => 2,
        }
    }
}

pub fn primary_action_for(state: ConnectionState, busy: bool) -> Option<PrimaryAction> {
    if busy {
        return None;
    }

    match state {
        ConnectionState::Disconnected | ConnectionState::Error => Some(PrimaryAction::Connect),
        ConnectionState::Connected => Some(PrimaryAction::Disconnect),
        ConnectionState::Connecting | ConnectionState::Disconnecting => None,
    }
}
