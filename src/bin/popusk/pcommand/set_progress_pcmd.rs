use super::{PCommand, PExecError, PEResult};

use popusk::app::App;
use popusk::types::{Progress, LibEntityMut};

use std::path::PathBuf;

/// UNSAFE COMMAND
#[derive(Debug, Clone)]
pub struct SetProgressPCMD {
    path: PathBuf,
    progress: Progress,
}

impl SetProgressPCMD {
    pub fn new(path: PathBuf, progress: Progress) -> Self {
        SetProgressPCMD { path, progress }
    }

    fn get_libentity(&self, app: &mut App) -> PEResult<LibEntityMut> {
        match app.library_mut().get_libentity_mut(self.path.clone()) {
            Some(libentity) => Ok(libentity),
            None => Err(PExecError::LibEntityWasNotFound { entitypath: self.path.clone() })
        }
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<()> {
        let mut libentity = self.get_libentity(app)?;

        libentity.set_progress(Some(self.progress));
        libentity.dump_to_storage();

        Ok(())
    }

    fn print_info_msg(&self) {
        println!(
            "The progress '{}/{}' was associated with the library entity '{}'",
            self.progress.passed(),
            self.progress.ceiling(),
            self.path.to_string_lossy(),
        );
    }
}

impl PCommand for SetProgressPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecError> {
        self.execute_inner(app)?;
        self.print_info_msg();

        Ok(())
    }
}
