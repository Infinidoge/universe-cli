use std::path::{Path, PathBuf};

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
