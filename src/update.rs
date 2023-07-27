use crate::helpers::{build_nix_command, handle_output, CliResult, UniverseCliError};
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

            let Ok(output) = command.output() else {
                return Err(UniverseCliError::FailedToExecuteNix);
            };
            handle_output(output, "Updating failed");
        }
        None => {
            println!("Updating all inputs");

            let Ok( output ) = command.arg("update").output() else {
                return Err(UniverseCliError::FailedToExecuteNix);
            };

            handle_output(output, "Updating failed");
        }
    }

    Ok(())
}
