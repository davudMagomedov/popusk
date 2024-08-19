use super::{PCommand, PExecutionError};

use crate::app::App;
use crate::id::ID;
use crate::progress::Progress;

#[derive(Debug, Clone)]
pub struct AddProgressPCMD {
    id: ID,
    progress: Progress,
}

impl AddProgressPCMD {
    pub fn new(id: ID, progress: Progress) -> Self {
        AddProgressPCMD { id, progress }
    }
}

impl PCommand for AddProgressPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        crate::core_commands::corecmd_add_progress(app.storage_mut(), self.id, self.progress)?;

        println!(
            "The progress '{}/{}' was associated with the ID {}",
            self.progress.passed(),
            self.progress.ceiling(),
            self.id
        );

        Ok(())
    }
}
