use crate::config::{read_config, Config, ConfigError};
use crate::library::{Library, LibraryError};
use crate::scripts::{open_scripts, Scripts, ScriptsError};
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
}

/// Contains all information about application state - storage, config, etc.
pub struct App {
    library: Library,
    config: Config,
    scripts: Scripts,
}

impl App {
    pub fn open() -> Result<Self, AppError> {
        Ok(App {
            library: Library::new(Storage::open_with_working_dir(&PathBuf::from(WORKING_DIR))?),
            config: read_config()?,
            scripts: open_scripts()?,
        })
    }

    pub fn create() -> Result<Self, AppError> {
        let working_dir_path = PathBuf::from(WORKING_DIR);

        std::fs::create_dir(&working_dir_path)?;

        Ok(App {
            library: Library::new(Storage::create_with_working_dir(&working_dir_path)?),
            config: read_config()?,
            scripts: open_scripts()?,
        })
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
