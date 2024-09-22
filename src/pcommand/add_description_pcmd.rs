use crate::app::App;
use crate::types::ID;

use super::{PCommand, PExecutionError};

#[derive(Debug, Clone)]
pub struct AddDescriptionPCMD {
    id: ID,
    description: String,
}

impl AddDescriptionPCMD {
    pub fn new(id: ID, description: String) -> Self {
        AddDescriptionPCMD { id, description }
    }
}

impl PCommand for AddDescriptionPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        crate::core_commands::corecmd_add_description(
            unsafe { app.library_mut().storage_mut() },
            self.id,
            self.description.clone(),
        )?;

        println!("The description was associated with the ID {}", self.id);

        Ok(())
    }
}
