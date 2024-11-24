use crate::library::{Library, LibraryError};
use crate::localconf::{
    read_filled_local_config, LocalConfigError, LocalConfigFilled,
};
use crate::scripts::{open_scripts_from_directory, Scripts, ScriptsError};
use crate::storage::{Storage, StorageError};

use std::path::Path;

use thiserror::Error;

const WORKING_DIR: &str = ".popusk";

#[derive(Debug, Error)]
pub enum AppError {
    #[error("library error: {0}")]
    LibraryError(#[from] LibraryError),
    #[error("storage error: {0}")]
    StorageError(#[from] StorageError),
    #[error("scripts: {0}")]
    ScriptsError(#[from] ScriptsError),
    #[error("local config error: {0}")]
    LocalConfigError(#[from] LocalConfigError),
}

/// Contains all information about application state - storage, config, etc.
pub struct App {
    library: Library,
    scripts: Scripts,
    local_config: LocalConfigFilled,
}

impl App {
    // alias to `App::open_with(CURRENT_DIR)`
    pub fn open() -> Result<Self, AppError> {
        App::open_with(Path::new("."))
    }

    // alias to `App::create_with(CURRENT_DIR)`
    pub fn create() -> Result<(), AppError> {
        App::create_with(Path::new("."))
    }

    pub fn open_with(library_path: &Path) -> Result<Self, AppError> {
        let local_config = read_filled_local_config(library_path)?;
        let storage = Storage::open_with_working_dir(&library_path.join(WORKING_DIR))?;

        Ok(App {
            library: Library::new(storage),
            scripts: open_scripts_from_directory(local_config.global_config_path())?,
            local_config,
        })
    }

    pub fn create_with(library_path: &Path) -> Result<(), AppError> {
        let _storage = Storage::create_with_working_dir(&library_path.join(WORKING_DIR))?;
        Ok(())
    }

    pub fn library(&self) -> &Library {
        &self.library
    }

    pub fn library_mut(&mut self) -> &mut Library {
        &mut self.library
    }

    pub fn scripts(&self) -> &Scripts {
        &self.scripts
    }
}
