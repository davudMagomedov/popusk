use popusk::app::App;
use popusk::types::LibEntityMut;

use super::{PCommand, PExecError, PEResult};

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SetDescriptionPCMD {
    path: PathBuf,
    description: String,
}

impl SetDescriptionPCMD {
    pub fn new(path: PathBuf, description: String) -> Self {
        SetDescriptionPCMD { path, description }
    }

    fn get_libentity(&self, app: &mut App) -> PEResult<LibEntityMut> {
        app.library_mut()
            .get_libentity_mut(self.path.clone())
            .ok_or_else(|| PExecError::LibEntityWasNotFound {
                entitypath: self.path.clone(),
            })
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<()> {
        let mut libentity = self.get_libentity(app)?;
        libentity.set_description(Some(self.description.clone()));
        libentity.dump_to_storage();

        Ok(())
    }
}

impl PCommand for SetDescriptionPCMD {
    fn execute(&self, app: &mut App) -> PEResult<()> {
        self.execute_inner(app)?;
        println!(
            "The description was added in libreary entity '{}'",
            self.path.to_string_lossy()
        );

        Ok(())
    }
}
