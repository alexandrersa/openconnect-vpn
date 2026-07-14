use std::{
    sync::{
        Arc,
        mpsc::{self, Receiver, TryRecvError},
    },
    thread,
    time::{Duration, Instant},
};

use eframe::{App, Frame, egui};
use zeroize::Zeroizing;

use crate::{
    application::{SessionStatus, VpnBackend, VpnBackendError, VpnBackendResult, VpnSession},
    domain::{
        ConnectionState, CredentialError, Credentials, PrimaryAction, ServerAddress, Username,
        VpnProtocol, primary_action_for,
    },
    i18n::Language,
    ui::{
        theme::FORM_WIDTH,
        widgets::{centered_action_button, centered_label, draw_connection_status},
    },
};

const POLL_INTERVAL: Duration = Duration::from_secs(1);

enum WorkerMessage {
    Connected(VpnBackendResult<Box<dyn VpnSession>>),
    Disconnected(VpnBackendResult<()>),
}

#[derive(Clone, Copy)]
enum StatusMessage {
    AlreadyConnected,
    EnterCredentials,
    AuthorizeOperation,
    NoConnection,
    InvalidStateFile,
    ClosingTunnel,
    ConnectionVerified,
    ConnectionUnverified,
    VpnDisconnected,
    OperationInterrupted,
    ConnectionEnded,
}

enum StatusDetail {
    Message(StatusMessage),
    CredentialError(CredentialError),
    BackendError(VpnBackendError),
}

pub struct VpnApp {
    backend: Arc<dyn VpnBackend>,
    server: String,
    protocol: VpnProtocol,
    username: String,
    password: Zeroizing<String>,
    language: Language,
    state: ConnectionState,
    detail: StatusDetail,
    worker: Option<Receiver<WorkerMessage>>,
    active_session: Option<Box<dyn VpnSession>>,
    last_poll: Instant,
    action_button_rect: Option<egui::Rect>,
}

impl VpnApp {
    pub fn new(backend: Arc<dyn VpnBackend>) -> Self {
        let connected = backend.is_connected();
        Self {
            backend,
            server: String::new(),
            protocol: VpnProtocol::default(),
            username: String::new(),
            password: Zeroizing::new(String::new()),
            language: Language::default(),
            state: if connected {
                ConnectionState::Connected
            } else {
                ConnectionState::Disconnected
            },
            detail: if connected {
                StatusDetail::Message(StatusMessage::AlreadyConnected)
            } else {
                StatusDetail::Message(StatusMessage::EnterCredentials)
            },
            worker: None,
            active_session: None,
            last_poll: Instant::now(),
            action_button_rect: None,
        }
    }

    fn is_busy(&self) -> bool {
        self.worker.is_some()
    }

    fn primary_action(&self) -> Option<PrimaryAction> {
        primary_action_for(self.state, self.is_busy())
    }

    fn run_primary_action(&mut self) {
        match self.primary_action() {
            Some(PrimaryAction::Connect) => self.connect(),
            Some(PrimaryAction::Disconnect) => self.disconnect(),
            None => {}
        }
    }

    fn connect(&mut self) {
        let server = match ServerAddress::parse(&self.server) {
            Ok(server) => server,
            Err(error) => return self.show_credential_error(error),
        };

        let username = match Username::parse(&self.username) {
            Ok(username) => username,
            Err(error) => return self.show_credential_error(error),
        };

        if self.password.is_empty() {
            return self.show_credential_error(CredentialError::EmptyPassword);
        }

        if let Err(error) = self.backend.preflight() {
            self.state = ConnectionState::Error;
            self.detail = StatusDetail::BackendError(error);
            return;
        }

        if self.backend.is_connected() {
            self.state = ConnectionState::Connected;
            self.detail = StatusDetail::Message(StatusMessage::AlreadyConnected);
            return;
        }

        let password = std::mem::replace(&mut self.password, Zeroizing::new(String::new()));
        let credentials = match Credentials::new(server, self.protocol, username, password) {
            Ok(credentials) => credentials,
            Err(error) => return self.show_credential_error(error),
        };

        let backend = Arc::clone(&self.backend);
        let (sender, receiver) = mpsc::channel();
        self.worker = Some(receiver);
        self.state = ConnectionState::Connecting;
        self.detail = StatusDetail::Message(StatusMessage::AuthorizeOperation);

        thread::spawn(move || {
            let result = backend.connect(credentials);
            let _ = sender.send(WorkerMessage::Connected(result));
        });
    }

