use popusk::app::App;
use popusk::comps_appearance::parse_string_to_tags;
use popusk::types::{LibEntityMut, Tag};

use super::{PCommand, PEResult, PExecError};
use super::minilib::verify_tags;

use std::path::PathBuf;

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct AddTagsPCMD {
    path: PathBuf,
    stried_tags: String,
}

impl AddTagsPCMD {
    pub fn new(path: PathBuf, stried_tags: String) -> Self {
        AddTagsPCMD { path, stried_tags }
    }

    fn get_libentity(&self, app: &mut App) -> PEResult<LibEntityMut> {
        app.library_mut()
            .get_libentity_mut(self.path.clone())
            .ok_or_else(|| PExecError::LibEntityWasNotFound {
                entitypath: self.path.clone(),
            })
    }

    fn get_tags(&self) -> PEResult<Vec<Tag>> {
        parse_string_to_tags(&self.stried_tags).map_err(|error| PExecError::DeserError {
            format: "string",
            component: "tags",
            error,
        })
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<()> {
        let mut libentity = self.get_libentity(app)?;

        let mut tags = libentity.tags();
        let new_tags = self.get_tags()?;

        tags.extend(new_tags);
        tags = tags.into_iter().unique().collect();
        verify_tags(&tags)?;

        libentity.set_tags(tags);
        libentity.dump_to_storage();

        Ok(())
    }
}

impl PCommand for AddTagsPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecError> {
        self.execute_inner(app)?;

        println!("The tags were added");

        Ok(())
    }
}
