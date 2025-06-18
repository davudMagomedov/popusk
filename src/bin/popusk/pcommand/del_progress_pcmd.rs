use super::{PCommand, PEResult, PExecError};

use popusk::app::App;
use popusk::comps_appearance::progress_to_string;
use popusk::types::{LibEntityMut, Progress};

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DelProgressPCMD {
    path: PathBuf,
}

impl DelProgressPCMD {
    pub fn new(path: PathBuf) -> Self {
        DelProgressPCMD { path }
    }

    fn get_libentity(&self, app: &mut App) -> PEResult<LibEntityMut> {
        app.library_mut()
            .get_libentity_mut(self.path.clone())
            .ok_or_else(|| PExecError::LibEntityWasNotFound {
                entitypath: self.path.clone(),
            })
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<Progress> {
        let libentity = self.get_libentity(app)?;
        match libentity.progress() {
            Some(progress) => Ok(progress),
            None => Err(PExecError::ComponentWasNotFound {
                component: "progress",
                entitypath: self.path.clone(),
            }),
        }
    }

    fn print_info_msg(&self, deleted_progress: Progress) {
        println!(
            "The progress {} was deleted",
            progress_to_string(&deleted_progress),
        );
    }
}

impl PCommand for DelProgressPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecError> {
        let deleted_progress = self.execute_inner(app)?;
        self.print_info_msg(deleted_progress);

        Ok(())
    }
}
