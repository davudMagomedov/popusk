use super::{PCommand, PExecutionError};

use crate::app::App;
use crate::error_ext::IntoBoxExt;
use crate::io_ext::IoExt;
use crate::library::LibraryError;
use crate::types::{LibEntityMut, LibEntityMetaError, EntityBase, ID};

use std::io::{stdin, Error as IoError};

use serde_json::{from_str as from_json_str, Error as JsonError};
use thiserror::Error as ThisError;

type CMDResult<T, E = CMDError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum CMDError {
    #[error("library entity with ID {id} wasn't found")]
    LibEntityWasNotFound { id: ID },
    #[error("could not parse json entity base: {0}")]
    CouldNotParseJsonEbase(JsonError),

    #[error("library entity: {0}")]
    LibEntityMeta(#[from] LibEntityMetaError),
    #[error("library: {0}")]
    Library(#[from] LibraryError),
    #[error("i/o: {0}")]
    IO(#[from] IoError),
}

/// UNSAFE COMMAND
#[derive(Debug, Clone)]
pub struct AddEntitybasePCMD {
    id: ID,
}

impl AddEntitybasePCMD {
    pub fn new(id: ID) -> Self {
        AddEntitybasePCMD { id }
    }

    fn get_libentity(&self, app: &mut App) -> CMDResult<LibEntityMut> {
        app.library_mut()
            .get_libentity_mut_by_id(self.id)?
            .ok_or_else(|| CMDError::LibEntityWasNotFound { id: self.id })
    }

    fn read_ebase(&self) -> CMDResult<EntityBase> {
        let json_ebase = stdin().read_to_end_string()?;
        let ebase = from_json_str::<EntityBase>(&json_ebase)
            .map_err(|e| CMDError::CouldNotParseJsonEbase(e))?;

        Ok(ebase)
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<()> {
        let mut libentity = self.get_libentity(app)?;
        let ebase = self.read_ebase()?;

        libentity.set_ebase(ebase)?;
        libentity.dump_to_storage()?;

        Ok(())
    }
}

impl PCommand for AddEntitybasePCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        self.execute_inner(app).map_err(|e| e.into_box())?;

        println!("Given entitybase was associated with the ID {}", self.id);

        Ok(())
    }
}
