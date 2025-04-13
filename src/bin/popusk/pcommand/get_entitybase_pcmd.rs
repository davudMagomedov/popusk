use popusk::app::App;
use popusk::comps_appearance::entitybase_to_fullinfo_string;
use popusk::types::{ID, LibEntityConst, EntityBase, LibEntityMetaError};
use popusk::library::LibraryError;
use popusk::error_ext::CommonizeResultExt;

use super::{PCommand, PExecutionError};

use thiserror::Error as ThisError;

type CMDResult<T, E = CMDError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum CMDError {
    #[error("library entity with ID {id} wasn't found")]
    LibEntityWasNotFound { id: ID },

    #[error("library entity: {0}")]
    LibEntityMeta(#[from] LibEntityMetaError),
    #[error("library: {0}")]
    Library(#[from] LibraryError)
}

#[derive(Debug, Clone)]
pub struct GetEntitybasePCMD {
    id: ID,
}

impl GetEntitybasePCMD {
    pub fn new(id: ID) -> Self {
        GetEntitybasePCMD { id }
    }

    fn get_libentity(&self, app: &App) -> CMDResult<LibEntityConst> {
        app.library()
            .get_libentity_by_id(self.id)?
            .ok_or_else(|| CMDError::LibEntityWasNotFound { id: self.id })
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<EntityBase> {
        Ok(self.get_libentity(app)?.ebase()?)
    }

    fn print_info_msg(&self, entitybase: EntityBase) {
        println!("{}", entitybase_to_fullinfo_string(&entitybase));
    }
}

impl PCommand for GetEntitybasePCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let entitybase = self.execute_inner(app).commonize()?;
        self.print_info_msg(entitybase);

        Ok(())
    }
}
