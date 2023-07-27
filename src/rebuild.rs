use std::ffi::OsString;
use std::process::Command;

use crate::helpers::find_flake_root;
use crate::helpers::CliResult;
use crate::helpers::UniverseCliError;
use crate::Cli;
use clap::Args;
use which::which;

#[derive(Args, Debug, Clone)]
pub(crate) struct QuickRebuildArgs {
    #[arg(short, long)]
    profile_name: Option<String>,

    #[arg(short = 'c', long)]
    specialisation: Option<String>,

    #[arg(long)]
    flake: Option<String>,

    #[arg(long)]
    install_bootloader: bool,

    #[arg(long)]
    fast: bool,

    #[arg(long)]
    show_trace: bool,
}

fn build_rebuild_command(cli: &Cli) -> CliResult<Command> {
    // https://github.com/rust-lang/rust/issues/53667
    if let Err(_) = which("nixos-rebuild") {
        return Err(UniverseCliError::CommandNotFound);
    }

    let mut command = Command::new("nixos-rebuild");
    command.arg("--use-remote-sudo");

    if cli.verbose {
        command.arg("--verbose");
    }

    Ok(command)
}

// https://stackoverflow.com/questions/75611314/how-can-i-make-clap-ignore-flags-after-a-certain-subcommand
#[derive(Debug)]
pub struct RebuildArgs {
    args: Vec<OsString>,
}

impl clap::FromArgMatches for RebuildArgs {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let args = matches
            .get_raw("args")
            .unwrap_or_default()
            .map(|i| i.to_owned())
            .collect();

        Ok(Self { args })
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        *self = Self::from_arg_matches(matches)?;

        Ok(())
    }
}

impl Args for RebuildArgs {
    fn augment_args(cmd: clap::Command) -> clap::Command {
        cmd.disable_help_flag(true).arg(
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

pub(crate) fn command_rebuild(cli: &Cli, rebuild_args: &RebuildArgs) -> CliResult<()> {
    let mut command = build_rebuild_command(cli)?;

    if let Some(flake_root) = find_flake_root(cli)? {
        command.arg("--flake");
        command.arg(flake_root);
    }

    command.args(&rebuild_args.args);

    let _result = command.spawn().unwrap().wait().unwrap();

    Ok(())
}

pub(crate) fn command_quick_rebuild(cli: &Cli, rebuild_args: &QuickRebuildArgs) -> CliResult<()> {
    let mut command = build_rebuild_command(cli)?;

    if let Some(flake) = &rebuild_args.flake {
        command.arg("--flake");
        command.arg(flake);
    } else if let Some(flake_root) = find_flake_root(cli)? {
        command.arg("--flake");
        command.arg(flake_root);
    }

    command.arg("switch");

    if rebuild_args.show_trace {
        command.arg("--show-trace");
    }

    if rebuild_args.fast {
        command.arg("--fast");
    }

    if rebuild_args.install_bootloader {
        command.arg("--install-bootloader");
    }

    let _result = command.spawn().unwrap().wait().unwrap();

    Ok(())
}
