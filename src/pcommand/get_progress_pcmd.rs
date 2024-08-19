use crate::app::App;
use crate::comps_appearance::progress_to_string;
use crate::id::ID;

use super::{PCommand, PExecutionError};

use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct GetProgressPCMD {
    id: ID,
}

impl GetProgressPCMD {
    pub fn new(id: ID) -> Self {
        GetProgressPCMD { id }
    }
}

impl PCommand for GetProgressPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let maybe_progress = app.storage().get_progress(self.id)?;

        match maybe_progress {
            Some(progress) => println!("Progress: {}", progress_to_string(&progress)),
            None => return Err(anyhow!("couldn't find the progress for ID {}", self.id).into()),
        }

        Ok(())
    }
}
