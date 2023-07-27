use crate::helpers::{build_nix_command, CliResult, UniverseCliError};
use crate::Cli;

pub(crate) fn command_update(cli: &Cli, inputs: &Option<Vec<String>>) -> CliResult<()> {
    let mut command = build_nix_command(cli)?;
    command.arg("flake");

    match inputs {
        Some(inputs) => {
            println!("Updating inputs: {}", inputs.join(", "));

            let input_args = inputs
                .into_iter()
                .flat_map(|input| vec!["--update-input".to_owned(), input.to_owned()])
                .collect::<Vec<String>>();

            command.arg("lock").args(input_args);

            if !command.spawn()?.wait()?.success() {
                return Err(UniverseCliError::FailedToExecuteNix);
            };
        }
        None => {
            println!("Updating all inputs");

            command.arg("update");

            if !command.spawn()?.wait()?.success() {
                return Err(UniverseCliError::FailedToExecuteNix);
            };
        }
    }

    Ok(())
}
