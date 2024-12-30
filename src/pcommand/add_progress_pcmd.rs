use super::{PCommand, PExecutionError};

use crate::app::App;
use crate::types::{Progress, ID, LibEntityMetaError, LibEntityMut};
use crate::library::LibraryError;
use crate::error_ext::CommonizeResultExt;

use thiserror::Error as ThisError;

type CMDResult<T> = Result<T, AddProgressError>;

#[derive(Debug, ThisError)]
enum AddProgressError {
    #[error("could not find library entity with ID {id}")]
    CouldNotFindLibEntity { id: ID },

    #[error("library entity: {0}")]
    LibEntity(#[from] LibEntityMetaError),
    #[error("library: {0}")]
    Library(#[from] LibraryError),
}

/// UNSAFE COMMAND
#[derive(Debug, Clone)]
pub struct AddProgressPCMD {
    id: ID,
    progress: Progress,
}

impl AddProgressPCMD {
    pub fn new(id: ID, progress: Progress) -> Self {
        AddProgressPCMD { id, progress }
    }

    fn get_libentity(&self, app: &mut App) -> CMDResult<LibEntityMut> {
        match app.library_mut().get_libentity_mut_by_id(self.id)? {
            Some(libentity) => Ok(libentity),
            None => Err(AddProgressError::CouldNotFindLibEntity { id: self.id })
        }
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<()> {
        let mut libentity = self.get_libentity(app)?;

        libentity.set_progress(Some(self.progress))?;
        libentity.dump_to_storage()?;

        Ok(())
    }

    fn print_info_msg(&self) {
        println!(
            "The progress '{}/{}' was associated with the ID {}",
            self.progress.passed(),
            self.progress.ceiling(),
            self.id
        );
    }
}

impl PCommand for AddProgressPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        self.execute_inner(app).commonize()?;
        self.print_info_msg();

        Ok(())
    }
}
