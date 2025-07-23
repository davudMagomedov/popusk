use super::{PCommand, PEResult, PExecError};
use super::minilib::verify_etype;

use popusk::app::App;
use popusk::comps_appearance::entitytype_to_string;
use popusk::libentity_meta::LibEntityMut;
use popusk::types::EntityType;

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SetEtypePCMD {
    path: PathBuf,
    etype: EntityType,
}

impl SetEtypePCMD {
    pub fn new(path: PathBuf, etype: EntityType) -> Self {
        SetEtypePCMD { path, etype }
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
        verify_etype(self.etype, &libentity)?;
        libentity.set_etype(self.etype);
        libentity.dump_to_storage();

        Ok(())
    }
}

impl PCommand for SetEtypePCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecError> {
        self.execute_inner(app)?;
        println!(
            "Entity type '{}' was set for library entity '{}'",
            entitytype_to_string(self.etype),
            self.path.to_string_lossy()
        );

        Ok(())
    }
}
