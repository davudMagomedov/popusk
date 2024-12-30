use super::{PCommand, PExecutionError};

use crate::app::App;
use crate::library::LibraryError;
use crate::storage::StorageError;
use crate::error_ext::IntoBoxExt;
use crate::types::{LibEntityMetaError, ID};

use std::path::PathBuf;

use thiserror::Error as ThisError;

type CMDResult<T> = Result<T, AddPathError>;

#[derive(Debug, ThisError)]
enum AddPathError {
    #[error("library entity: {0}")]
    LibEntity(#[from] LibEntityMetaError),
    #[error("library: {0}")]
    Library(#[from] LibraryError),
    #[error("storage: {0}")]
    Storage(#[from] StorageError)
}

/// UNSAFE COMMAND
#[derive(Debug, Clone)]
pub struct AddPathPCMD {
    path: PathBuf,
}

impl AddPathPCMD {
    pub fn new(path: PathBuf) -> Self {
        AddPathPCMD { path }
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<ID> {
        let id = app.library_mut()
            .storage()
            .borrow_mut()
            .link_id_to_path(self.path.clone())?;

        Ok(id)
    }
}

impl PCommand for AddPathPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let id = self.execute_inner(app).map_err(|e| e.into_box())?;

        println!(
            "The {} ID was associated with the path {}",
            id,
            self.path.to_string_lossy()
        );

        Ok(())
    }
}