    fn disconnect(&mut self) {
        let Some(pid) = self
            .active_session
            .as_ref()
            .map(|session| session.pid())
            .or_else(|| self.backend.active_pid())
        else {
            self.state = ConnectionState::Disconnected;
            self.detail = StatusDetail::Message(StatusMessage::NoConnection);
            return;
        };

        if !self.backend.is_managed_process(pid) {
            self.state = ConnectionState::Error;
            self.detail = StatusDetail::Message(StatusMessage::InvalidStateFile);
            return;
        }

        let backend = Arc::clone(&self.backend);
        let (sender, receiver) = mpsc::channel();
        self.worker = Some(receiver);
        self.state = ConnectionState::Disconnecting;
        self.detail = StatusDetail::Message(StatusMessage::ClosingTunnel);

        thread::spawn(move || {
            let result = backend.disconnect(pid);
            let _ = sender.send(WorkerMessage::Disconnected(result));
        });
    }

    fn show_credential_error(&mut self, error: CredentialError) {
        self.state = ConnectionState::Error;
        self.detail = StatusDetail::CredentialError(error);
    }

    fn handle_worker_messages(&mut self) {
        let Some(receiver) = &self.worker else {
            return;
        };

        match receiver.try_recv() {
            Ok(WorkerMessage::Connected(Ok(session))) => {
                let verified = session.verified();
                self.worker = None;
                self.active_session = Some(session);
                self.state = ConnectionState::Connected;
                self.detail = if verified {
                    StatusDetail::Message(StatusMessage::ConnectionVerified)
                } else {
                    StatusDetail::Message(StatusMessage::ConnectionUnverified)
                };
            }
            Ok(WorkerMessage::Connected(Err(error))) => {
                self.worker = None;
                self.state = ConnectionState::Error;
                self.detail = StatusDetail::BackendError(error);
            }
            Ok(WorkerMessage::Disconnected(Ok(()))) => {
                self.worker = None;
                self.clear_active_session();
                self.state = ConnectionState::Disconnected;
                self.detail = StatusDetail::Message(StatusMessage::VpnDisconnected);
            }
            Ok(WorkerMessage::Disconnected(Err(error))) => {
                self.worker = None;
                self.state = ConnectionState::Error;
                self.detail = StatusDetail::BackendError(error);
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                self.worker = None;
                self.state = ConnectionState::Error;
                self.detail = StatusDetail::Message(StatusMessage::OperationInterrupted);
            }
        }
    }

    fn refresh_state(&mut self) {
        if self.worker.is_some() || self.last_poll.elapsed() < POLL_INTERVAL {
            return;
        }
        self.last_poll = Instant::now();

        if let Some(session) = &mut self.active_session {
            match session.poll() {
                Ok(SessionStatus::Active) => {}
                Ok(SessionStatus::Exited) => {
                    self.active_session = None;
                    self.backend.clear_state();
                    self.state = ConnectionState::Disconnected;
                    self.detail = StatusDetail::Message(StatusMessage::ConnectionEnded);
                    return;
                }
                Err(error) => {
                    self.active_session = None;
                    self.state = ConnectionState::Error;
                    self.detail = StatusDetail::BackendError(error);
                    return;
                }
            }
        }

        if self.state == ConnectionState::Connected && !self.backend.is_connected() {
            self.clear_active_session();
            self.state = ConnectionState::Disconnected;
            self.detail = StatusDetail::Message(StatusMessage::ConnectionEnded);
        }
    }

    fn clear_active_session(&mut self) {
        if let Some(mut session) = self.active_session.take() {
            let _ = session.poll();
        }
        self.backend.clear_state();
    }

    fn detail_text(&self) -> String {
        let text = self.language.catalog();
        match &self.detail {
            StatusDetail::Message(message) => match message {
                StatusMessage::AlreadyConnected => text.already_connected,
                StatusMessage::EnterCredentials => text.enter_credentials,
                StatusMessage::AuthorizeOperation => text.authorize_operation,
                StatusMessage::NoConnection => text.no_connection,
                StatusMessage::InvalidStateFile => text.invalid_state_file,
                StatusMessage::ClosingTunnel => text.closing_tunnel,
                StatusMessage::ConnectionVerified => text.connection_verified,
                StatusMessage::ConnectionUnverified => text.connection_unverified,
                StatusMessage::VpnDisconnected => text.vpn_disconnected,
                StatusMessage::OperationInterrupted => text.operation_interrupted,
                StatusMessage::ConnectionEnded => text.connection_ended,
            }
            .to_owned(),
            StatusDetail::CredentialError(error) => text.credential_error(*error).to_owned(),
            StatusDetail::BackendError(error) => text.backend_error(error),
        }
    }
}

