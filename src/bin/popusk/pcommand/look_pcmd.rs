use super::{PCommand, PEResult, PExecError};

use popusk::app::App;
use popusk::scripts::Context;
use popusk::types::{LibEntity, StyledText};

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct LookPCMD {
    path: PathBuf,
}

impl LookPCMD {
    pub fn new(path: PathBuf) -> Self {
        LookPCMD { path }
    }

    fn get_libentity(&self, app: &App) -> PEResult<LibEntity> {
        match app.library().get_libentity(self.path.clone()) {
            Some(libentity) => Ok(libentity.produce_libentity()),
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

    fn execute_inner(&self, app: &mut App) -> PEResult<StyledText> {
        let libentity = self.get_libentity(app)?;
        let context = self.make_context(app)?;
        let result = app.scripts().look_output(libentity, context)?;

        Ok(result)
    }
}

impl PCommand for LookPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecError> {
        let result = self.execute_inner(app)?;
        print!("{}", result);

        Ok(())
    }
}
