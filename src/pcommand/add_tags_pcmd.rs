use crate::app::App;
use crate::comps_appearance::parse_string_to_tags;
use crate::error_ext::ComError;
use crate::id::ID;

use super::{PCommand, PExecutionError};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct AddTagsPCMD {
    id: ID,
    stried_tags: String,
}

impl AddTagsPCMD {
    pub fn new(id: ID, stried_tags: String) -> Self {
        AddTagsPCMD { id, stried_tags }
    }
}

impl PCommand for AddTagsPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let mut entitybase = match unsafe { app.library().storage() }.get_entitybase(self.id)? {
            Some(entitybase) => entitybase,
            None => {
                return Err(
                    ComError::from(format!("couldn't find entitybase for ID {}", self.id)).into(),
                )
            }
        };

        let tags = parse_string_to_tags(&self.stried_tags)?;

        entitybase.tags_mut().extend(tags);
        *entitybase.tags_mut() = entitybase
            .tags()
            .into_iter()
            .unique()
            .map(|s| s.clone())
            .collect();

        unsafe { app.library_mut().storage_mut() }.update_entitybase(self.id, entitybase)?;

        println!("The tags were added");

        Ok(())
    }
}
