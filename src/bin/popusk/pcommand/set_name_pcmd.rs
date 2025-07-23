use super::{PCommand, PEResult, PExecError};

use popusk::app::App;
use popusk::libentity_meta::LibEntityMut;

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SetNamePCMD {
    path: PathBuf,
    name: String,
}

impl SetNamePCMD {
    pub fn new(path: PathBuf, name: String) -> Self {
        SetNamePCMD { path, name }
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
        libentity.set_name(self.name.clone());
        libentity.dump_to_storage();

        Ok(())
    }
}

impl PCommand for SetNamePCMD {
    fn execute(&self, app: &mut App) -> PEResult<()> {
        self.execute_inner(app)?;
        println!(
            "Name '{}' was set for library entity '{}'",
            self.name,
            self.path.to_string_lossy()
        );

        Ok(())
    }
}
