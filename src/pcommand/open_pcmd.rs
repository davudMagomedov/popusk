use super::{PCommand, PExecutionError};

use crate::app::App;
use crate::entity_base::EntityType;
use crate::error_ext::ComError;
use crate::scripts::Context;

use std::io::{stdin, stdout, Error as IoError, Write};
use std::path::PathBuf;

fn read_input_stdin() -> Result<String, IoError> {
    let mut string = String::new();
    stdin().read_line(&mut string)?;

    Ok(string.trim().to_string())
}

fn write_stdout(string: &str) -> Result<(), IoError> {
    stdout().write_all(string.as_bytes())?;
    stdout().flush()?;

    Ok(())
}

#[derive(Debug, Clone)]
pub struct OpenPCMD {
    path: PathBuf,
    just_look: bool,
}

impl OpenPCMD {
    pub fn new(path: PathBuf, just_look: bool) -> Self {
        OpenPCMD { path, just_look }
    }
}

impl PCommand for OpenPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let libentity = match app.library().get_libentity(self.path.clone())? {
            Some(libentity) => libentity,
            None => {
                return Err(ComError::from(format!(
                    "there's no libentity with path '{}'",
                    self.path.to_string_lossy()
                ))
                .into())
            }
        };
        let context = match Context::auto() {
            Some(context) => context,
            None => {
                return Err(
                    ComError::from(format!("couldn't make context (Context object)")).into(),
                )
            }
        };

        let libentity_id = libentity.id();
        if libentity.etype() == EntityType::Document {
            let new_progress = app.scripts().open_libentity(libentity, context)?;
            unsafe { app.library_mut().storage_mut() }
                .update_progress(libentity_id, new_progress)?;
        }

        Ok(())
    }
}
