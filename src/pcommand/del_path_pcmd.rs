use super::{PCommand, PExecutionError};

use crate::app::App;
use crate::storage::StorageError;
use crate::types::ID;
use crate::error_ext::CommonizeResultExt;

use std::path::PathBuf;

use thiserror::Error as ThisError;

type CMDResult<T, E = DelPathError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum DelPathError {
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

    fn execute_inner(&self, app: &mut App) -> CMDResult<ID> {
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
