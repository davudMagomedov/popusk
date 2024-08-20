use crate::app::App;
use crate::comps_interaction::libentity_has_progress;
use crate::error_ext::ComError;

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
        let id = match app.storage_mut().get_id(self.path.clone())? {
            Some(id) => id,
            None => {
                return Err(ComError::from(format!(
                    "couldn't find ID linked to path '{}'",
                    self.path.to_string_lossy()
                ))
                .into())
            }
        };

        // Critical section {

        // FIX: The function can raise here and as a result violent the invariants.

        app.storage_mut().unlink_id_from_path(self.path.clone())?;
        let entity_base = app.storage_mut().unlink_entitybase_from_id(id)?;

        if libentity_has_progress(entity_base.etype()) {
            app.storage_mut().unlink_progress_from_id(id)?;
        }

        // }

        println!(
            "Libentity with path '{}' was deleted",
            self.path.to_string_lossy()
        );

        Ok(())
    }
}
