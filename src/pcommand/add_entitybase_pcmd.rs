use super::{PCommand, PExecutionError};

use crate::app::App;
use crate::entity_base::EntityBase;
use crate::id::ID;

use std::io::{stdin, Read};

use bincode::deserialize as bincode_deserialize;

#[derive(Debug, Clone)]
pub struct AddEntitybasePCMD {
    id: ID,
}

impl AddEntitybasePCMD {
    pub fn new(id: ID) -> Self {
        AddEntitybasePCMD { id }
    }
}

impl PCommand for AddEntitybasePCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let mut serialized_entitybase = Vec::new();
        stdin().read_to_end(&mut serialized_entitybase)?;

        let entitybase: EntityBase = bincode_deserialize(&serialized_entitybase)?;
        crate::core_commands::corecmd_add_entitybase(app.storage_mut(), self.id, entitybase)?;

        println!("Given entitybase was associated with the ID {}", self.id);

        Ok(())
    }
}
