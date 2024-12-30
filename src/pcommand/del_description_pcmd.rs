use crate::app::App;
use crate::types::{ID, LibEntityMetaError, LibEntityMut};
use crate::library::LibraryError;
use crate::error_ext::CommonizeResultExt;

use super::{PCommand, PExecutionError};

use thiserror::Error as ThisError;

type CMDResult<T, E = DelDescrError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum DelDescrError {
    #[error("library entity with ID {id} wasn't found")]
    LibEntityWasNotFound { id: ID },

    #[error("library entity: {0}")]
    LibEntityMeta(#[from] LibEntityMetaError),
    #[error("library: {0}")]
    Library(#[from] LibraryError),
}

#[derive(Debug, Clone)]
pub struct DelDescriptionPCMD {
    id: ID,
}

impl DelDescriptionPCMD {
    pub fn new(id: ID) -> Self {
        DelDescriptionPCMD { id }
    }

    fn get_libentity(&self, app: &mut App) -> CMDResult<LibEntityMut> {
        app.library_mut().get_libentity_mut_by_id(self.id)?
            .ok_or_else(|| DelDescrError::LibEntityWasNotFound { id: self.id })
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<()> {
        let mut libentity = self.get_libentity(app)?;

        libentity.set_description(None)?;
        libentity.dump_to_storage()?;

        Ok(())
    }
}

impl PCommand for DelDescriptionPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        self.execute_inner(app).commonize()?;

        println!("The description was deleted");

        Ok(())
    }
}
