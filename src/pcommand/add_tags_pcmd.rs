use crate::app::App;
use crate::comps_appearance::parse_string_to_tags;
use crate::id::ID;

use super::{PCommand, PExecutionError};

use anyhow::anyhow;
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
        let mut entitybase = match app.storage().get_entitybase(self.id)? {
            Some(entitybase) => entitybase,
            None => return Err(anyhow!("couldn't find entitybase for ID {}", self.id).into()),
        };

        let tags = parse_string_to_tags(&self.stried_tags)?;

        entitybase.tags_mut().extend(tags);
        *entitybase.tags_mut() = entitybase
            .tags()
            .into_iter()
            .unique()
            .map(|s| s.clone())
            .collect();

        app.storage_mut().update_entitybase(self.id, entitybase)?;

        println!("The tags were added");

        Ok(())
    }
}
