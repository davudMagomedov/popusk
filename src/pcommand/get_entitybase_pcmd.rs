use crate::app::App;
use crate::comps_appearance::entitybase_to_fullinfo_string;
use crate::id::ID;

use super::{PCommand, PExecutionError};

#[derive(Debug, Clone)]
pub struct GetEntitybasePCMD {
    id: ID,
}

impl GetEntitybasePCMD {
    pub fn new(id: ID) -> Self {
        GetEntitybasePCMD { id }
    }
}

impl PCommand for GetEntitybasePCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let maybe_entitybase = unsafe { app.library().storage() }.get_entitybase(self.id)?;

        match maybe_entitybase {
            Some(entitybase) => println!("{}", entitybase_to_fullinfo_string(&entitybase)),
            None => println!(
                "Couldn't find entity base associated with the {} ID",
                self.id
            ),
        }

        Ok(())
    }
}
