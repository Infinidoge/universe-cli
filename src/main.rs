use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod helpers;
use helpers::CliExit;

mod rebuild;
use rebuild::{command_quick_rebuild, command_rebuild, QuickRebuildArgs, RebuildArgs};

mod update;
use update::command_update;

mod misc;
use misc::{command_cd, command_edit};

const DEFAULT_FLAKE_ROOT: &str = "/etc/nixos";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    flake_root: Option<String>,

    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Update {
        inputs: Option<Vec<String>>,
    },
    Rebuild {
        #[command(flatten)]
        rebuild_args: RebuildArgs,
    },
    Edit {
        path: PathBuf,
    },
    Cd {
        path: Option<PathBuf>,
    },

    // Shortcuts
    // Quick Rebuild
    B {
        #[command(flatten)]
        rebuild_args: QuickRebuildArgs,
    },
}

fn main() -> CliExit {
    let cli = Cli::parse();

    match cli.command {
        Commands::Update { ref inputs } => command_update(&cli, &inputs),
        Commands::Rebuild { ref rebuild_args } => command_rebuild(&cli, rebuild_args),
        Commands::B { ref rebuild_args } => command_quick_rebuild(&cli, rebuild_args),
        Commands::Edit { ref path } => command_edit(&cli, path),
        Commands::Cd { ref path } => command_cd(&cli, path),
    }
    .into()
}
