use std::{
    env, fs,
    io::{Read, Write},
    path::PathBuf,
    process::{Child, Command, ExitStatus, Stdio},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use zeroize::Zeroizing;

use crate::{
    application::{
        BackendErrorMessage, SessionStatus, VpnBackend, VpnBackendError, VpnBackendResult,
        VpnSession,
    },
    domain::{Credentials, VpnProtocol},
};

const LEGACY_PID_FILE: &str = "/run/openconnect-vpn-gui.pid";
const OPENCONNECT_OBSERVATION_WINDOW: Duration = Duration::from_secs(8);
const PROCESS_POLL_INTERVAL: Duration = Duration::from_millis(200);
const COMMAND_LOG_LIMIT: usize = 8192;

#[derive(Clone, Debug)]
pub struct OpenConnectConfig {
    openconnect: &'static str,
    pkexec: &'static str,
    kill: &'static str,
}

impl Default for OpenConnectConfig {
    fn default() -> Self {
        Self {
            openconnect: "/usr/sbin/openconnect",
            pkexec: "/usr/bin/pkexec",
            kill: "/bin/kill",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct OpenConnectBackend {
    config: OpenConnectConfig,
}

impl OpenConnectBackend {
    pub fn new(config: OpenConnectConfig) -> Self {
        Self { config }
    }

    fn ensure_supported_platform(&self) -> VpnBackendResult<()> {
        if cfg!(target_os = "linux") {
            Ok(())
        } else {
            Err(VpnBackendError::localized(
                BackendErrorMessage::UnsupportedPlatform,
            ))
        }
    }

    fn start_at_endpoint(
        &self,
        server: &str,
        protocol: VpnProtocol,
        username: &str,
        password: &Zeroizing<String>,
    ) -> Result<Box<dyn VpnSession>, ConnectionAttemptError> {
        let mut child = Command::new(self.config.pkexec)
            .arg(self.config.openconnect)
            .args(openconnect_args(server, protocol, username))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|_| {
                ConnectionAttemptError::Local(VpnBackendError::localized(
                    BackendErrorMessage::SystemAuthorizationUnavailable,
                ))
            })?;

        let command_log = Arc::new(Mutex::new(Vec::new()));
        if let Some(stdout) = child.stdout.take() {
            collect_process_output(stdout, Arc::clone(&command_log));
        }
        if let Some(stderr) = child.stderr.take() {
            collect_process_output(stderr, Arc::clone(&command_log));
        }

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(password.as_bytes())
                .and_then(|_| stdin.write_all(b"\n"))
                .map_err(|_| ConnectionAttemptError::Unavailable)?;
        }

        let mut openconnect_pid = None;
        let mut observed_at = None;

        loop {
            if let Some(status) = child
                .try_wait()
                .map_err(|_| ConnectionAttemptError::Unavailable)?
            {
                return connection_exit_error(status, &command_log);
            }

            if openconnect_pid.is_none() {
                openconnect_pid = resolve_openconnect_pid(child.id());
                if openconnect_pid.is_some() {
                    observed_at = Some(Instant::now());
                }
            }

            if output_says_connected(&command_log) {
                let pid = openconnect_pid
                    .or_else(|| resolve_openconnect_pid(child.id()))
                    .unwrap_or_else(|| child.id());
                store_vpn_pid(pid).map_err(ConnectionAttemptError::Local)?;
                return Ok(Box::new(OpenConnectSession {
                    process: child,
                    pid,
                    verified: true,
                }));
            }

            if let (Some(pid), Some(observed_at)) = (openconnect_pid, observed_at)
                && observed_at.elapsed() >= OPENCONNECT_OBSERVATION_WINDOW
            {
                store_vpn_pid(pid).map_err(ConnectionAttemptError::Local)?;
                return Ok(Box::new(OpenConnectSession {
                    process: child,
                    pid,
                    verified: false,
                }));
            }

            thread::sleep(PROCESS_POLL_INTERVAL);
        }
    }
}

impl VpnBackend for OpenConnectBackend {
    fn preflight(&self) -> VpnBackendResult<()> {
        self.ensure_supported_platform()?;

        if !command_exists(self.config.openconnect) {
            return Err(VpnBackendError::localized(
                BackendErrorMessage::OpenConnectNotFound,
            ));
        }
        if !command_exists(self.config.pkexec) {
            return Err(VpnBackendError::localized(
                BackendErrorMessage::PkexecNotFound,
            ));
        }

        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.active_pid()
            .is_some_and(|pid| self.is_managed_process(pid))
    }

    fn active_pid(&self) -> Option<u32> {
        [user_pid_file(), PathBuf::from(LEGACY_PID_FILE)]
            .into_iter()
            .find_map(read_pid_file)
    }

    fn is_managed_process(&self, pid: u32) -> bool {
        is_openconnect_process(pid)
    }

    fn connect(&self, credentials: Credentials) -> VpnBackendResult<Box<dyn VpnSession>> {
        self.preflight()?;

        let (server, protocol, username, password) = credentials.into_parts();
        match self.start_at_endpoint(server.as_str(), protocol, username.as_str(), &password) {
            Ok(connection) => Ok(connection),
            Err(ConnectionAttemptError::Cancelled) => Err(VpnBackendError::localized(
                BackendErrorMessage::AuthorizationCancelled,
            )),
            Err(ConnectionAttemptError::Local(error)) => Err(error),
            Err(ConnectionAttemptError::Unavailable) => Err(VpnBackendError::localized(
                BackendErrorMessage::ConnectionUnavailable,
            )),
        }
    }

    fn disconnect(&self, pid: u32) -> VpnBackendResult<()> {
        let output = Command::new(self.config.pkexec)
            .arg(self.config.kill)
            .args(["-TERM", &pid.to_string()])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|error| {
                VpnBackendError::localized_with_detail(
                    BackendErrorMessage::PolkitStartFailed,
                    error.to_string(),
                )
            })?;

        if !output.status.success() {
            return Err(VpnBackendError::localized_with_detail(
                BackendErrorMessage::DisconnectFailed,
                command_output(&output.stderr, b""),
            ));
        }

        for _ in 0..10 {
            if !self.is_connected() {
                clear_vpn_pid();
                return Ok(());
            }
            thread::sleep(Duration::from_millis(250));
        }

        Err(VpnBackendError::localized(
            BackendErrorMessage::DisconnectStillActive,
        ))
    }

    fn clear_state(&self) {
        clear_vpn_pid();
    }
}

struct OpenConnectSession {
    process: Child,
    pid: u32,
    verified: bool,
}

impl VpnSession for OpenConnectSession {
    fn pid(&self) -> u32 {
        self.pid
    }

    fn verified(&self) -> bool {
        self.verified
    }

    fn poll(&mut self) -> VpnBackendResult<SessionStatus> {
        match self.process.try_wait() {
            Ok(Some(_)) => return Ok(SessionStatus::Exited),
            Ok(None) => {}
            Err(_) => {
                return Err(VpnBackendError::localized(
                    BackendErrorMessage::ProcessMonitoringFailed,
                ));
            }
        }

        if cfg!(target_os = "linux") && !is_openconnect_process(self.pid) {
            return Ok(SessionStatus::Exited);
        }

        Ok(SessionStatus::Active)
    }
}

enum ConnectionAttemptError {
    Cancelled,
    Local(VpnBackendError),
    Unavailable,
}

pub fn openconnect_args(server: &str, protocol: VpnProtocol, username: &str) -> Vec<String> {
    vec![
        format!("--protocol={}", protocol.openconnect_name()),
        server.to_owned(),
        "--user".to_owned(),
        username.to_owned(),
        "--passwd-on-stdin".to_owned(),
        "--non-inter".to_owned(),
    ]
}

fn command_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn user_pid_file() -> PathBuf {
    env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir)
        .join("openconnect-vpn-gui.pid")
}

