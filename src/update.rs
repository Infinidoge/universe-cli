use crate::helpers::{build_nix_command, CliResult, UniverseCliError};
use crate::Cli;

pub(crate) fn command_update(cli: &Cli, inputs: &Option<Vec<String>>) -> CliResult<()> {
    let mut command = build_nix_command(cli)?;
    command.arg("flake").arg("update");

    if let Some(inputs) = inputs {
        println!("Updating inputs: {}", inputs.join(", "));
        command.args(inputs);
    } else {
        println!("Updating all inputs");
    }

    if !command.spawn()?.wait()?.success() {
        return Err(UniverseCliError::FailedToExecuteNix);
    };

    Ok(())
}
