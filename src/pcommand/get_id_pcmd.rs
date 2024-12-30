use crate::app::App;
use crate::types::{ID, LibEntityConst, LibEntityMetaError};
use crate::library::LibraryError;
use crate::error_ext::CommonizeResultExt;

use super::{PCommand, PExecutionError};

use std::path::PathBuf;

use thiserror::Error as ThisError;

type CMDResult<T, E = GetIDError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum GetIDError {
    #[error("library entity with path '{path}' wasn't found")]
    LibEntityWasNotFound { path: PathBuf },

    #[error("library entity: {0}")]
    LibEntityMeta(#[from] LibEntityMetaError),
    #[error("library: {0}")]
    Library(#[from] LibraryError)
}

#[derive(Debug, Clone)]
pub struct GetIDPCMD {
    path: PathBuf,
}

impl GetIDPCMD {
    pub fn new(path: PathBuf) -> Self {
        GetIDPCMD { path }
    }

    fn get_libentity(&self, app: &App) -> CMDResult<LibEntityConst> {
        app.library()
            .get_libentity(self.path.clone())?
            .ok_or_else(|| GetIDError::LibEntityWasNotFound { path: self.path.clone() })
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<ID> {
        let libentity = self.get_libentity(app)?;
        Ok(libentity.id())
    }

    fn print_info_msg(&self, id: ID) {
        println!("ID: {0}", id);
    }
}

impl PCommand for GetIDPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let id = self.execute_inner(app).commonize()?;
        self.print_info_msg(id);

        Ok(())
    }
}
