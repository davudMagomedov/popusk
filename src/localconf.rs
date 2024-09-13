use crate::global_conf_directory::{configdir, GlobalConfError};

use std::fs::read_to_string as read_file_to_string;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};
use thiserror::Error;
use toml::{de::Error as TomlDEError, from_str as toml_from_str};

/// Must have a dot '.' in the start.
pub const LOCAL_CONFIG_FILENAME: &str = ".popuskconf.toml";

#[derive(Debug, Error)]
pub enum LocalConfigError {
    #[error("file wasn't found: {path}")]
    FileWasNotFound { path: PathBuf },
    #[error("error during config file processing: {err}")]
    ConfigProcessing { err: TomlDEError },
    #[error("IO error: {0}")]
    IO(#[from] IoError),
    #[error("global config directory: {0}")]
    GLobalConfError(#[from] GlobalConfError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalConfig {
    #[cfg(target_family = "unix")]
    global_config_path: Option<PathBuf>,
    #[cfg(target_family = "windows")]
    global_config_path: PathBuf,
}

impl LocalConfig {
    pub fn global_config_path(&self) -> Option<&PathBuf> {
        self.global_config_path.as_ref()
    }

    #[cfg(unix)]
    pub fn default() -> Result<LocalConfig, LocalConfigError> {
        Ok(LocalConfig {
            global_config_path: Some(configdir()?),
        })
    }

    #[cfg(target_family = "unix")]
    pub fn destruct(self) -> (Option<PathBuf>,) {
        (self.global_config_path,)
    }

    #[cfg(target_family = "windows")]
    pub fn destruct(self) -> (PathBuf,) {
        (self.global_config_path,)
    }

    /// After this function there's guarantrees that all fields that wasn't filled and can be
    /// filled will be so.
    pub fn destruct_default(mut self) -> Result<(PathBuf,), LocalConfigError> {
        self.defaultize()?;
        let LocalConfig {
            #[cfg(target_family = "unix")]
                global_config_path: Some(global_config_path),
            #[cfg(target_family = "windows")]
            global_config_path,
        } = self
        else {
            unreachable!();
        };

        Ok((global_config_path,))
    }

    pub fn defaultize(&mut self) -> Result<(), LocalConfigError> {
        #[cfg(target_family = "unix")]
        if self.global_config_path.is_none() {
            self.global_config_path = Some(configdir()?)
        }

        Ok(())
    }
}

/// Opens local-config file in given `library_path` and parse it to `LocalConfig`.
///
/// # Errors
/// 1. If `library_path + LOCAL_CONFIG_FILENAME` doesn't exist.
/// 2. If local config file has invalid data.
/// 3. If an IO error happened.
pub fn read_local_config(library_path: &Path) -> Result<LocalConfig, LocalConfigError> {
    let local_config_path = library_path.join(LOCAL_CONFIG_FILENAME);
    let file_content = match read_file_to_string(&local_config_path) {
        Ok(file_content) => file_content,
        Err(io_error) if io_error.kind() == IoErrorKind::NotFound => {
            return Err(LocalConfigError::FileWasNotFound {
                path: local_config_path,
            })
        }
        Err(io_error) => return Err(io_error.into()),
    };
    let local_config: LocalConfig = match toml_from_str(&file_content) {
        Ok(local_config) => local_config,
        Err(err) => return Err(LocalConfigError::ConfigProcessing { err }),
    };

    Ok(local_config)
}

pub fn read_local_config_or_default(library_path: &Path) -> Result<LocalConfig, LocalConfigError> {
    match read_local_config(library_path) {
        Ok(local_config) => Ok(local_config),
        #[cfg(target_family = "unix")]
        Err(LocalConfigError::FileWasNotFound { .. }) => Ok(LocalConfig::default()?),
        Err(other_error) => Err(other_error.into()),
    }
}
