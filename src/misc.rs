use std::path::{Path, PathBuf};
use std::process::Command;

use clap::Args;
use edit::edit_file;

use crate::helpers::{find_flake_root, CliResult, UniverseCliError};
use crate::Cli;

impl From<std::io::Error> for UniverseCliError {
    fn from(value: std::io::Error) -> Self {
        UniverseCliError::IoError(value.kind())
    }
}

pub(crate) fn command_edit(cli: &Cli, path: &PathBuf) -> CliResult<()> {
    let path = if let Some(base_path) = find_flake_root(cli)? {
        Path::new(base_path).join(path)
    } else {
        Path::new(path).to_path_buf()
    };

    edit_file(path)?;

    Ok(())
}

pub(crate) fn command_cd(cli: &Cli, path: &Option<PathBuf>) -> CliResult<()> {
    let Some(base_path) = find_flake_root(cli)? else {
        return Ok(());
    };

    let base_path = Path::new(base_path);

    let path = if let Some(path) = path {
        base_path.join(path)
    } else {
        base_path.to_path_buf()
    };

    if path.is_dir() {
        println!("{}", path.canonicalize()?.as_os_str().to_str().unwrap());
        Ok(())
    } else {
        Err(UniverseCliError::InvalidDirectory)
    }
}

#[derive(Debug, Clone, Args)]
pub(crate) struct GcArgs {
    #[arg(long, short = 'd')]
    /// Delete all old generations for all profiles
    delete_old: bool,

    /// Delete generations older than the given period
    delete_older_than: Option<String>,

    dry_run: bool,
}

fn add_gc_arguments(command: &mut Command, cli: &Cli, gc_args: &GcArgs) {
    if cli.quiet {
        command.arg("--quiet");
    } else if cli.verbose {
        command.arg("--verbose");
    }

    if gc_args.delete_old {
        command.arg("--delete-old");
    }

    if gc_args.dry_run {
        command.arg("--dry-run");
    }

    if let Some(period) = &gc_args.delete_older_than {
        command.arg("--delete-older-than");
        command.arg(period);
    }
}

pub(crate) fn command_gc(cli: &Cli, gc_args: &GcArgs) -> CliResult<()> {
    let mut user = Command::new("nix-collect-garbage");
    let mut root = Command::new("sudo"); // HACK: Hard-codes using sudo, should offer a way to use some other kind of privledge escalation
    root.arg("nix-collect-garbage");

    add_gc_arguments(&mut user, cli, gc_args);
    add_gc_arguments(&mut root, cli, gc_args);

    let _user_result = user.spawn().unwrap().wait().unwrap();
    let _root_result = user.spawn().unwrap().wait().unwrap();

    Ok(())
}
