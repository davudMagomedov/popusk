use crate::app::App;

use super::{PCommand, PExecutionError};

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GetIDPCMD {
    path: PathBuf,
}

impl GetIDPCMD {
    pub fn new(path: PathBuf) -> Self {
        GetIDPCMD { path }
    }
}

impl PCommand for GetIDPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let maybe_id = app.storage().get_id(self.path.clone())?;

        match maybe_id {
            Some(id) => println!("ID: {}", id),
            None => println!("There's no ID linked to given path"),
        }

        Ok(())
    }
}
