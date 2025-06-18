use crate::entity_data::{EDError, EntityData};
use crate::library::Library;
use crate::localconf::{read_local_config, LCError, LocalConfig};
use crate::scripts::{open_scripts_from_directory, Scripts, ScriptsError};

use std::path::Path;

use thiserror::Error;

pub const WORKING_DIR: &str = ".popusk";
pub const ED_WORKING_SUBDIR: &str = "eds";
/// Is toml file. Must have dot in the beginning.
pub const LOCAL_CONFIG_FNAME: &str = ".popuskconf.toml";

#[derive(Debug, Error)]
pub enum AppError {
    #[error("storage error: {0}")]
    EDError(#[from] EDError),
    #[error("scripts: {0}")]
    ScriptsError(#[from] ScriptsError),
    #[error("local config error: {0}")]
    LocalConfigError(#[from] LCError),
}

/// Contains all information about application state - storage, config, etc.
pub struct App {
    library: Library,
    scripts: Scripts,
    #[allow(dead_code)]
    local_config: LocalConfig,
}

impl App {
    pub fn new(library_path: &Path) -> Result<Self, AppError> {
        let local_config = read_local_config(&library_path.join(LOCAL_CONFIG_FNAME))?;
        let entity_data = EntityData::new(library_path.join(WORKING_DIR).join(ED_WORKING_SUBDIR));

        Ok(App {
            library: Library::new(entity_data),
            scripts: open_scripts_from_directory(local_config.config_path())?,
            local_config,
        })
    }

    pub fn create_new(library_path: &Path) -> Result<(), AppError> {
        let _entity_data =
            EntityData::create_new(library_path.join(WORKING_DIR).join(ED_WORKING_SUBDIR))?;
        Ok(())
    }

    pub fn library(&self) -> &Library {
        &self.library
    }

    pub fn library_mut(&mut self) -> &mut Library {
        &mut self.library
    }

    pub fn local_config(&self) -> &LocalConfig {
        &self.local_config
    }

    pub fn scripts(&self) -> &Scripts {
        &self.scripts
    }
}
