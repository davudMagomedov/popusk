use popusk::app::App;
use popusk::types::LibEntityMut;

use super::{PCommand, PEResult, PExecError};

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DelDescriptionPCMD {
    path: PathBuf,
}

impl DelDescriptionPCMD {
    pub fn new(path: PathBuf) -> Self {
        DelDescriptionPCMD { path }
    }

    fn get_libentity(&self, app: &mut App) -> PEResult<LibEntityMut> {
        app.library_mut()
            .get_libentity_mut(self.path.clone())
            .ok_or_else(|| PExecError::LibEntityWasNotFound {
                entitypath: self.path.clone(),
            })
    }

    fn validation_check(&self, libentity: &LibEntityMut) -> PEResult<()> {
        if libentity.description().is_none() {
            return Err(PExecError::ComponentWasNotFound {
                component: "description",
                entitypath: self.path.clone(),
            });
        }

        Ok(())
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<()> {
        let mut libentity = self.get_libentity(app)?;

        self.validation_check(&libentity)?;

        libentity.set_description(None);
        libentity.dump_to_storage();

        Ok(())
    }
}

impl PCommand for DelDescriptionPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecError> {
        self.execute_inner(app)?;

        println!("The description was deleted");

        Ok(())
    }
}
