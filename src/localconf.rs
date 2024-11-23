//! This modules provides, mainly, two structures: `LocalConfig` and `LocalConfigFilled`, they are
//! two interconnected.
//!
//! ```plain
//! LocalConfig ===+=> LocalConfigFilled
//!                |
//!             +--+
//!             |
//! Replacing what is possible
//! ```
//!
//! ## LocalConfig
//! `LocalConfig` is serializable and deserializable structure. Its fields depend on target family
//! you use ("unix" or "windows").
//!
//! Since `LocalConfig`'s fields depend on OS, it needs to be converted to independent from an OS
//! strcuture `LocalConfigFilled`.
//!
//! ## LocalConfigFilled
//! `LocalConfigFilled` is "filled" version of `LocalConfig`.

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

#[derive(Debug, Clone)]
pub struct LocalConfigFilled {
    global_config_path: PathBuf,
}

impl LocalConfigFilled {
    pub fn global_config_path(&self) -> &PathBuf {
        &self.global_config_path
    }
}

/// This is a structure-form of a local config file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalConfig {
    #[cfg(target_family = "unix")]
    global_config_path: Option<PathBuf>,

    #[cfg(target_family = "windows")]
    global_config_path: PathBuf,
}

impl LocalConfig {
    pub fn fill(self) -> Result<LocalConfigFilled, LocalConfigError> {
        #[cfg(target_family = "unix")]
        match self.global_config_path {
            Some(global_config_path) => Ok(LocalConfigFilled { global_config_path }),
            None => Ok(LocalConfigFilled {
                global_config_path: configdir()?,
            }),
        }

        #[cfg(target_family = "windows")]
        Ok(LocalConfigFilled {
            global_config_path: self.global_config_path,
        })
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

/// More like `read_local_config` but with filled local config.
pub fn read_filled_local_config(
    library_path: &Path,
) -> Result<LocalConfigFilled, LocalConfigError> {
    let local_config_path = library_path.join(LOCAL_CONFIG_FILENAME);
    let file_content = match read_file_to_string(&local_config_path) {
        Ok(file_content) => file_content,
        Err(io_error) if io_error.kind() == IoErrorKind::NotFound => {
            return Ok(LocalConfigFilled {
                global_config_path: configdir()?,
            });
        }
        Err(io_error) => return Err(io_error.into()),
    };
    let local_config_filled: LocalConfigFilled = match toml_from_str::<LocalConfig>(&file_content) {
        Ok(local_config) => local_config.fill()?,
        Err(err) => return Err(LocalConfigError::ConfigProcessing { err }),
    };

    Ok(local_config_filled)
}
