use crate::{Cli, DEFAULT_FLAKE_ROOT, FLAKE_ROOT_ENV_VAR};
use std::path::Path;
use std::{env, error, fmt};

#[derive(Debug, Copy, Clone)]
pub(crate) enum FlakeRootError {
    PathDoesNotExist,
    MissingFlakeDotNix,
}

impl fmt::Display for FlakeRootError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use FlakeRootError::*;
        match *self {
            PathDoesNotExist => write!(f, "path does not exist"),
            MissingFlakeDotNix => write!(f, "flake.nix not found"),
        }
    }
}

impl error::Error for FlakeRootError {}

fn check_flake_root(flake_root: &str) -> Result<&str, FlakeRootError> {
    let path = Path::new(flake_root);

    if !path.is_dir() {
        return Err(FlakeRootError::PathDoesNotExist);
    }

    if !path.join("flake.nix").is_file() {
        return Err(FlakeRootError::MissingFlakeDotNix);
    }

    Ok(flake_root)
}

pub(crate) fn find_flake_root(cli: &Cli) -> Result<Option<String>, FlakeRootError> {
    if let Some(flake_root) = &cli.flake_root {
        return Ok(Some(check_flake_root(flake_root)?.to_owned()));
    }

    if let Ok(env_value) = env::var(FLAKE_ROOT_ENV_VAR) {
        check_flake_root(env_value.as_str())?;
        return Ok(Some(env_value));
    }

    if let Ok(default_root) = check_flake_root(DEFAULT_FLAKE_ROOT) {
        Ok(Some(default_root.to_owned()))
    } else {
        Ok(None)
    }
}
