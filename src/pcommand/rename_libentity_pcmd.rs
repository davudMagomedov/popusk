use crate::app::App;
use crate::error_ext::CommonizeResultExt;
use crate::types::ID;
use crate::storage::StorageError;

use super::{PCommand, PExecutionError};

use std::fs::rename as rename_file;
use std::path::PathBuf;
use std::io::Error as IoError;

use thiserror::Error as ThisError;

type CMDResult<T, E = RenameLibEntityError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum RenameLibEntityError {
    #[error("could not find library entity with path '{path}'")]
    CouldNotFindLibEntity { path: PathBuf },
    #[error("there is already library entity with path '{path}'")]
    LibEntityAlreadyExists { path: PathBuf },
    #[error("file '{path}' already exists")]
    FileAlreadyExists { path: PathBuf },
    #[error("file '{path}' does not exist")]
    FileDoesNotExist { path: PathBuf },

    #[error("storage: {0}")]
    Storage(#[from] StorageError),
    #[error("i/o: {0}")]
    IO(#[from] IoError),
}

#[derive(Debug, Clone)]
pub struct RenameLibEntityPCMD {
    path: PathBuf,
    new_path: PathBuf,
}

impl RenameLibEntityPCMD {
    pub fn new(path: PathBuf, new_path: PathBuf) -> Self {
        RenameLibEntityPCMD { path, new_path }
    }

    fn old_libentity_exists(&self, app: &App) -> CMDResult<bool> {
        app.library().storage().borrow()
            .get_id(self.path.clone())
            .map(|opt| opt.is_some())
            .map_err(|err| err.into())
    }

    fn new_libentity_exists(&self, app: &App) -> CMDResult<bool> {
        app.library().storage().borrow()
            .get_id(self.new_path.clone())
            .map(|opt| opt.is_some())
            .map_err(|err| err.into())
    }

    fn unlink_id_from_path_raw(&self, app: &mut App) -> CMDResult<ID> {
        debug_assert!(self.old_libentity_exists(app)?);

        let id = unsafe {
            app.library_mut().storage().borrow_mut().unlink_id_from_path_raw(self.path.clone())?
        };

        Ok(id)
    }

    fn link_id_to_new_path_raw(&self, app: &mut App, id: ID) -> CMDResult<()> {
        debug_assert!(!self.new_libentity_exists(app)?);

        unsafe {
            app.library_mut().storage().borrow_mut().link_id_to_path_raw(
                self.new_path.clone(),
                id
            )?
        };

        Ok(())
    }

    fn error_if_old_libentity_dont_exist(&self, app: &App) -> CMDResult<()> {
        match self.old_libentity_exists(app)? {
            true => Ok(()),
            false => Err(RenameLibEntityError::CouldNotFindLibEntity { path: self.path.clone() }),
        }
    }

    fn error_if_new_libentity_exists(&self, app: &App) -> CMDResult<()> {
        match self.new_libentity_exists(app)? {
            true => Err(RenameLibEntityError::LibEntityAlreadyExists { path: self.new_path.clone() }),
            false => Ok(()),
        }
    }

    fn error_if_path_dont_exist(&self) -> CMDResult<()> {
        match self.path.exists() {
            true => Ok(()),
            false => Err(RenameLibEntityError::FileDoesNotExist { path: self.path.clone() }),
        }
    }

    fn error_if_new_path_exists(&self) -> CMDResult<()> {
        match self.new_path.exists() {
            true => Err(RenameLibEntityError::FileAlreadyExists { path: self.new_path.clone() }),
            false => Ok(()),
        }
    }

    fn rename_file(&self) -> CMDResult<()> {
        rename_file(&self.path, &self.new_path)?;
        Ok(())
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<()> {
        self.error_if_old_libentity_dont_exist(app)?;
        self.error_if_new_libentity_exists(app)?;
        self.error_if_path_dont_exist()?;
        self.error_if_new_path_exists()?;

        let id = self.unlink_id_from_path_raw(app)?;
        self.link_id_to_new_path_raw(app, id)?;

        self.rename_file()?;

        Ok(())
    }
}

impl PCommand for RenameLibEntityPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        self.execute_inner(app).commonize()?;

        println!(
            "Library entity was succesfully placed from '{}' to '{}'.",
            self.path.to_string_lossy(),
            self.new_path.to_string_lossy()
        );

        Ok(())
    }
}
