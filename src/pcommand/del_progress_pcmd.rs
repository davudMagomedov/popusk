use super::{PCommand, PExecutionError};

use crate::app::App;
use crate::comps_appearance::progress_to_string;
use crate::types::ID;

#[derive(Debug, Clone)]
pub struct DelProgressPCMD {
    id: ID,
}

impl DelProgressPCMD {
    pub fn new(id: ID) -> Self {
        DelProgressPCMD { id }
    }
}

impl PCommand for DelProgressPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let deleted_progress = crate::core_commands::corecmd_del_progress(
            unsafe { app.library_mut().storage_mut() },
            self.id,
        )?;

        println!(
            "Progress {} was detached from the {} id",
            progress_to_string(&deleted_progress),
            self.id
        );

        Ok(())
    }
}
