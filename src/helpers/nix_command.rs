use std::process::Command;

use which::which;

use crate::helpers::{find_flake_root, CliResult, UniverseCliError};
use crate::Cli;

pub(crate) fn build_nix_command(cli: &Cli) -> CliResult<Command> {
    let mut command = Command::new("nix");

    // https://github.com/rust-lang/rust/issues/53667
    if let Err(_) = which("nix") {
        return Err(UniverseCliError::CommandNotFound);
    }

    if let Some(flake_root) = find_flake_root(cli)? {
        command.current_dir(flake_root);
    }

    if cli.verbose {
        command.arg("--verbose");
    }

    Ok(command)
}

pub(crate) fn spawn_command(mut command: Command) -> CliResult<()> {
    if !command.spawn()?.wait()?.success() {
        return Err(UniverseCliError::FailedToExecuteNix);
    };

    Ok(())
}
