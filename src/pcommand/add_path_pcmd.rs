use super::{PCommand, PExecutionError};

use crate::app::App;

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AddPathPCMD {
    path: PathBuf,
}

impl AddPathPCMD {
    pub fn new(path: PathBuf) -> Self {
        AddPathPCMD { path }
    }
}

impl PCommand for AddPathPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let id = crate::core_commands::corecmd_add_path(app.storage_mut(), self.path.clone())?;

        println!(
            "The {} ID was associated with the path {}",
            id,
            self.path.to_string_lossy()
        );

        Ok(())
    }
}
