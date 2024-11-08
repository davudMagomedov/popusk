use crate::app::App;
use crate::error_ext::ComError;
use crate::types::EntityType;

use super::{PCommand, PExecutionError};

use std::fs::rename as rename_file;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RenameLibentityPCMD {
    path: PathBuf,
    new_path: PathBuf,
}

impl RenameLibentityPCMD {
    pub fn new(path: PathBuf, new_path: PathBuf) -> Self {
        RenameLibentityPCMD { path, new_path }
    }
}

impl PCommand for RenameLibentityPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let id = match unsafe { app.library().storage() }.get_id(self.path.clone())? {
            Some(id) => id,
            None => {
                return Err(ComError::from(format!(
                    "couldn't find library entity with path '{}'",
                    self.path.to_string_lossy()
                ))
                .into())
            }
        };

        if app.library().get_etype(id)?.unwrap() == EntityType::Section {
            return Err(ComError::from(format!(
                "the library entity you're trying to rename is Section"
            ))
            .into());
        }

        rename_file(self.path.clone(), self.new_path.clone())?;

        _ = unsafe {
            app.library_mut()
                .storage_mut()
                .unlink_id_from_path_raw(self.path.clone())?
        };
        unsafe {
            app.library_mut()
                .storage_mut()
                .link_id_to_path_raw(self.new_path.clone(), id)?
        };

        println!(
            "Library entity was succesfully placed from '{}' to '{}'.",
            self.path.to_string_lossy(),
            self.new_path.to_string_lossy()
        );

        Ok(())
    }
}
