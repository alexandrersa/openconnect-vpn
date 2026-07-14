use openconnect_vpn_gui::{
    application::{BackendErrorMessage, VpnBackendError},
    domain::{ConnectionState, CredentialError},
    i18n::Language,
};

#[test]
fn given_a_new_app_when_selecting_the_default_language_then_it_is_english() {
    assert_eq!(Language::default(), Language::English);
}

#[test]
fn given_each_supported_language_when_loading_its_catalog_then_essential_text_is_available() {
    for language in Language::ALL {
        let catalog = language.catalog();

        for text in [
            catalog.native_name,
            catalog.language,
            catalog.subtitle,
            catalog.server,
            catalog.username,
            catalog.password,
            catalog.connect,
            catalog.disconnect,
            catalog.security_notice,
            catalog.connection_state(ConnectionState::Connected),
            catalog.credential_error(CredentialError::EmptyPassword),
        ] {
            assert!(
                !text.trim().is_empty(),
                "{language:?} needs translated text"
            );
        }
    }
}

#[test]
fn given_a_localized_backend_error_when_switching_languages_then_its_message_changes() {
    let error = VpnBackendError::localized(BackendErrorMessage::OpenConnectNotFound);

    assert_eq!(
        Language::English.catalog().backend_error(&error),
        "OpenConnect was not found. Install the openconnect package."
    );
    assert_eq!(
        Language::Japanese.catalog().backend_error(&error),
        "OpenConnect が見つかりません。openconnect パッケージをインストールしてください。"
    );
    assert_eq!(
        Language::Mandarin.catalog().backend_error(&error),
        "未找到 OpenConnect。请安装 openconnect 软件包。"
    );
    assert_eq!(
        error.to_string(),
        "O OpenConnect não foi encontrado. Instale o pacote openconnect."
    );
}
