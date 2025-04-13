use super::{PCommand, PExecutionError};

use popusk::app::App;
use popusk::comps_appearance::progress_to_string;
use popusk::types::{ID, Progress};
use popusk::storage::StorageError;
use popusk::error_ext::CommonizeResultExt;

use thiserror::Error as ThisError;

type CMDResult<T, E = CMDError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum CMDError {
    #[error("progress doesn't exist anyway")]
    ProgressDoesNotExist,

    #[error("storage: {0}")]
    Storage(#[from] StorageError),
}

#[derive(Debug, Clone)]
pub struct DelProgressPCMD {
    id: ID,
}

impl DelProgressPCMD {
    pub fn new(id: ID) -> Self {
        DelProgressPCMD { id }
    }

    fn progress_exists(&self, app: &mut App) -> CMDResult<bool> {
        Ok(app.library_mut().storage().borrow_mut().get_progress(self.id)?.is_some())
    }

    fn delete_progress(&self, app: &mut App) -> CMDResult<Progress> {
        Ok(app.library_mut().storage().borrow_mut().unlink_progress_from_id(self.id)?)
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<Progress> {
        if self.progress_exists(app)? {
            Ok(self.delete_progress(app)?)
        } else {
            Err(CMDError::ProgressDoesNotExist)
        }
    }

    fn print_info_msg(&self, deleted_progress: Progress) {
        println!(
            "Progress {} was detached from the {} id",
            progress_to_string(&deleted_progress),
            self.id
        );
    }
}

impl PCommand for DelProgressPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let deleted_progress = self.execute_inner(app).commonize()?;
        self.print_info_msg(deleted_progress);

        Ok(())
    }
}