fn read_pid_file(path: PathBuf) -> Option<u32> {
    let value = fs::read_to_string(path).ok()?;
    value.trim().parse().ok().filter(|pid: &u32| *pid > 1)
}

fn store_vpn_pid(pid: u32) -> VpnBackendResult<()> {
    fs::write(user_pid_file(), format!("{pid}\n")).map_err(|error| {
        VpnBackendError::localized_with_detail(
            BackendErrorMessage::StateStoreFailed,
            error.to_string(),
        )
    })
}

fn clear_vpn_pid() {
    let _ = fs::remove_file(user_pid_file());
}

fn is_openconnect_process(pid: u32) -> bool {
    if !cfg!(target_os = "linux") {
        return false;
    }

    let comm_path = format!("/proc/{pid}/comm");
    matches!(fs::read_to_string(comm_path), Ok(name) if name.trim() == "openconnect")
}

fn resolve_openconnect_pid(supervisor_pid: u32) -> Option<u32> {
    if !cfg!(target_os = "linux") {
        return Some(supervisor_pid);
    }

    if is_openconnect_process(supervisor_pid) {
        return Some(supervisor_pid);
    }

    let children_path = format!("/proc/{supervisor_pid}/task/{supervisor_pid}/children");
    let children = fs::read_to_string(children_path).ok()?;
    children
        .split_whitespace()
        .filter_map(|pid| pid.parse::<u32>().ok())
        .find_map(|pid| {
            if is_openconnect_process(pid) {
                Some(pid)
            } else {
                resolve_openconnect_pid(pid)
            }
        })
}

