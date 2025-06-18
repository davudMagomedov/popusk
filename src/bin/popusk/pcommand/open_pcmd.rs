use super::{PCommand, PEResult, PExecError};

use popusk::app::App;
use popusk::scripts::Context;
use popusk::types::{LibEntityMut, Progress};

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct OpenPCMD {
    path: PathBuf,
    just_look: bool,
}

impl OpenPCMD {
    pub fn new(path: PathBuf, just_look: bool) -> Self {
        OpenPCMD { path, just_look }
    }

    fn get_libentity(&self, app: &mut App) -> PEResult<LibEntityMut> {
        match app.library_mut().get_libentity_mut(self.path.clone()) {
            Some(libentity) => Ok(libentity),
            None => Err(PExecError::LibEntityWasNotFound {
                entitypath: self.path.clone(),
            }),
        }
    }

    fn make_context(&self, _app: &App) -> PEResult<Context> {
        match Context::auto() {
            Some(context) => Ok(context),
            None => Err(PExecError::CouldNotCreateContext),
        }
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<Option<Progress>> {
        let mut libentity = self.get_libentity(app)?;
        let libentity_static = libentity.produce_libentity();
        let context = self.make_context(app)?;

        let maybe_new_progress = app.scripts().open_libentity(libentity_static, context)?;

        match maybe_new_progress {
            some_progress @ Some(_) => libentity.set_progress(some_progress),
            None => (),
        };
        libentity.dump_to_storage();

        Ok(maybe_new_progress)
    }
}

impl PCommand for OpenPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecError> {
        let _maybe_new_progress = self.execute_inner(app)?;

        Ok(())
    }
}
