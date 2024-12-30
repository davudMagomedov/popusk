use super::{PCommand, PExecutionError};

use crate::app::App;
use crate::types::{ID, FreeData, json_to_freedata, LibEntityMetaError, LibEntityMut};
use crate::error_ext::{ComError, CommonizeResultExt, IntoBoxExt};
use crate::io_ext::IoExt;
use crate::library::LibraryError;
use crate::storage::StorageError;

use std::io::{stdin, Error as IoError, ErrorKind as IoErrorKind};
use std::path::{PathBuf, Path};

use thiserror::Error as Error;
use serde_json::{from_str as from_json_str, Value as JsonValue, Error as JsonError};

type CMDResult<T> = Result<T, AddFreeDataError>;

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
    #[error("couldn't find library entity")]
    CouldNotFindLibEntity,

    #[error("couldn't parse json value: {0}")]
    Json(#[from] JsonError),
    #[error("io: {0}")]
    IO(#[from] IoError),
    #[error("storage: {0}")]
    Storage(#[from] StorageError),
    #[error("library: {0}")]
    Library(#[from] LibraryError),
    #[error("library entity: {0}")]
    LibEntityMeta(#[from] LibEntityMetaError),
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

fn read_to_end_file(file: &Path) -> CMDResult<String> {
    match std::fs::read_to_string(file) {
        Ok(o) => Ok(o),
        Err(e) if e.kind() == IoErrorKind::NotFound => Err(
            AddFreeDataError::CouldNotFindFile(file.to_owned()),
        ),
        Err(e) => Err(e.into()),
    }
}

fn parse_freedata_from_str(content: &str) -> CMDResult<FreeData> {
    let json_value: JsonValue = from_json_str(content)?;
    let freedata = json_to_freedata(json_value)
        .map_err(|e| AddFreeDataError::CouldNotConvertJsonToFData(e))?;

    Ok(freedata)
}

impl AddFreeDataPCMD {
    pub fn new(id: ID, file: Option<PathBuf>) -> Self {
        AddFreeDataPCMD { id, file }
    }

    fn read_freedata_from_stdin(&self) -> CMDResult<FreeData> {
        debug_assert!(self.file.is_none());

        let stringified_freedata = stdin().read_to_end_string()?;
        let freedata = parse_freedata_from_str(&stringified_freedata)?;

        Ok(freedata)
    }

    /// The function is called assuming that `self.file.is_some()`.
    fn read_freedata_from_file(&self) -> CMDResult<FreeData> {
        debug_assert!(self.file.is_some());

        let stringified_freedata = read_to_end_file(self.file.as_ref().unwrap())?;
        let freedata = parse_freedata_from_str(&stringified_freedata)?;

        Ok(freedata)
    }

    fn read_freedata(&self) -> CMDResult<FreeData> {
        match self.file {
            Some(_) => self.read_freedata_from_file(),
            None => self.read_freedata_from_stdin(),
        }
    }

    fn get_libentity(&self, app: &mut App) -> CMDResult<LibEntityMut> {
        app.library_mut()
            .get_libentity_mut_by_id(self.id)?
            .ok_or_else(|| AddFreeDataError::CouldNotFindLibEntity)
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<()> {
        let freedata = self.read_freedata()?;
        let mut libentity = self.get_libentity(app)?;

        libentity.set_freedata(Some(freedata))?;
        libentity.dump_to_storage()?;

        Ok(())
    }
}

impl PCommand for AddFreeDataPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        self.execute_inner(app).commonize()?;
        println!("Free Data was added");

        Ok(())
    }
}
