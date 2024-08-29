use crate::app::App;

use super::{PCommand, PExecutionError};

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DelLibentityPCMD {
    path: PathBuf,
}

impl DelLibentityPCMD {
    pub fn new(path: PathBuf) -> Self {
        DelLibentityPCMD { path }
    }
}

impl PCommand for DelLibentityPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        app.library_mut().del_libentity(self.path.clone())?;

        println!(
            "Libentity with path '{}' was deleted",
            self.path.to_string_lossy()
        );

        Ok(())
    }
}