impl App for VpnApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        self.handle_worker_messages();
        self.refresh_state();

        if ctx.input(|input| input.key_pressed(egui::Key::Enter)) {
            self.run_primary_action();
        }

        if matches!(
            self.state,
            ConnectionState::Connecting | ConnectionState::Disconnecting
        ) {
            ctx.request_repaint_after(Duration::from_millis(100));
        } else {
            ctx.request_repaint_after(POLL_INTERVAL);
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(egui::Color32::from_rgb(218, 224, 220)))
            .show(ctx, |ui| {
                let background_rect = ui.max_rect();
                egui::Image::new(egui::include_image!("../../assets/vpn-background.jpg"))
                    .fit_to_exact_size(background_rect.size())
                    .tint(egui::Color32::from_rgba_unmultiplied(172, 188, 178, 150))
                    .paint_at(ui, background_rect);
                ui.painter().rect_filled(
                    background_rect,
                    0.0,
                    egui::Color32::from_rgba_unmultiplied(82, 92, 86, 116),
                );
            });

        egui::Area::new(egui::Id::new("openconnect-vpn-card"))
            .order(egui::Order::Foreground)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, -8.0))
            .show(ctx, |ui| self.render_card(ui));
    }
}

impl VpnApp {
    fn render_card(&mut self, ui: &mut egui::Ui) {
        let text = self.language.catalog();
        egui::Frame::new()
            .fill(egui::Color32::from_rgba_unmultiplied(235, 239, 236, 250))
            .stroke(egui::Stroke::new(
                1.0_f32,
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 150),
            ))
            .shadow(egui::Shadow {
                offset: [0, 14],
                blur: 30,
                spread: 0,
                color: egui::Color32::from_rgba_unmultiplied(18, 32, 24, 86),
            })
            .corner_radius(22.0)
            .inner_margin(egui::Margin::symmetric(34, 26))
            .show(ui, |ui| {
                ui.set_width(FORM_WIDTH);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let language_selector = egui::ComboBox::from_id_salt("application-language")
                        .selected_text(self.language.native_name())
                        .show_ui(ui, |ui| {
                            for language in Language::ALL {
                                ui.selectable_value(
                                    &mut self.language,
                                    language,
                                    language.native_name(),
                                );
                            }
                        });
                    if language_selector.response.changed() {
                        ui.ctx().request_repaint();
                    }
                    ui.label(egui::RichText::new(text.language).small());
                });
                ui.add_space(5.0);

                centered_label(
                    ui,
                    FORM_WIDTH,
                    34.0,
                    egui::RichText::new("OpenConnect VPN")
                        .strong()
                        .size(24.0)
                        .color(egui::Color32::from_rgb(14, 93, 52)),
                );
                ui.add_space(4.0);
                centered_label(
                    ui,
                    FORM_WIDTH,
                    20.0,
                    egui::RichText::new(text.subtitle).color(egui::Color32::from_rgb(74, 93, 82)),
                );
                ui.add_space(22.0);

                let busy = self.is_busy();
                ui.add_enabled_ui(!busy && self.state != ConnectionState::Connected, |ui| {
                    centered_label(
                        ui,
                        FORM_WIDTH,
                        18.0,
                        egui::RichText::new(text.server).strong(),
                    );
                    ui.add(
                        egui::TextEdit::singleline(&mut self.server)
                            .hint_text(text.server_hint)
                            .desired_width(FORM_WIDTH),
                    );
                    ui.add_space(12.0);
                    centered_label(
                        ui,
                        FORM_WIDTH,
                        18.0,
                        egui::RichText::new(text.protocol).strong(),
                    );
                    egui::ComboBox::from_id_salt("vpn-protocol")
                        .width(FORM_WIDTH)
                        .selected_text(self.protocol.label())
                        .show_ui(ui, |ui| {
                            for protocol in VpnProtocol::ALL {
                                ui.selectable_value(&mut self.protocol, protocol, protocol.label());
                            }
                        });
                    ui.add_space(12.0);
                    centered_label(
                        ui,
                        FORM_WIDTH,
                        18.0,
                        egui::RichText::new(text.username).strong(),
                    );
                    ui.add(
                        egui::TextEdit::singleline(&mut self.username)
                            .hint_text(text.username_hint)
                            .desired_width(FORM_WIDTH),
                    );
                    ui.add_space(12.0);
                    centered_label(
                        ui,
                        FORM_WIDTH,
                        18.0,
                        egui::RichText::new(text.password).strong(),
                    );
                    ui.add(
                        egui::TextEdit::singleline(&mut *self.password)
                            .password(true)
                            .hint_text(text.password_hint)
                            .desired_width(FORM_WIDTH),
                    );
                });

                ui.add_space(16.0);
                let detail = self.detail_text();
                draw_connection_status(
                    ui,
                    text.connection_state(self.state),
                    self.state,
                    &detail,
                    FORM_WIDTH,
                );
                ui.add_space(18.0);
                self.action_button_rect = None;
                self.render_primary_button(ui, busy);

