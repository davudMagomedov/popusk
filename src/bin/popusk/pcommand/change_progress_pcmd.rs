use super::{PCommand, PEResult, PExecError};

use popusk::app::App;
use popusk::comps_appearance::progress_to_string;
use popusk::types::{LibEntityMut, Progress, ProgressUpdate};

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ChangeProgressPCMD {
    path: PathBuf,
    progress_update: ProgressUpdate,
}

impl ChangeProgressPCMD {
    pub fn new(path: PathBuf, progress_update: ProgressUpdate) -> Self {
        ChangeProgressPCMD {
            path,
            progress_update,
        }
    }

    fn get_libentity(&self, app: &mut App) -> PEResult<LibEntityMut> {
        app.library_mut()
            .get_libentity_mut(self.path.clone())
            .ok_or_else(|| PExecError::LibEntityWasNotFound {
                entitypath: self.path.clone(),
            })
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<Progress> {
        let mut libentity = self.get_libentity(app)?;

        let mut progress =
            libentity
                .progress()
                .ok_or_else(|| PExecError::ComponentWasNotFound {
                    component: "progress",
                    entitypath: self.path.clone(),
                })?;
        self.progress_update.execute_for(&mut progress)?;

        libentity.set_progress(Some(progress));
        libentity.dump_to_storage();

        Ok(progress)
    }
}

impl PCommand for ChangeProgressPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecError> {
        let progress = self.execute_inner(app)?;

        println!(
            "The progress was updated to {}",
            progress_to_string(&progress)
        );

        Ok(())
    }
}
