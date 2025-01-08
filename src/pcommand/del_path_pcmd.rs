use super::{PCommand, PExecutionError};

use crate::app::App;
use crate::storage::StorageError;
use crate::types::ID;
use crate::error_ext::CommonizeResultExt;

use std::path::PathBuf;

use thiserror::Error as ThisError;

type CMDResult<T, E = CMDError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum CMDError {
    #[error("path '{path}' was not found")]
    PathWasNotFound { path: PathBuf },

    #[error("storage: {0}")]
    Storage(#[from] StorageError),
}

/// UNSAFE COMMAND
#[derive(Debug, Clone)]
pub struct DelPathPCMD {
    path: PathBuf,
}

impl DelPathPCMD {
    pub fn new(path: PathBuf) -> Self {
        DelPathPCMD { path }
    }

    fn validation_check(&self, app: &App) -> CMDResult<()> {
        let storage_own = app.library().storage();
        let storage = storage_own.borrow();

        if storage.get_id(self.path.clone())?.is_none() {
            return Err(CMDError::PathWasNotFound { path: self.path.clone() });
        }

        Ok(())
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<ID> {
        self.validation_check(app)?;

        let id = app.library_mut()
            .storage().borrow_mut()
            .unlink_id_from_path(self.path.clone())?;

        Ok(id)
    }

    fn print_info_msg(&self, deleted_id: ID) {
        println!(
            "The {} ID was detached from the path '{}'",
            deleted_id,
            self.path.to_string_lossy(),
        );
    }
}

impl PCommand for DelPathPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let deleted_id = self.execute_inner(app).commonize()?;
        self.print_info_msg(deleted_id);

        Ok(())
    }
}
