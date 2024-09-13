use crate::config::{read_config_from_directory, Config, ConfigError};
use crate::library::{Library, LibraryError};
use crate::localconf::{read_local_config_or_default, LocalConfigError};
use crate::scripts::{open_scripts_from_directory, Scripts, ScriptsError};
use crate::storage::{Storage, StorageError};

use std::io::Error as IoError;
use std::path::PathBuf;

use thiserror::Error;

const WORKING_DIR: &str = ".popusk";

#[derive(Debug, Error)]
pub enum AppError {
    #[error("library error: {0}")]
    LibraryError(#[from] LibraryError),
    #[error("storage error: {0}")]
    StorageError(#[from] StorageError),
    #[error("config: {0}")]
    ConfigError(#[from] ConfigError),
    #[error("scripts: {0}")]
    ScriptsError(#[from] ScriptsError),
    #[error("an I/O error occured: {0}")]
    IO(#[from] IoError),
    #[error("local config error: {0}")]
    LocalConfigError(#[from] LocalConfigError),
}

/// Contains all information about application state - storage, config, etc.
pub struct App {
    library: Library,
    config: Config,
    scripts: Scripts,
}

impl App {
    pub fn open() -> Result<Self, AppError> {
        let (global_config_path,) =
            read_local_config_or_default(&PathBuf::from("."))?.destruct_default()?;

        Ok(App {
            library: Library::new(Storage::open_with_working_dir(&PathBuf::from(WORKING_DIR))?),
            config: read_config_from_directory(&global_config_path)?,
            scripts: open_scripts_from_directory(&global_config_path)?,
        })
    }

    pub fn create() -> Result<(), AppError> {
        Storage::create_with_working_dir(&PathBuf::from(WORKING_DIR))?;
        Ok(())
    }

    pub fn library(&self) -> &Library {
        &self.library
    }

    pub fn library_mut(&mut self) -> &mut Library {
        &mut self.library
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn scripts(&self) -> &Scripts {
        &self.scripts
    }
}
