use std::ffi::OsString;
use std::process::Command;

use crate::helpers::{
    env, find_flake_root, is_nixos, run_command, spawn_command, CliResult, UniverseCliError,
};
use crate::Cli;
use clap::Args;
use which::which;

#[derive(Args, Debug, Clone)]
pub(crate) struct QuickRebuildArgs {
    #[arg(short, long)]
    /// Specify a profile name for the new configuration
    profile_name: Option<String>,

    #[arg(short = 'c', long)]
    /// Activate the given specialisation
    specialisation: Option<String>,

    #[arg(long)]
    /// Build a specific flake input
    flake: Option<String>,

    #[arg(long)]
    /// (Re)install the bootloader
    install_bootloader: bool,

    #[arg(long)]
    /// Skip building Nix, to rebuild faster
    fast: bool,

    #[arg(long)]
    show_trace: bool,

    #[arg(long)]
    /// Roll back to the previous configuration
    rollback: bool,

    #[arg(long)]
    offline: bool,
}

fn build_rebuild_command(cli: &Cli) -> CliResult<Command> {
    // https://github.com/rust-lang/rust/issues/53667
    if let Err(_) = which("nixos-rebuild") {
        return Err(UniverseCliError::CommandNotFound);
    }

    let mut command = Command::new("nixos-rebuild");
    command.arg("--sudo");

    if cli.verbose {
        command.arg("--verbose");
    }

    Ok(command)
}

fn build_hm_command(cli: &Cli) -> CliResult<Command> {
    if let Err(_) = which("home-manager") {
        return Err(UniverseCliError::CommandNotFound);
    }

    let mut command = Command::new("home-manager");

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

    spawn_command(command)
}

pub(crate) fn command_quick_rebuild(cli: &Cli, rebuild_args: &QuickRebuildArgs) -> CliResult<()> {
    let mut command = if is_nixos() {
        build_rebuild_command(cli)?
    } else {
        build_hm_command(cli)?
    };

    if let Some(flake) = &rebuild_args.flake {
        command.arg("--flake");
        command.arg(flake);
    } else if let Some(flake_root) = find_flake_root(cli)? {
        command.arg("--flake");
        if is_nixos() {
            command.arg(flake_root.clone());
        } else {
            let username = env("USERNAME").unwrap_or(run_command("whoami").trim_end().to_owned());
            let hostname = env("HOSTNAME").unwrap_or(run_command("hostname").trim_end().to_owned());

            command.arg(format!("{}#{}@{}", flake_root, username, hostname));
        }
    }

    command.arg("switch");

    if rebuild_args.show_trace {
        command.arg("--show-trace");
        command.arg("--no-eval-cache");
    }

    if is_nixos() {
        if rebuild_args.fast {
            command.arg("--fast");
        }

        if rebuild_args.install_bootloader {
            command.arg("--install-bootloader");
        }

        if rebuild_args.rollback {
            command.arg("--rollback");
        }

        if rebuild_args.offline {
            command.arg("--offline");
        }

        if let Some(specialisation) = &rebuild_args.specialisation {
            command.arg("--specialisation");
            command.arg(specialisation);
        }

        if let Some(profile_name) = &rebuild_args.profile_name {
            command.arg("--profile-name");
            command.arg(profile_name);
        }
    }

    spawn_command(command)
}
