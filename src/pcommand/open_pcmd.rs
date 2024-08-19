use super::{PCommand, PExecutionError};

use crate::app::App;
use crate::comps_appearance::{progress_to_string, progress_update_from_string};
use crate::comps_interaction::libentity_has_progress;
use crate::id::ID;
use crate::progress_update::ProgressUpdate;

use std::io::{stdin, stdout, Error as IoError, Write};
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context};

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
            .with_context(|| "can't spawn viewer process")?
            .wait()
            .with_context(|| "error with the viewer process")?;

        if exit_status.success() {
            Ok(())
        } else {
            Err(anyhow!("viewer exited unsuccesfully").into())
        }
    }

    /// Returns `Ok(Some(prog_upd))` if the entity exists and has a progress, `Ok(None)` if entity
    /// exists but doesn't have a progress, `Err(_)` if entity doesn't exists or some invariants
    /// are broken.
    fn read_progress_update(
        &self,
        entity_id: ID,
        app: &App,
    ) -> Result<Option<ProgressUpdate>, PExecutionError> {
        let entitybase = match app.storage().get_entitybase(entity_id)? {
            Some(entitybase) => entitybase,
            None => {
                return Err(anyhow!(
                    "couldn't find entitybase for id {} (the invariants were broken)",
                    entity_id
                )
                .into())
            }
        };

        if libentity_has_progress(entitybase.etype()) {
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
            None => return Err(anyhow!("viewer required at least one argument - name").into()),
        };
        let addit_args = &args[1..];

        self.reading_session(viewer, addit_args)?;

        let entity_id = match app.storage().get_id(self.path.clone())? {
            Some(entity_id) => entity_id,
            None => {
                return Err(anyhow!(
                    "the entity with the path '{}' doesn't exist",
                    self.path.to_string_lossy()
                )
                .into())
            }
        };

        let progress_update = match self.read_progress_update(entity_id, app)? {
            Some(progress_update) => progress_update,
            None => return Ok(()),
        };

        let mut progress = match app.storage().get_progress(entity_id)? {
            Some(progress) => progress,
            None => {
                return Err(anyhow!(
                    "progress wasn't found for the entity that supposed to have one"
                )
                .into())
            }
        };

        progress_update.execute_for(&mut progress)?;
        app.storage_mut().update_progress(entity_id, progress)?;

        println!(
            "The progress was updated to {}",
            progress_to_string(&progress)
        );

        Ok(())
    }
}
