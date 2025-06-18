use super::{PCommand, PEResult, PExecError};

use popusk::app::App;
use popusk::comps_appearance::progress_to_string;
use popusk::types::{LibEntityConst, Progress};

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GetProgressPCMD {
    path: PathBuf,
}

impl GetProgressPCMD {
    pub fn new(path: PathBuf) -> Self {
        GetProgressPCMD { path }
    }

    fn get_libentity(&self, app: &App) -> PEResult<LibEntityConst> {
        app.library()
            .get_libentity(self.path.clone())
            .ok_or_else(|| PExecError::LibEntityWasNotFound {
                entitypath: self.path.clone(),
            })
    }

    fn get_progress(&self, app: &App) -> PEResult<Progress> {
        match self.get_libentity(app)?.progress() {
            Some(progress) => Ok(progress),
            None => Err(PExecError::ComponentWasNotFound {
                component: "progress",
                entitypath: self.path.clone(),
            }),
        }
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<Progress> {
        self.get_progress(app)
    }

    fn print_info_msg(&self, progress: &Progress) {
        println!("Progress: {}", progress_to_string(progress));
    }
}

impl PCommand for GetProgressPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecError> {
        let progress = self.execute_inner(app)?;
        self.print_info_msg(&progress);

        Ok(())
    }
}
