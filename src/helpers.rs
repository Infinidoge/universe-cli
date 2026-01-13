use std::error;
use std::fmt;
use std::path::Path;
use std::process::{Command, ExitCode, Termination};

mod flake_root;
pub(crate) use flake_root::{find_flake_root, FlakeRootError};

mod nix_command;
pub(crate) use nix_command::{build_nix_command, spawn_command};

use crate::ENV_VAR_PREFIX;

#[derive(Debug, Clone)]
pub(crate) enum UniverseCliError {
    CommandNotFound,
    InvalidFlakeRoot(FlakeRootError),
    FlakeRootMissing,
    IoError(std::io::ErrorKind),
    FailedToExecuteNix,
    InvalidDirectory,
}

impl fmt::Display for UniverseCliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UniverseCliError::*;
        match *self {
            CommandNotFound => write!(f, "failed to execute nix: command not found"),
            InvalidFlakeRoot(e) => write!(f, "invalid flake root: {e}"),
            FlakeRootMissing => write!(f, "no flake root specified or detected"),
            FailedToExecuteNix => write!(f, "failed to execute nix"),
            IoError(e) => write!(f, "io error: {e}"),
            InvalidDirectory => write!(f, "invalid directory"),
        }
    }
}

impl error::Error for UniverseCliError {}

impl From<FlakeRootError> for UniverseCliError {
    fn from(value: FlakeRootError) -> Self {
        UniverseCliError::InvalidFlakeRoot(value)
    }
}

pub(crate) type CliResult<T> = Result<T, UniverseCliError>;

pub(crate) enum CliExit {
    Ok,
    Err(UniverseCliError),
}

impl From<CliResult<()>> for CliExit {
    fn from(value: CliResult<()>) -> Self {
        match value {
            Ok(_) => CliExit::Ok,
            Err(e) => CliExit::Err(e),
        }
    }
}

impl Termination for CliExit {
    fn report(self) -> ExitCode {
        match self {
            CliExit::Ok => ExitCode::SUCCESS,
            CliExit::Err(err) => {
                eprintln!("error: {err}");
                ExitCode::FAILURE
            }
        }
    }
}

pub(crate) fn is_nixos() -> bool {
    Path::new("/etc/NIXOS").exists()
}

pub(crate) fn env(var: &str) -> Result<String, std::env::VarError> {
    std::env::var(format!("{}_{}", ENV_VAR_PREFIX, var))
}

pub(crate) fn run_command(command: &str) -> String {
    String::from_utf8(
        Command::new(command)
            .output()
            .expect("trivial command errored on run")
            .stdout,
    )
    .expect("trivial command returned invalid string")
}
