use super::{PCommand, PExecutionError};

use crate::app::App;

use std::path::PathBuf;

/// UNSAFE COMMAND
#[derive(Debug, Clone)]
pub struct DelPathPCMD {
    path: PathBuf,
}

impl DelPathPCMD {
    pub fn new(path: PathBuf) -> Self {
        DelPathPCMD { path }
    }
}

impl PCommand for DelPathPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let deleted_id = crate::core_commands::corecmd_del_path(
            unsafe { app.library_mut().storage_mut() },
            self.path.clone(),
        )?;

        println!(
            "The {} ID was detached from the path '{}'",
            deleted_id,
            self.path.to_string_lossy(),
        );

        Ok(())
    }
}
