use crate::global_conf_directory::{configdir, GlobalConfError};

use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};
use thiserror::Error;
use toml::{de::Error as TomlDEError, from_str as toml_from_str};

const CONFIG_INDIRECTORY_FILE_NAME: &str = concat!(env!("CARGO_PKG_NAME"), ".toml");

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("global conf: {0}")]
    GlobalConfError(#[from] GlobalConfError),
    #[error("couldn't find config in the path '{path}'")]
    ConfigDoesNotExist { path: PathBuf },
    #[error("I/O error occured: {0}")]
    IO(#[from] IoError),
    #[error("couldn't parse syntax of config file (toml): {0}")]
    WrongConfigContent(#[from] TomlDEError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    document_extension: Box<[String]>,
    viewer: Box<[String]>,
}

impl Config {
    pub fn document_extension(&self) -> &[String] {
        &self.document_extension
    }

    pub fn viewer(&self) -> &[String] {
        &self.viewer
    }
}

fn configfile() -> Result<PathBuf, ConfigError> {
    Ok(configdir()?.join(CONFIG_INDIRECTORY_FILE_NAME))
}

/// alias to `read_config_from_file(&configfile()?)`
pub fn read_config() -> Result<Config, ConfigError> {
    read_config_from_file(&configfile()?)
}

/// alias to `read_config_from_file(&directory.join(CONFIG_INDIRECTORY_FILE_NAME))`
pub fn read_config_from_directory(directory: &Path) -> Result<Config, ConfigError> {
    read_config_from_file(&directory.join(CONFIG_INDIRECTORY_FILE_NAME))
}

/// Interpretes file with given path as a configuration file. If it's impossible, returns error.
pub fn read_config_from_file(configfile: &Path) -> Result<Config, ConfigError> {
    let config_strcontent = match std::fs::read_to_string(&configfile) {
        Ok(c) => c,
        Err(io_error) if io_error.kind() == IoErrorKind::NotFound => {
            return Err(ConfigError::ConfigDoesNotExist {
                path: configfile.to_owned(),
            });
        }
        Err(io_error) => return Err(io_error.into()),
    };

    let config: Config = toml_from_str(&config_strcontent)?;

    Ok(config)
}
