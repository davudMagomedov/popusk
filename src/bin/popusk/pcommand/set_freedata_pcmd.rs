use crate::io_ext::IoExt;

use super::{PCommand, PEResult, PExecError};

use popusk::app::App;
use popusk::custom_fs::open_readfile;
use popusk::error_ext::ComError;
use popusk::types::{json_to_freedata, FreeData, LibEntityMut};
use popusk::ErrorExt;

use std::io::stdin;
use std::path::{Path, PathBuf};

use serde_json::{from_str as from_json_str, Value as JsonValue};

#[derive(Debug, Clone)]
pub struct SetFreeDataPCMD {
    // path of entity base and ...
    path: PathBuf,
    // ... path of the file with freedata.
    file: Option<PathBuf>,
}

fn read_to_end_file(file: &Path) -> PEResult<String> {
    open_readfile(file)?
        .read_to_end_string()
        .map_err(|ioerr| PExecError::StdinStream { ioerr })
}

fn parse_freedata_from_str(content: &str) -> PEResult<FreeData> {
    let json_value: JsonValue = from_json_str(content).map_err(|error| PExecError::DeserError {
        format: "json",
        component: "freedata",
        error: error.into_box(),
    })?;
    let freedata = json_to_freedata(json_value).map_err(|_error| PExecError::DeserError {
        format: "json",
        component: "freedata",
        error: ComError::from("freedata must be dictionary at the root"),
    })?;

    Ok(freedata)
}

impl SetFreeDataPCMD {
    pub fn new(path: PathBuf, file: Option<PathBuf>) -> Self {
        SetFreeDataPCMD { path, file }
    }

    fn read_freedata_from_stdin(&self) -> PEResult<FreeData> {
        debug_assert!(self.file.is_none());

        let stringified_freedata = stdin()
            .read_to_end_string()
            .map_err(|ioerr| PExecError::StdinStream { ioerr })?;
        let freedata = parse_freedata_from_str(&stringified_freedata)?;

        Ok(freedata)
    }

    /// The function is called assuming that `self.file.is_some()`.
    fn read_freedata_from_file(&self) -> PEResult<FreeData> {
        debug_assert!(self.file.is_some());

        let stringified_freedata = read_to_end_file(self.file.as_ref().unwrap())?;
        let freedata = parse_freedata_from_str(&stringified_freedata)?;

        Ok(freedata)
    }

    fn read_freedata(&self) -> PEResult<FreeData> {
        match self.file {
            Some(_) => self.read_freedata_from_file(),
            None => self.read_freedata_from_stdin(),
        }
    }

    fn get_libentity(&self, app: &mut App) -> PEResult<LibEntityMut> {
        app.library_mut()
            .get_libentity_mut(self.path.clone())
            .ok_or_else(|| PExecError::LibEntityWasNotFound {
                entitypath: self.path.clone(),
            })
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<()> {
        let freedata = self.read_freedata()?;
        let mut libentity = self.get_libentity(app)?;

        libentity.set_freedata(Some(freedata));
        libentity.dump_to_storage();

        Ok(())
    }
}

impl PCommand for SetFreeDataPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecError> {
        self.execute_inner(app)?;

        println!(
            "Freedata was added to library entity '{}'",
            self.path.to_string_lossy()
        );

        Ok(())
    }
}
