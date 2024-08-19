use std::env::var as env_var;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GlobalConfError {
    #[error("home directory was not found ($HOME isn't defined)")]
    HomeDirectoryWasNotFound,
}

fn homedir() -> Result<PathBuf, GlobalConfError> {
    match env_var("HOME") {
        Ok(homedir) => Ok(PathBuf::from(homedir)),
        Err(_) => Err(GlobalConfError::HomeDirectoryWasNotFound),
    }
}

pub fn configdir() -> Result<PathBuf, GlobalConfError> {
    Ok(homedir()?.join(format!(".config/{}", env!("CARGO_PKG_NAME"))))
}
