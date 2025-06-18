use super::{PCommand, PExecError, PEResult};

use popusk::app::App;
use popusk::custom_fs::rename;

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RenameLibEntityPCMD {
    path: PathBuf,
    new_path: PathBuf,
}

impl RenameLibEntityPCMD {
    pub fn new(path: PathBuf, new_path: PathBuf) -> Self {
        RenameLibEntityPCMD { path, new_path }
    }

    fn old_libentity_exists(&self, app: &App) -> bool {
        app.library().libentity_exists(&self.path)
    }

    fn new_libentity_exists(&self, app: &App) -> bool {
        app.library().libentity_exists(&self.new_path)
    }

    fn error_if_old_libentity_dont_exist(&self, app: &App) -> PEResult<()> {
        match self.old_libentity_exists(app) {
            true => Ok(()),
            false => Err(PExecError::LibEntityWasNotFound { entitypath: self.path.clone() }),
        }
    }

    fn error_if_new_libentity_exists(&self, app: &App) -> PEResult<()> {
        match self.new_libentity_exists(app) {
            true => Err(PExecError::LibEntityAlreadyExists { entitypath: self.path.clone() }),
            false => Ok(()),
        }
    }

    fn error_if_path_dont_exist(&self) -> PEResult<()> {
        match self.path.exists() {
            true => Ok(()),
            false => Err(PExecError::PathDoesNotExist { path: self.path.clone() }),
        }
    }

    fn error_if_new_path_exists(&self) -> PEResult<()> {
        match self.new_path.exists() {
            true => Err(PExecError::PathAlreadyExists { path: self.new_path.clone() }),
            false => Ok(()),
        }
    }

    fn rename_file(&self) -> PEResult<()> {
        rename(&self.path, &self.new_path).map_err(Into::into)
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<()> {
        self.error_if_old_libentity_dont_exist(app)?;
        self.error_if_new_libentity_exists(app)?;
        self.error_if_path_dont_exist()?;
        self.error_if_new_path_exists()?;

        app.library_mut().rename(&self.path, &self.new_path);
        self.rename_file()?;

        Ok(())
    }
}

impl PCommand for RenameLibEntityPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecError> {
        self.execute_inner(app)?;

        println!(
            "Library entity was succesfully placed from '{}' to '{}'",
            self.path.to_string_lossy(),
            self.new_path.to_string_lossy()
        );

        Ok(())
    }
}
