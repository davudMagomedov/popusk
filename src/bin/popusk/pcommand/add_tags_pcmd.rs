use popusk::app::App;
use popusk::comps_appearance::parse_string_to_tags;
use popusk::error_ext::{ComError, CommonizeResultExt};
use popusk::types::{ID, Tag};
use popusk::library::LibraryError;
use popusk::types::{LibEntityMut, LibEntityMetaError};

use super::{PCommand, PExecutionError};

use thiserror::Error as ThisError;
use itertools::Itertools;

type CMDResult<T, E = CMDError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum CMDError {
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
            .ok_or_else(|| CMDError::LibEntityWasNotFound { id: self.id })
    }

    fn get_tags(&self) -> CMDResult<Vec<Tag>> {
        parse_string_to_tags(&self.stried_tags)
            .map_err(|e| CMDError::CouldNotParseStringToTags(e))
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<()> {
        let mut libentity = self.get_libentity(app)?;

        let mut tags = libentity.tags()?;
        let new_tags = self.get_tags()?;

        tags.extend(new_tags);
        tags = tags.into_iter().unique().collect();

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
