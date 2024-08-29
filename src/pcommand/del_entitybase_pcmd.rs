use super::{PCommand, PExecutionError};

use crate::app::App;
use crate::comps_appearance::entitybase_to_oneline_string;
use crate::id::ID;

/// UNSAFE COMMAND
#[derive(Debug, Clone)]
pub struct DelEntitybasePCMD {
    id: ID,
}

impl DelEntitybasePCMD {
    pub fn new(id: ID) -> Self {
        DelEntitybasePCMD { id }
    }
}

impl PCommand for DelEntitybasePCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let deleted_entitybase = crate::core_commands::corecmd_del_entitybase(
            unsafe { app.library_mut().storage_mut() },
            self.id,
        )?;

        println!(
            "The entitybase {} was detached from the {} ID",
            entitybase_to_oneline_string(&deleted_entitybase),
            self.id
        );

        Ok(())
    }
}
