use openconnect_vpn_gui::{
    domain::{
        ConnectionState, PrimaryAction, ServerAddress, Username, VpnProtocol, primary_action_for,
    },
    infrastructure::openconnect_support::{command_error, openconnect_args},
};

#[test]
fn given_server_without_spaces_when_validating_then_it_is_accepted() {
    assert!(ServerAddress::parse("vpn.example.org").is_ok());
    assert!(ServerAddress::parse("vpn.example.org/portal").is_ok());
}

#[test]
fn given_empty_or_spaced_server_when_validating_then_it_is_rejected() {
    for value in ["", "   ", "vpn example org"] {
        assert!(
            ServerAddress::parse(value).is_err(),
            "{value:?} deveria ser rejeitado"
        );
    }
}

#[test]
fn given_non_empty_username_when_validating_then_it_is_accepted() {
    assert!(Username::parse("alice").is_ok());
    assert!(Username::parse("user@example.org").is_ok());
}

#[test]
fn given_connection_state_when_deciding_primary_action_then_action_matches_status() {
    assert_eq!(
        primary_action_for(ConnectionState::Disconnected, false),
        Some(PrimaryAction::Connect)
    );
    assert_eq!(
        primary_action_for(ConnectionState::Error, false),
        Some(PrimaryAction::Connect)
    );
    assert_eq!(
        primary_action_for(ConnectionState::Connected, false),
        Some(PrimaryAction::Disconnect)
    );
    assert_eq!(primary_action_for(ConnectionState::Connecting, false), None);
    assert_eq!(
        primary_action_for(ConnectionState::Disconnecting, false),
        None
    );
    assert_eq!(
        primary_action_for(ConnectionState::Disconnected, true),
        None
    );
}

#[test]
fn given_openconnect_command_when_building_args_then_server_protocol_and_stdin_are_used() {
    let args = openconnect_args("vpn.example.org", VpnProtocol::PanGp, "alice");

    assert!(args.iter().any(|arg| arg == "--protocol=gp"));
    assert!(args.iter().any(|arg| arg == "vpn.example.org"));
    assert!(args.iter().any(|arg| arg == "alice"));
    assert!(args.iter().any(|arg| arg == "--passwd-on-stdin"));
    assert!(args.iter().any(|arg| arg == "--non-inter"));
    assert!(!args.iter().any(|arg| arg == "--background"));
    assert!(!args.iter().any(|arg| arg == "--pid-file"));
}

#[test]
fn given_provider_profiles_when_building_args_then_each_uses_its_supported_protocol() {
    let expected_protocols = [
        (VpnProtocol::CiscoSecure, "anyconnect"),
        (VpnProtocol::Ocserv, "anyconnect"),
        (VpnProtocol::PrismaAccess, "gp"),
        (VpnProtocol::Ivanti, "pulse"),
    ];

    for (profile, openconnect_protocol) in expected_protocols {
        let args = openconnect_args("vpn.example.org", profile, "alice");
        assert!(
            args.iter()
                .any(|arg| arg == &format!("--protocol={openconnect_protocol}")),
            "{profile:?} deveria usar {openconnect_protocol}"
        );
    }
}

#[test]
fn given_all_supported_protocols_when_building_args_then_each_has_an_openconnect_name() {
    for protocol in VpnProtocol::ALL {
        let args = openconnect_args("vpn.example.org", protocol, "alice");
        assert!(
            args.iter()
                .any(|arg| arg == &format!("--protocol={}", protocol.openconnect_name())),
            "{protocol:?} deveria gerar argumento de protocolo"
        );
    }
}

#[test]
fn given_noisy_command_output_when_formatting_error_then_message_is_human_sized() {
    let stderr = b"falha\nna\tconexao";
    assert_eq!(
        command_error("Erro", stderr, b"stdout"),
        "Erro: falha na conexao"
    );

    let long_error = vec![b'x'; 400];
    assert_eq!(command_error("Erro", &long_error, b"").chars().count(), 306);
}
