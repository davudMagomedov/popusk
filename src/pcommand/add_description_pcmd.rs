use crate::app::App;
use crate::types::ID;
use crate::error_ext::CommonizeResultExt;
use crate::library::LibraryError;
use crate::types::{LibEntityMetaError, LibEntityMut};

use super::{PCommand, PExecutionError};

use thiserror::Error as ThisError;

type CMDResult<T, E = AddDescriptionError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum AddDescriptionError {
    #[error("library entity with ID {id} wasn't found")]
    LibEntityWasNotFound { id: ID },

    #[error("library entity: {0}")]
    LibEntityMeta(#[from] LibEntityMetaError),
    #[error("library: {0}")]
    Library(#[from] LibraryError)
}

#[derive(Debug, Clone)]
pub struct AddDescriptionPCMD {
    id: ID,
    description: String,
}

impl AddDescriptionPCMD {
    pub fn new(id: ID, description: String) -> Self {
        AddDescriptionPCMD { id, description }
    }

    fn get_libentity(&self, app: &mut App) -> CMDResult<LibEntityMut> {
        app.library_mut()
            .get_libentity_mut_by_id(self.id)?
            .ok_or_else(|| AddDescriptionError::LibEntityWasNotFound { id: self.id })
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<()> {
        let mut libentity = self.get_libentity(app)?;
        libentity.set_description(Some(self.description.clone()))?;
        libentity.dump_to_storage()?;

        Ok(())
    }
}

impl PCommand for AddDescriptionPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        self.execute_inner(app).commonize()?;
        println!("The description was associated with the ID {}", self.id);

        Ok(())
    }
}
