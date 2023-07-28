use std::ffi::OsString;

use clap::Args;

use crate::helpers::{build_nix_command, find_flake_root, CliResult, UniverseCliError};
use crate::Cli;

#[derive(Debug)]
pub struct RunArgs {
    flake_output: OsString,

    args: Vec<OsString>,
}

impl clap::FromArgMatches for RunArgs {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let flake_output = matches
            .get_raw("flake_output")
            .and_then(|mut flake_output| flake_output.next())
            .map(|flake_output| flake_output.to_owned())
            .ok_or(clap::Error::new(
                clap::error::ErrorKind::MissingRequiredArgument,
            ))?;

        let args = matches
            .get_raw("args")
            .unwrap_or_default()
            .map(|i| i.to_owned())
            .collect();

        Ok(Self { flake_output, args })
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        *self = Self::from_arg_matches(matches)?;

        Ok(())
    }
}

impl Args for RunArgs {
    fn augment_args(cmd: clap::Command) -> clap::Command {
        cmd.arg(clap::Arg::new("flake_output").required(true)).arg(
            clap::Arg::new("args")
                .action(clap::ArgAction::Append)
                .allow_hyphen_values(true)
                .trailing_var_arg(true),
        )
    }

    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        Self::augment_args(cmd)
    }
}

pub(crate) fn command_run(cli: &Cli, run_args: &RunArgs) -> CliResult<()> {
    let mut command = build_nix_command(cli)?;
    command.arg("run");

    let Some(flake_root) = find_flake_root(cli)? else {
        return Err(UniverseCliError::FlakeRootMissing);
    };

    command.arg(format!(
        "{}#{}",
        flake_root,
        run_args.flake_output.as_os_str().to_str().unwrap()
    ));

    command.args(&run_args.args);

    if !command.spawn()?.wait()?.success() {
        return Err(UniverseCliError::FailedToExecuteNix);
    };

    Ok(())
}
