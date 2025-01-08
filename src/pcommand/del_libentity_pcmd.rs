use crate::app::App;
use crate::types::{LibEntityMetaError, LibEntityMut};
use crate::library::LibraryError;
use crate::error_ext::CommonizeResultExt;

use super::{PCommand, PExecutionError};

use std::path::PathBuf;

use thiserror::Error as ThisError;

type CMDResult<T, E = CMDError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum CMDError {
    #[error("library entity with path '{path}' wasn't found")]
    LibEntityWasNotFound { path: PathBuf },

    #[error("library entity: {0}")]
    LibEntityMeta(#[from] LibEntityMetaError),
    #[error("library: {0}")]
    Library(#[from] LibraryError),
}

#[derive(Debug, Clone)]
pub struct DelLibentityPCMD {
    path: PathBuf,
}

impl DelLibentityPCMD {
    pub fn new(path: PathBuf) -> Self {
        DelLibentityPCMD { path }
    }

    fn get_libentity(&self, app: &mut App) -> CMDResult<LibEntityMut> {
        app.library_mut().get_libentity_mut(self.path.clone())?
            .ok_or_else(|| CMDError::LibEntityWasNotFound {
                path: self.path.clone()
            })
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<()> {
        self.get_libentity(app)?.delete()?;

        Ok(())
    }
}

impl PCommand for DelLibentityPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        self.execute_inner(app).commonize()?;

        println!(
            "Libentity with path '{}' was deleted",
            self.path.to_string_lossy()
        );

        Ok(())
    }
}
