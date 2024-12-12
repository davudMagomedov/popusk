use super::{PCommand, PExecutionError};

use crate::app::App;
use crate::types::{ID, FreeData, json_to_freedata};
use crate::error_ext::{ComError, IntoBoxExt};
use crate::storage::StorageError;

use std::io::{stdin, Read, Error as IoError, ErrorKind as IoErrorKind};
use std::path::{PathBuf, Path};

use thiserror::Error as Error;
use serde_json::{from_str as from_json_str, Value as JsonValue, Error as JsonError};

#[derive(Debug, Error)]
enum AddFreeDataError {
    #[error("non utf8 stdin input")]
    NonUTF8StdinInput,
    #[error("couldn't parse FreeData from string: {0}")]
    CouldNotParseFreeData(ComError),
    #[error("couldn't find file: {0}")]
    CouldNotFindFile(PathBuf),
    #[error("couldn't convert json to freedata: {0}")]
    CouldNotConvertJsonToFData(ComError),

    #[error("couldn't parse json value: {0}")]
    Json(#[from] JsonError),
    #[error("io: {0}")]
    IO(#[from] IoError),
    #[error("storage: {0}")]
    Storage(#[from] StorageError),
}

impl From<AddFreeDataError> for PExecutionError {
    fn from(afde: AddFreeDataError) -> Self {
        use AddFreeDataError::*;

        match afde {
            Storage(storage_error) => PExecutionError::StorageError(storage_error),
            IO(io_error) => PExecutionError::IO(io_error),
            other => PExecutionError::Other(other.into_box())
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddFreeDataPCMD {
    id: ID,
    file: Option<PathBuf>,
}

fn read_to_end_stdin() -> Result<String, AddFreeDataError> {
    let mut buff = Vec::new();
    stdin().read_to_end(&mut buff)?;
    String::from_utf8(buff).map_err(|_| AddFreeDataError::NonUTF8StdinInput)
}

fn read_to_end_file(file: &Path) -> Result<String, AddFreeDataError> {
    match std::fs::read_to_string(file) {
        Ok(o) => Ok(o),
        Err(e) if e.kind() == IoErrorKind::NotFound => Err(
            AddFreeDataError::CouldNotFindFile(file.to_owned()),
        ),
        Err(e) => Err(e.into()),
    }
}

fn parse_freedata_from_str(content: &str) -> Result<FreeData, AddFreeDataError> {
    let json_value: JsonValue = from_json_str(content)?;
    let freedata = json_to_freedata(json_value)
        .map_err(|e| AddFreeDataError::CouldNotConvertJsonToFData(e))?;

    Ok(freedata)
}

impl AddFreeDataPCMD {
    pub fn new(id: ID, file: Option<PathBuf>) -> Self {
        AddFreeDataPCMD { id, file }
    }

    fn read_freedata_from_stdin(&self) -> Result<FreeData, AddFreeDataError> {
        debug_assert!(self.file.is_none());

        let stringified_freedata = read_to_end_stdin()?;
        let freedata = parse_freedata_from_str(&stringified_freedata)?;

        Ok(freedata)
    }

    /// The function is called assuming that `self.file.is_some()`.
    fn read_freedata_from_file(&self) -> Result<FreeData, AddFreeDataError> {
        debug_assert!(self.file.is_some());

        let stringified_freedata = read_to_end_file(self.file.as_ref().unwrap())?;
        let freedata = parse_freedata_from_str(&stringified_freedata)?;

        Ok(freedata)
    }

    fn read_freedata(&self) -> Result<FreeData, AddFreeDataError> {
        match self.file {
            Some(_) => self.read_freedata_from_file(),
            None => self.read_freedata_from_stdin(),
        }
    }

    fn execute_inner(&self, app: &mut App) -> Result<(), AddFreeDataError> {
        let freedata = self.read_freedata()?;
        unsafe { app.library_mut().storage_mut() }.link_freedata_to_id(self.id, freedata)?;

        Ok(())
    }
}

impl PCommand for AddFreeDataPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        self.execute_inner(app).map_err(|e| PExecutionError::from(e))
    }
}
