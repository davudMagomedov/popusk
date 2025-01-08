use crate::app::App;
use crate::comps_appearance::progress_to_string;
use crate::error_ext::CommonizeResultExt;
use crate::types::{ProgressUpdate, ID, LibEntityMetaError, LibEntityMut,
                   ProgressUpdateError, Progress};
use crate::library::LibraryError;

use super::{PCommand, PExecutionError};

use thiserror::Error as ThisError;

type CMDResult<T, E = CMDError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum CMDError {
    #[error("library entity with ID {id} wasn't found")]
    LibEntityWasNotFound { id: ID },
    #[error("progress was not found for this ID")]
    ProgressWasNotFound,

    #[error("library entity: {0}")]
    LibEntityMeta(#[from] LibEntityMetaError),
    #[error("library: {0}")]
    Library(#[from] LibraryError),
    #[error("progress update: {0}")]
    ProgressUpdate(#[from] ProgressUpdateError)
}

#[derive(Debug, Clone)]
pub struct ChangeProgressPCMD {
    id: ID,
    progress_update: ProgressUpdate,
}

impl ChangeProgressPCMD {
    pub fn new(id: ID, progress_update: ProgressUpdate) -> Self {
        ChangeProgressPCMD {
            id,
            progress_update,
        }
    }

    fn get_libentity(&self, app: &mut App) -> CMDResult<LibEntityMut> {
        app.library_mut().get_libentity_mut_by_id(self.id)?
            .ok_or_else(|| CMDError::LibEntityWasNotFound { id: self.id })
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<Progress> {
        let mut libentity = self.get_libentity(app)?;

        let mut progress = libentity.progress()?
            .ok_or_else(|| CMDError::ProgressWasNotFound)?;
        self.progress_update.execute_for(&mut progress)?;

        libentity.set_progress(Some(progress))?;
        libentity.dump_to_storage()?;

        Ok(progress)
    }
}

impl PCommand for ChangeProgressPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let progress = self.execute_inner(app).commonize()?;

        println!(
            "The progress was updated to {}",
            progress_to_string(&progress)
        );

        Ok(())
    }
}
