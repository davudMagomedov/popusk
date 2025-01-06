use crate::app::App;
use crate::scripts::ScriptsError;
use crate::error_ext::CommonizeResultExt;
use crate::library::LibraryError;
use crate::types::{LibEntityData, EntityType, LibEntityMetaError};

use super::{PCommand, PExecutionError};

use std::path::PathBuf;

use thiserror::Error as ThisError;
use itertools::Itertools;

type CMDResult<T, E = AddLibEntityError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum AddLibEntityError {
    #[error("library entity '{path}' already exists")]
    LibEntityAlreadyExists { path: PathBuf },
    #[error("file '{path}' doesn't exist")]
    FileDoesNotExist { path: PathBuf },

    #[error("scripts: {0}")]
    Scripts(#[from] ScriptsError),
    #[error("library: {0}")]
    Library(#[from] LibraryError),
    #[error("libentity meta: {0}")]
    LibEntityMeta(#[from] LibEntityMetaError)
}

#[derive(Debug, Clone)]
pub struct AddLibentityPCMD {
    path: PathBuf,
}

impl AddLibentityPCMD {
    pub fn new(path: PathBuf) -> Self {
        AddLibentityPCMD { path }
    }

    fn fix_libentity_data(&self, mut libentity_data: LibEntityData) -> CMDResult<LibEntityData> {
        libentity_data.path = self.path.clone();
        if self.path.is_dir() { libentity_data.etype = EntityType::Section }
        libentity_data.tags = libentity_data.tags.into_iter().unique().collect();

        Ok(libentity_data)
    }

    /// Returns error if somehow the command can't be run.
    fn validation_check(&self, app: &App) -> CMDResult<()> {
        if app.library().libentity_exists(self.path.clone())? {
            return Err(AddLibEntityError::LibEntityAlreadyExists { path: self.path.clone() });
        };

        if !self.path.exists() {
            return Err(AddLibEntityError::FileDoesNotExist { path:  self.path.clone() });
        }

        Ok(())
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<()> {
        self.validation_check(app)?;

        let libentity_data = app.scripts().add_library_entity(self.path.clone())?;
        let libentity_data = self.fix_libentity_data(libentity_data)?;
        let libentity = app.library_mut().create_libentity_from_libentitydata(libentity_data)?;
        libentity.dump_to_storage()?;

        Ok(())
    }
}

impl PCommand for AddLibentityPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        self.execute_inner(app).commonize()?;

        Ok(())
    }
}
