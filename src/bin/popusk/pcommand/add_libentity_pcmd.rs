use popusk::app::App;
use popusk::types::{LibEntity, EntityType};

use super::{PCommand, PExecError, PEResult};

use std::path::PathBuf;

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct AddLibentityPCMD {
    path: PathBuf,
}

impl AddLibentityPCMD {
    pub fn new(path: PathBuf) -> Self {
        AddLibentityPCMD { path }
    }

    fn fix_libentity_data(&self, mut libentity_data: LibEntity) -> PEResult<LibEntity> {
        libentity_data.path = self.path.clone();
        if self.path.is_dir() { libentity_data.etype = EntityType::Section }
        libentity_data.tags = libentity_data.tags.into_iter().unique().collect();

        Ok(libentity_data)
    }

    /// Returns error if somehow the command can't be run.
    fn validation_check(&self, app: &App) -> PEResult<()> {
        if app.library().libentity_exists(&self.path.clone()) {
            return Err(PExecError::LibEntityAlreadyExists { entitypath: self.path.clone() });
        };

        if !self.path.exists() {
            return Err(PExecError::PathDoesNotExist { path: self.path.clone() });
        }

        Ok(())
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<()> {
        self.validation_check(app)?;

        let libentity = app.scripts().add_library_entity(self.path.clone())?;
        let libentity = self.fix_libentity_data(libentity)?;
        let libentity_mut = app.library_mut().create_from_static_libentity(libentity);
        libentity_mut.dump_to_storage();

        Ok(())
    }
}

impl PCommand for AddLibentityPCMD {
    fn execute(&self, app: &mut App) -> PEResult<()> {
        self.execute_inner(app)
    }
}