                ui.add_space(17.0);
                ui.separator();
                ui.add_space(7.0);
                centered_label(
                    ui,
                    FORM_WIDTH,
                    30.0,
                    egui::RichText::new(text.security_notice)
                        .small()
                        .color(egui::Color32::from_rgb(88, 99, 92)),
                );
            });
    }

    fn render_primary_button(&mut self, ui: &mut egui::Ui, busy: bool) {
        let text = self.language.catalog();
        let response = match self.state {
            ConnectionState::Connected => centered_action_button(
                ui,
                !busy,
                FORM_WIDTH,
                egui::RichText::new(text.disconnect)
                    .strong()
                    .color(egui::Color32::WHITE),
                egui::Color32::from_rgb(176, 41, 41),
            ),
            ConnectionState::Connecting | ConnectionState::Disconnecting => centered_action_button(
                ui,
                false,
                FORM_WIDTH,
                egui::RichText::new(text.wait)
                    .strong()
                    .color(egui::Color32::from_rgb(87, 97, 91)),
                egui::Color32::from_rgb(208, 216, 211),
            ),
            ConnectionState::Disconnected | ConnectionState::Error => centered_action_button(
                ui,
                true,
                FORM_WIDTH,
                egui::RichText::new(text.connect)
                    .strong()
                    .color(egui::Color32::WHITE),
                egui::Color32::from_rgb(14, 122, 68),
            ),
        };

        self.action_button_rect = Some(response.rect);
        if response.clicked() {
            self.run_primary_action();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use eframe::egui;

    use super::*;
    use crate::application::{VpnBackendError, VpnBackendResult};

    #[derive(Default)]
    struct FakeBackend {
        connected: bool,
    }

    impl VpnBackend for FakeBackend {
        fn preflight(&self) -> VpnBackendResult<()> {
            Ok(())
        }

        fn is_connected(&self) -> bool {
            self.connected
        }

        fn active_pid(&self) -> Option<u32> {
            None
        }

        fn is_managed_process(&self, _pid: u32) -> bool {
            true
        }

        fn connect(&self, _credentials: Credentials) -> VpnBackendResult<Box<dyn VpnSession>> {
            Err(VpnBackendError::new("fake backend não conecta"))
        }

        fn disconnect(&self, _pid: u32) -> VpnBackendResult<()> {
            Ok(())
        }

        fn clear_state(&self) {}
    }

    fn test_app() -> VpnApp {
        VpnApp::new(Arc::new(FakeBackend::default()))
    }

    #[test]
    fn enter_key_activates_connect_action_inside_the_fixed_window() {
        let ctx = egui::Context::default();
        let mut frame = eframe::Frame::_new_kittest();
        let mut app = test_app();
        app.server = "vpn.exemplo.org".into();
        app.username = "invalido".into();
        let screen_rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(500.0, 640.0));

        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(screen_rect),
                events: vec![egui::Event::Key {
                    key: egui::Key::Enter,
                    physical_key: Some(egui::Key::Enter),
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                }],
                ..Default::default()
            },
            |ctx| app.update(ctx, &mut frame),
        );

        assert_eq!(app.state, ConnectionState::Error);
    }

    #[test]
    fn connect_button_receives_clicks_inside_the_fixed_window() {
        let ctx = egui::Context::default();
        let mut frame = eframe::Frame::_new_kittest();
        let mut app = test_app();
        app.server = "vpn.exemplo.org".into();
        app.username = "invalido".into();
        let screen_rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(500.0, 640.0));

        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(screen_rect),
                ..Default::default()
            },
            |ctx| app.update(ctx, &mut frame),
        );
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(screen_rect),
                ..Default::default()
            },
            |ctx| app.update(ctx, &mut frame),
        );
        let button_rect = app
            .action_button_rect
            .expect("the connection button should be visible");
        assert!((button_rect.width() - FORM_WIDTH).abs() < 1.0);
        let button_center = button_rect.center();

        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(screen_rect),
                events: vec![egui::Event::PointerMoved(button_center)],
                ..Default::default()
            },
            |ctx| app.update(ctx, &mut frame),
        );
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(screen_rect),
                events: vec![egui::Event::PointerButton {
                    pos: button_center,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                }],
                ..Default::default()
            },
            |ctx| app.update(ctx, &mut frame),
        );
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(screen_rect),
                events: vec![egui::Event::PointerButton {
                    pos: button_center,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                }],
                ..Default::default()
            },
            |ctx| app.update(ctx, &mut frame),
        );

        assert_eq!(app.state, ConnectionState::Error);
    }
}
