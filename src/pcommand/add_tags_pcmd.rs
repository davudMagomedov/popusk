use crate::app::App;
use crate::comps_appearance::parse_string_to_tags;
use crate::error_ext::{ComError, CommonizeResultExt};
use crate::types::{ID, Tag};
use crate::library::LibraryError;
use crate::types::{LibEntityMut, LibEntityMetaError};

use super::{PCommand, PExecutionError};

use thiserror::Error as ThisError;

type CMDResult<T, E = AddTagsError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum AddTagsError {
    #[error("library entity with ID {id} wasn't found")]
    LibEntityWasNotFound { id: ID },
    #[error("could not parse string to vector of tags: {0}")]
    CouldNotParseStringToTags(ComError),

    #[error("library: {0}")]
    Library(#[from] LibraryError),
    #[error("library entity: {0}")]
    LibEntity(#[from] LibEntityMetaError),
}

#[derive(Debug, Clone)]
pub struct AddTagsPCMD {
    id: ID,
    stried_tags: String,
}

impl AddTagsPCMD {
    pub fn new(id: ID, stried_tags: String) -> Self {
        AddTagsPCMD { id, stried_tags }
    }

    fn get_libentity(&self, app: &mut App) -> CMDResult<LibEntityMut> {
        app.library_mut()
            .get_libentity_mut_by_id(self.id)?
            .ok_or_else(|| AddTagsError::LibEntityWasNotFound { id: self.id })
    }

    fn get_tags(&self) -> CMDResult<Vec<Tag>> {
        parse_string_to_tags(&self.stried_tags)
            .map_err(|e| AddTagsError::CouldNotParseStringToTags(e))
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<()> {
        let mut libentity = self.get_libentity(app)?;

        let mut tags = libentity.tags()?;
        let new_tags = self.get_tags()?;

        tags.extend(new_tags);

        libentity.set_tags(tags)?;
        libentity.dump_to_storage()?;

        Ok(())
    }
}

impl PCommand for AddTagsPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        self.execute_inner(app).commonize()?;

        println!("The tags were added");

        Ok(())
    }
}
