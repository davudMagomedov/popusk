//! This modules provides, mainly, two structures: `LocalConfigRaw` and `LocalConfig`, they are two
//! interconnected.
//!
//! ```plain
//! LocalConfigRaw ==+==> LocalConfig
//!                  |
//!             +----+
//!             |
//! Replacing what is possible
//! ```
//!
//! ## LocalConfig
//! `LocalConfigRaw` is serializable and deserializable structure. Its fields depend on target family
//! you use ("unix" or "windows").
//!
//! Since `LocalConfigRaw`'s fields depend on OS, it needs to be converted to independent from an OS
//! structure `LocalConfig`.
//!
//! ## LocalConfig
//! `LocalConfig` is "filled" version of `LocalConfigRaw`. For example, field
//! `LocalConfigRaw.global_config_path` during turning to `LocalConfig` will be filled with the
//! default value for unix systems '$HOME/.config/popusk'.

use crate::custom_fs::{open_readfile, FSError};
use crate::error_ext::ResultExt;
use crate::global_conf_directory::{configdir, GlobalConfError};

use std::fs::File;
use std::io::{ErrorKind as IoErrorKind, Read};
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};
use thiserror::Error;
use toml::{de::Error as TomlDEError, from_str as toml_from_str};

type LCResult<T> = Result<T, LCError>;

#[derive(Debug, Error)]
pub enum LCError {
    #[error("error during processing local config file {lc_file}: {err}")]
    ConfigProcessing { lc_file: PathBuf, err: TomlDEError },
    #[error("{0}")]
    FS(#[from] FSError),
    #[error(
        "config does not exist
  Create {lc_file} and fill necessary fields
  Read documentation for more information"
    )]
    ConfigNotFound { lc_file: PathBuf },
    #[error("{0}")]
    GLobalConfError(#[from] GlobalConfError),
}

#[derive(Debug, Clone)]
pub struct LocalConfig {
    config_path: PathBuf,
}

impl LocalConfig {
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }
}

/// This is a structure-form of a local config file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalConfigRaw {
    #[cfg(target_family = "unix")]
    config_path: Option<PathBuf>,

    #[cfg(target_family = "windows")]
    config_path: PathBuf,
}

impl LocalConfigRaw {
    pub fn fill(self) -> Result<LocalConfig, LCError> {
        #[cfg(target_family = "unix")]
        match self.config_path {
            Some(global_config_path) => Ok(LocalConfig {
                config_path: global_config_path,
            }),
            None => Ok(LocalConfig {
                config_path: configdir()?,
            }),
        }

        #[cfg(target_family = "windows")]
        Ok(LocalConfig {
            config_path: self.config_path,
        })
    }
}

fn open_lc_file(lc_path: &Path) -> LCResult<File> {
    match open_readfile(lc_path) {
        Ok(file) => Ok(file),
        Err(FSError { ioerr, .. }) if ioerr.kind() == IoErrorKind::NotFound => {
            Err(LCError::ConfigNotFound {
                lc_file: lc_path.to_path_buf(),
            })
        }
        Err(fserr) => Err(fserr.into()),
    }
}

/// Opens local-config file and parses it to `LocalConfigRaw`.
pub fn read_local_config_raw(lc_path: &Path) -> LCResult<LocalConfigRaw> {
    let mut file = open_lc_file(lc_path)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .unwrap_or_explosion_with(|| format!("reading {}", lc_path.to_string_lossy(),));
    let lc_raw: LocalConfigRaw = match toml_from_str(&file_content) {
        Ok(v) => v,
        Err(toml_err) => {
            return Err(LCError::ConfigProcessing {
                lc_file: lc_path.to_path_buf(),
                err: toml_err,
            })
        }
    };
    Ok(lc_raw)
}

pub fn read_local_config(local_config_path: &Path) -> LCResult<LocalConfig> {
    // TODO: delete this as soon as unix-version LocalConfigRaw acquires non-optional field.
    #[cfg(unix)]
    if !local_config_path.exists() {
        return Ok(LocalConfig {
            config_path: configdir()?,
        });
    }

    read_local_config_raw(local_config_path)?.fill()
}
