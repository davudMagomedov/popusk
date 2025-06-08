mod encode_path;

use crate::custom_fs::{
    create_file, create_new_dir, open_readfile, remove_dir, remove_file, FSError,
};
use crate::types::{EntityBase, FreeData, Progress};

use std::fs;
use std::io::{Error as IoError, Read, Write};
use std::path::{Path, PathBuf};

use bincode::{deserialize as bincode_deserialize, serialize as bincode_serialize};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const EBASE_FILENAME: &str = "ebase";
const DESCRIPTION_FILENAME: &str = "description";
const PROGRESS_FILENAME: &str = "progress";
const FREEDATA_FILENAME: &str = "freedata";

const EBASE_NAME: &str = "entity base";
const DESCRIPTION_NAME: &str = "description";
const PROGRESS_NAME: &str = "progress";
const FREEDATA_NAME: &str = "freedata";

macro_rules! iofile_err_map {
    ($res:expr, $path:expr) => {
        $res.map_err(|ioerr| EDError::IOFile {
            path: $path.to_owned(),
            ioerr
        })
    };
}

type EDResult<T> = Result<T, EDError>;

#[derive(Debug, Error)]
pub enum EDError {
    #[error("{path}: {ioerr}")]
    IOFile { path: PathBuf, ioerr: IoError },
    #[error("could not deserialize {path} to {component}")]
    Deserialization {
        path: PathBuf,
        component: &'static str,
    },

    #[error("{0}")]
    FS(#[from] FSError),
}

#[derive(Debug)]
pub struct EntityData {
    // <path>
    //    <path_1>
    //        ebase
    //        description
    //        progress
    //        freedata
    //    <path_2>
    //        ...
    //    ...
    path: PathBuf,
}

impl EntityData {
    /// Given path must exist.
    pub fn new(path: PathBuf) -> Self {
        EntityData { path }
    }

    pub fn exists(&self, epath: &Path) -> bool {
        self.path.join(epath).exists()
    }

    pub fn touch(&self, epath: &Path) -> EDResult<()> {
        create_new_dir(&self.path.join(epath)).map_err(EDError::from)
    }

    pub fn delete(&self, epath: &Path) -> EDResult<()> {
        remove_dir(&self.path.join(epath)).map_err(EDError::from)
    }

    pub fn ebase_exists(&self, epath: &Path) -> bool {
        self.make_component_path(epath, EBASE_FILENAME).exists()
    }

    pub fn description_exists(&self, epath: &Path) -> bool {
        self.make_component_path(epath, DESCRIPTION_FILENAME).exists()
    }

    pub fn progress_exists(&self, epath: &Path) -> bool {
        self.make_component_path(epath, PROGRESS_FILENAME).exists()
    }

    pub fn freedata_exists(&self, epath: &Path) -> bool {
        self.make_component_path(epath, FREEDATA_FILENAME).exists()
    }

    pub fn get_ebase(&self, epath: &Path) -> EDResult<EntityBase> {
        self.get_component(epath, EBASE_FILENAME, EBASE_NAME)
    }

    pub fn get_description(&self, epath: &Path) -> EDResult<String> {
        self.get_component(epath, DESCRIPTION_FILENAME, DESCRIPTION_NAME)
    }

    pub fn get_progress(&self, epath: &Path) -> EDResult<Progress> {
        self.get_component(epath, PROGRESS_FILENAME, PROGRESS_NAME)
    }

    pub fn get_freedata(&self, epath: &Path) -> EDResult<FreeData> {
        self.get_component(epath, FREEDATA_FILENAME, FREEDATA_NAME)
    }

    pub fn set_ebase(&mut self, epath: &Path, ebase: EntityBase) -> EDResult<()> {
        self.set_component(epath, EBASE_FILENAME, ebase)
    }

    pub fn set_description(&mut self, epath: &Path, description: String) -> EDResult<()> {
        self.set_component(epath, DESCRIPTION_FILENAME, description)
    }

    pub fn set_progress(&mut self, epath: &Path, progress: Progress) -> EDResult<()> {
        self.set_component(epath, PROGRESS_FILENAME, progress)
    }

    pub fn set_freedata(&mut self, epath: &Path, freedata: FreeData) -> EDResult<()> {
        self.set_component(epath, FREEDATA_FILENAME, freedata)
    }

    pub fn unset_ebase(&mut self, epath: &Path) -> EDResult<()> {
        self.unset_component(epath, EBASE_FILENAME)
    }

    pub fn unset_description(&mut self, epath: &Path) -> EDResult<()> {
        self.unset_component(epath, DESCRIPTION_FILENAME)
    }

    pub fn unset_progress(&mut self, epath: &Path) -> EDResult<()> {
        self.unset_component(epath, PROGRESS_FILENAME)
    }

    pub fn unset_freedata(&mut self, epath: &Path) -> EDResult<()> {
        self.unset_component(epath, FREEDATA_FILENAME)
    }

    fn get_component<T: for<'de> Deserialize<'de>>(
        &self,
        path: &Path,
        component_filename: &'static str,
        component: &'static str,
    ) -> EDResult<T> {
        let component_path = self.make_component_path(path, component_filename);
        //let mut component_file = match open_readfile(&component_path) {
        //    Err(EntityDataError::CouldNotOpenFile { io_errkind: IoErrorKind::NotFound, .. }) =>
        //        return Ok(None),
        //    Err(e) => return Err(e),
        //    Ok(v) => v,
        //};
        let mut component_file = open_readfile(&component_path)?;
        let component_file_content =
            self.read_component_file(&mut component_file, &component_path)?;
        let deserialized: T = match bincode_deserialize(&component_file_content) {
            Ok(ebase) => ebase,
            Err(_) => {
                return Err(EDError::Deserialization {
                    path: component_path,
                    component,
                })
            }
        };

        Ok(deserialized)
    }

    fn set_component<T: Serialize>(
        &mut self,
        epath: &Path,
        component_filename: &'static str,
        obj: T,
    ) -> EDResult<()> {
        let component_path = self.make_component_path(epath, component_filename);
        let mut file = create_file(&component_path)?;
        let serialized = bincode_serialize(&obj).expect("error shouldn't occur");

        iofile_err_map!(file.write_all(&serialized), &component_path)?;

        Ok(())
    }

    fn unset_component(&mut self, epath: &Path, component_filename: &'static str) -> EDResult<()> {
        let component_path = self.make_component_path(epath, component_filename);
        remove_file(&component_path)?;
        Ok(())
    }

    fn read_component_file(&self, file: &mut fs::File, filepath: &Path) -> EDResult<Vec<u8>> {
        let mut component_file_content = Vec::new();
        iofile_err_map!(file.read_to_end(&mut component_file_content), filepath)?;
        Ok(component_file_content)
    }

    fn make_component_path(&self, epath: &Path, component: &str) -> PathBuf {
        self.path.join(epath).join(component)
    }
}