fn collect_process_output<R>(mut stream: R, log: Arc<Mutex<Vec<u8>>>)
where
    R: Read + Send + 'static,
{
    thread::spawn(move || {
        let mut buffer = [0_u8; 1024];
        loop {
            match stream.read(&mut buffer) {
                Ok(0) | Err(_) => break,
                Ok(count) => append_limited_log(&log, &buffer[..count]),
            }
        }
    });
}

fn append_limited_log(log: &Arc<Mutex<Vec<u8>>>, chunk: &[u8]) {
    let Ok(mut log) = log.lock() else {
        return;
    };
    log.extend_from_slice(chunk);
    if log.len() > COMMAND_LOG_LIMIT {
        let excess = log.len() - COMMAND_LOG_LIMIT;
        log.drain(..excess);
    }
}

fn output_says_connected(log: &Arc<Mutex<Vec<u8>>>) -> bool {
    let Ok(log) = log.lock() else {
        return false;
    };
    let output = String::from_utf8_lossy(&log).to_ascii_lowercase();
    output.contains("connected as")
        || output.contains("esp session established")
        || output.contains("cstp connected")
}

fn connection_exit_error<T>(
    status: ExitStatus,
    log: &Arc<Mutex<Vec<u8>>>,
) -> Result<T, ConnectionAttemptError> {
    if status.code() == Some(126) {
        return Err(ConnectionAttemptError::Cancelled);
    }

    let output = log.lock().map(|log| log.clone()).unwrap_or_default();
    if output.is_empty() {
        Err(ConnectionAttemptError::Unavailable)
    } else {
        Err(ConnectionAttemptError::Local(
            VpnBackendError::localized_with_detail(
                BackendErrorMessage::ConnectionStartFailed,
                command_output(&output, b""),
            ),
        ))
    }
}

pub fn command_error(prefix: &str, stderr: &[u8], stdout: &[u8]) -> String {
    let message = command_output(stderr, stdout);
    if message.is_empty() {
        prefix.into()
    } else {
        format!(
            "{prefix}: {}",
            message.chars().take(300).collect::<String>()
        )
    }
}

fn command_output(stderr: &[u8], stdout: &[u8]) -> String {
    let raw = String::from_utf8_lossy(stderr);
    let raw = if raw.trim().is_empty() {
        String::from_utf8_lossy(stdout)
    } else {
        raw
    };
    raw.split_whitespace().collect::<Vec<_>>().join(" ")
}
