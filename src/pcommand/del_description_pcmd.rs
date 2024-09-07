use crate::app::App;
use crate::id::ID;

use super::{PCommand, PExecutionError};

#[derive(Debug, Clone)]
pub struct DelDescriptionPCMD {
    id: ID,
}

impl DelDescriptionPCMD {
    pub fn new(id: ID) -> Self {
        DelDescriptionPCMD { id }
    }
}

impl PCommand for DelDescriptionPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let description = crate::core_commands::corecmd_del_description(
            unsafe { app.library_mut().storage_mut() },
            self.id,
        )?;

        println!("{}", description);

        Ok(())
    }
}
