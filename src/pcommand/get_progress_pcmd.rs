use crate::app::App;
use crate::comps_appearance::progress_to_string;
use crate::error_ext::ComError;
use crate::types::ID;

use super::{PCommand, PExecutionError};

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
        let maybe_progress = unsafe { app.library().storage() }.get_progress(self.id)?;

        match maybe_progress {
            Some(progress) => println!("Progress: {}", progress_to_string(&progress)),
            None => {
                return Err(ComError::from(format!(
                    "couldn't find the progress for ID {}",
                    self.id
                ))
                .into())
            }
        }

        Ok(())
    }
}
