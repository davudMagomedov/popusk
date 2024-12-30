use crate::app::App;
use crate::comps_appearance::progress_to_string;
use crate::error_ext::CommonizeResultExt;
use crate::types::{ID, Progress, LibEntityConst, LibEntityMetaError};
use crate::library::LibraryError;

use super::{PCommand, PExecutionError};

use thiserror::Error as ThisError;

type CMDResult<T, E = GetProgressError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum GetProgressError {
    #[error("library entity with ID {id} wasn't found")]
    LibEntityWasNotFound { id: ID },
    #[error("progress was not found")]
    ProgressWasNotFound,

    #[error("library entity: {0}")]
    LibEntityMeta(#[from] LibEntityMetaError),
    #[error("library: {0}")]
    Library(#[from] LibraryError)
}

#[derive(Debug, Clone)]
pub struct GetProgressPCMD {
    id: ID,
}

impl GetProgressPCMD {
    pub fn new(id: ID) -> Self {
        GetProgressPCMD { id }
    }

    fn get_libentity(&self, app: &App) -> CMDResult<LibEntityConst> {
        app.library()
            .get_libentity_by_id(self.id)?
            .ok_or_else(|| GetProgressError::LibEntityWasNotFound { id: self.id })
    }

    fn get_progress(&self, app: &App) -> CMDResult<Progress> {
        match self.get_libentity(app)?.progress()? {
            Some(progress) => Ok(progress),
            None => Err(GetProgressError::ProgressWasNotFound),
        }
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<Progress> {
        self.get_progress(app)
    }

    fn print_info_msg(&self, progress: &Progress) {
        println!("Progress: {}", progress_to_string(progress));
    }
}

impl PCommand for GetProgressPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let progress = self.execute_inner(app).commonize()?;
        self.print_info_msg(&progress);

        Ok(())
    }
}
