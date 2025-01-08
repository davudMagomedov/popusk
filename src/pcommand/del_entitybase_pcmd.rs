use crate::app::App;
use crate::comps_appearance::entitybase_to_oneline_string;
use crate::types::{ID, EntityBase};
use crate::storage::StorageError;
use crate::error_ext::CommonizeResultExt;

use super::{PCommand, PExecutionError};

use thiserror::Error as ThisError;

type CMDResult<T, E = CMDError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum CMDError {
    #[error("library entity with ID {id} wasn't found")]
    LibEntityWasNotFound { id: ID },

    #[error("storage: {0}")]
    Storage(#[from] StorageError),
}

/// UNSAFE COMMAND
#[derive(Debug, Clone)]
pub struct DelEntitybasePCMD {
    id: ID,
}

impl DelEntitybasePCMD {
    pub fn new(id: ID) -> Self {
        DelEntitybasePCMD { id }
    }

    fn validation_check(&self, app: &App) -> CMDResult<()> {
        let storage_own = app.library().storage();
        let storage = storage_own.borrow();

        if storage.get_entitybase(self.id)?.is_none() {
            return Err(CMDError::LibEntityWasNotFound { id: self.id });
        }

        Ok(())
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<EntityBase> {
        let ebase = app.library_mut().storage().borrow_mut()
            .unlink_entitybase_from_id(self.id)?;

        Ok(ebase)
    }

    fn print_info_msg(&self, deleted_entitybase: EntityBase) {
        println!(
            "The entitybase {} was detached from the {} ID",
            entitybase_to_oneline_string(&deleted_entitybase),
            self.id
        );
    }
}

impl PCommand for DelEntitybasePCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let deleted_entitybase = self.execute_inner(app).commonize()?;
        self.print_info_msg(deleted_entitybase);

        Ok(())
    }
}
