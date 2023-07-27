use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod helpers;
use helpers::CliExit;

mod rebuild;
use rebuild::{command_quick_rebuild, command_rebuild, QuickRebuildArgs, RebuildArgs};

mod update;
use update::command_update;

mod misc;
use misc::{command_cd, command_edit, command_gc, GcArgs};

const DEFAULT_FLAKE_ROOT: &str = "/etc/nixos";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    /// Path to the flake root to operate on
    flake_root: Option<String>,

    #[arg(short, long)]
    /// Pass `--verbose` to supported Nix commands
    verbose: bool,

    #[arg(long)]
    /// Pass `--quiet` to supported Nix commands
    quiet: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Update all or a subset of all flake inputs
    Update { inputs: Option<Vec<String>> },

    /// NixOS rebuild with the flake root pre-selected
    Rebuild {
        #[command(flatten)]
        rebuild_args: RebuildArgs,
    },

    /// Open a file relative to flake root in $EDITOR
    Edit { path: PathBuf },

    /// Print absolute path to the given file relative to the flake root
    Cd { path: Option<PathBuf> },

    /// Rebuild switch shortcut
    B {
        #[command(flatten)]
        rebuild_args: QuickRebuildArgs,
    },

    /// College garbage for both the current user and root
    Gc {
        #[command(flatten)]
        gc_args: GcArgs,
    },
}

fn main() -> CliExit {
    let cli = Cli::parse();

    match cli.command {
        Commands::Update { ref inputs } => command_update(&cli, inputs),
        Commands::Rebuild { ref rebuild_args } => command_rebuild(&cli, rebuild_args),
        Commands::B { ref rebuild_args } => command_quick_rebuild(&cli, rebuild_args),
        Commands::Edit { ref path } => command_edit(&cli, path),
        Commands::Cd { ref path } => command_cd(&cli, path),
        Commands::Gc { ref gc_args } => command_gc(&cli, gc_args),
    }
    .into()
}
