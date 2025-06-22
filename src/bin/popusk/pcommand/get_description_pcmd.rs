use super::{PCommand, PEResult, PExecError};

use popusk::{App, LibEntityConst};

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GetDescriptionPCMD {
    path: PathBuf,
}

impl GetDescriptionPCMD {
    pub fn new(path: PathBuf) -> Self {
        GetDescriptionPCMD { path }
    }

    fn get_libentity(&self, app: &App) -> PEResult<LibEntityConst> {
        app.library()
            .get_libentity(self.path.clone())
            .ok_or_else(|| PExecError::LibEntityWasNotFound {
                entitypath: self.path.clone(),
            })
    }

    fn get_description(&self, libentity: &LibEntityConst) -> PEResult<String> {
        libentity
            .description()
            .ok_or_else(|| PExecError::ComponentWasNotFound {
                component: "description",
                entitypath: self.path.clone(),
            })
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<String> {
        self.get_description(&self.get_libentity(app)?)
    }
}

impl PCommand for GetDescriptionPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecError> {
        let description = self.execute_inner(app)?;
        println!("{}", description);

        Ok(())
    }
}
