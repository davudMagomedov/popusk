use super::{PCommand, PExecutionError};

use crate::app::App;
use crate::comps_appearance::{progress_to_string, progress_update_from_string};
use crate::comps_interaction::libentity_has_progress;
use crate::entity_base::EntityType;
use crate::error_ext::ComError;
use crate::progress_update::ProgressUpdate;

use std::io::{stdin, stdout, Error as IoError, Write};
use std::path::PathBuf;
use std::process::Command;

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

    fn reading_session(&self, viewer: &str, addit_args: &[String]) -> Result<(), PExecutionError> {
        let exit_status = Command::new(viewer)
            .args(addit_args)
            .arg(&self.path)
            .spawn()
            .map_err(|e| ComError::from(format!("can't spawn viewer process: {e}")))?
            .wait()
            .map_err(|e| ComError::from(format!("error with the viewer process: {e}")))?;

        if exit_status.success() {
            Ok(())
        } else {
            Err(ComError::from(format!("viewer exited unsuccesfully")).into())
        }
    }

    /// Returns `Ok(Some(prog_upd))` if the entity exists and has a progress, `Ok(None)` if entity
    /// exists but doesn't have a progress, `Err(_)` if entity doesn't exists or some invariants
    /// are broken.
    fn read_progress_update(
        &self,
        libentity_etype: EntityType,
    ) -> Result<Option<ProgressUpdate>, PExecutionError> {
        if libentity_has_progress(libentity_etype) {
            write_stdout("Progress update: ")?;
            Ok(Some(progress_update_from_string(&read_input_stdin()?)?))
        } else {
            Ok(None)
        }
    }
}

impl PCommand for OpenPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let args = app.config().viewer();
        let viewer = match args.get(0) {
            Some(viewer) => viewer,
            None => {
                return Err(
                    ComError::from(format!("viewer required at least one argument - name")).into(),
                )
            }
        };
        let addit_args = &args[1..];

        self.reading_session(viewer, addit_args)?;

        let libentity = match app.library().get_libentity(self.path.clone())? {
            Some(libentity) => libentity,
            None => return Err(ComError::from(format!("couldn't find library entity")).into()),
        };
        let libentity_etype = libentity.etype();
        let mut progress = match libentity.progress() {
            Some(progress) => progress.clone(),
            None => return Ok(()),
        };

        let progress_update = match self.read_progress_update(libentity_etype)? {
            Some(progress_update) => progress_update,
            None => return Ok(()),
        };

        progress_update.execute_for(&mut progress)?;

        unsafe { app.library_mut().storage_mut() }.update_progress(libentity.id(), progress)?;

        println!(
            "The progress was updated to {}",
            progress_to_string(&progress)
        );

        Ok(())
    }
}
