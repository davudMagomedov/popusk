use super::{PCommand, PEResult, PExecError};

use popusk::app::App;

use std::path::PathBuf;

/// UNSAFE COMMAND
#[derive(Debug, Clone)]
pub struct AddPathPCMD {
    path: PathBuf,
}

impl AddPathPCMD {
    pub fn new(path: PathBuf) -> Self {
        AddPathPCMD { path }
    }

    fn validation_check(&self, app: &App) -> PEResult<()> {
        if app.library().entity_data().borrow().exists(&self.path) {
            return Err(PExecError::LibEntityAlreadyExists {
                entitypath: self.path.clone(),
            });
        }

        Ok(())
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<()> {
        self.validation_check(app)?;

        app.library_mut()
            .entity_data()
            .borrow_mut()
            .touch(&self.path)?;

        Ok(())
    }
}

impl PCommand for AddPathPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecError> {
        self.execute_inner(app)?;

        println!("The '{}' was added", self.path.to_string_lossy());

        Ok(())
    }
}
