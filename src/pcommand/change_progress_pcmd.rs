use crate::app::App;
use crate::comps_appearance::progress_to_string;
use crate::error_ext::ComError;
use crate::id::ID;
use crate::progress_update::ProgressUpdate;

use super::{PCommand, PExecutionError};

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
}

impl PCommand for ChangeProgressPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let mut progress = match unsafe { app.library().storage() }.get_progress(self.id)? {
            Some(progress) => progress,
            None => {
                return Err(
                    ComError::from(format!("couldn't find progress for ID {}", self.id)).into(),
                )
            }
        };

        self.progress_update.execute_for(&mut progress)?;

        unsafe { app.library_mut().storage_mut() }.update_progress(self.id, progress)?;

        println!(
            "The progress was updated to {}",
            progress_to_string(&progress)
        );

        Ok(())
    }
}
