use super::{PCommand, PEResult, PExecError};

use popusk::app::App;
use popusk::comps_appearance::entitybase_to_fullinfo_string;
use popusk::types::{EntityBase, LibEntityConst};

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GetEntitybasePCMD {
    path: PathBuf,
}

impl GetEntitybasePCMD {
    pub fn new(path: PathBuf) -> Self {
        GetEntitybasePCMD { path }
    }

    fn get_libentity(&self, app: &App) -> PEResult<LibEntityConst> {
        app.library()
            .get_libentity(self.path.clone())
            .ok_or_else(|| PExecError::LibEntityWasNotFound {
                entitypath: self.path.clone(),
            })
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<EntityBase> {
        let libentity = self.get_libentity(app)?;
        let entitybase = EntityBase {
            name: libentity.name(),
            etype: libentity.etype(),
            tags: libentity.tags(),
        };
        Ok(entitybase)
    }

    fn print_info_msg(&self, entitybase: EntityBase) {
        println!("{}", entitybase_to_fullinfo_string(&entitybase));
    }
}

impl PCommand for GetEntitybasePCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecError> {
        let entitybase = self.execute_inner(app)?;
        self.print_info_msg(entitybase);

        Ok(())
    }
}
