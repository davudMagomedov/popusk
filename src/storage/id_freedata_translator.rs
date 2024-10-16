use super::{filename_from_id, id_from_filename, StorageError, Translator};

use crate::error_ext::{ComError, CommonizeResultExt};
use crate::types::{FreeData, ID};

use std::fs::{File, OpenOptions};
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Read, Write};
use std::path::{Path, PathBuf};

use bincode::{
    deserialize as bincode_deserialize, serialize as bincode_serialize, Error as BincodeError,
};
use thiserror::Error;

const ID_FREEDATA_TRANSLATIONS_DIR: &str = "idfreedt_t";

fn translations_dir(working_dir: &Path) -> PathBuf {
    working_dir.join(ID_FREEDATA_TRANSLATIONS_DIR)
}

#[derive(Debug, Error)]
pub enum IDFreeDataTError {
    #[error("couldn't make a translator because it already exists")]
    TranslatorAlreadyExists,
    #[error("couldn't open a translator because it doesn't exist")]
    TranslatorDoesNotExist,
    #[error("couldn't find the directory: {0}")]
    DirectoryDoesNotExist(PathBuf),
    #[error("couldn't find the file: {0}")]
    FileDoesNotExist(PathBuf),
    #[error("free data related to the {0} ID doesn't exist")]
    FreeDataDoesNotExist(ID),
    #[error("free data related to the {0} ID already exists")]
    FreeDataAlreadyExists(ID),
    #[error("serialization/deserialization error: {0}")]
    SerDeserError(#[from] BincodeError),
    #[error("an I/O error occured: {0}")]
    IO(#[from] IoError),

    #[error("{0}")]
    Other(#[from] ComError),
}

pub struct IDFreeDataTranslator {
    translations_dir: PathBuf,
}

impl IDFreeDataTranslator {
    pub fn open(working_dir: &Path) -> Result<Self, IDFreeDataTError> {
        let translations_dir = working_dir.join(ID_FREEDATA_TRANSLATIONS_DIR);

        if !translations_dir.exists() {
            return Err(IDFreeDataTError::TranslatorDoesNotExist);
        }

        Ok(IDFreeDataTranslator { translations_dir })
    }

    pub fn create(working_dir: &Path) -> Result<Self, IDFreeDataTError> {
        let translations_dir = translations_dir(working_dir);

        match std::fs::create_dir(&translations_dir) {
            Err(io_error) if io_error.kind() == IoErrorKind::AlreadyExists => {
                return Err(IDFreeDataTError::TranslatorAlreadyExists);
            }
            Err(io_error) => return Err(io_error.into()),
            Ok(_) => (),
        }

        Ok(IDFreeDataTranslator { translations_dir })
    }

    fn translate_inner(&self, key: ID) -> Result<Option<FreeData>, IDFreeDataTError> {
        let freedata_filename = filename_from_id(key);
        let freedata_file_path = self.translations_dir.join(freedata_filename);

        let mut file = match File::open(&freedata_file_path) {
            Ok(file) => file,
            Err(io_error) if io_error.kind() == IoErrorKind::NotFound => {
                return Ok(None);
            }
            Err(io_error) => return Err(io_error.into()),
        };
        let mut serialized_freedata: Vec<u8> = Vec::new();
        file.read_to_end(&mut serialized_freedata)?;
        let freedata: FreeData = bincode_deserialize(&serialized_freedata)?;

        Ok(Some(freedata))
    }

    fn keys_inner(&self) -> Result<Vec<ID>, IDFreeDataTError> {
        let mut translations = Vec::new();

        for entry in self.translations_dir.read_dir()? {
            let entry = entry?;

            let id = id_from_filename(entry.file_name()).commonize()?;
            translations.push(id);
        }

        Ok(translations)
    }

    fn add_translation_inner(&mut self, key: ID, value: FreeData) -> Result<(), IDFreeDataTError> {
        let freedata_filename = filename_from_id(key);
        let freedata_file_path = self.translations_dir.join(freedata_filename);

        let serialized_freedata = bincode_serialize(&value)?;
        let mut file = match File::create_new(&freedata_file_path) {
            Ok(file) => file,
            Err(io_error) if io_error.kind() == IoErrorKind::AlreadyExists => {
                return Err(IDFreeDataTError::FreeDataAlreadyExists(key))
            }

            Err(io_error) => return Err(io_error.into()),
        };
        file.write_all(&serialized_freedata)?;

        Ok(())
    }

    fn del_translation_inner(&mut self, key: ID) -> Result<FreeData, IDFreeDataTError> {
        let freedata_filename = filename_from_id(key);
        let freedata_file_path = self.translations_dir.join(freedata_filename);

        let mut file = match File::open(&freedata_file_path) {
            Ok(file) => file,
            Err(io_error) if io_error.kind() == IoErrorKind::NotFound => {
                return Err(IDFreeDataTError::FreeDataDoesNotExist(key));
            }
            Err(io_error) => {
                return Err(io_error.into());
            }
        };
        let mut serialized_freedata: Vec<u8> = Vec::new();
        file.read_to_end(&mut serialized_freedata)?;
        let freedata: FreeData = bincode_deserialize(&serialized_freedata)?;

        std::fs::remove_file(&freedata_file_path)?;

        Ok(freedata)
    }

    fn update_translation_inner(
        &mut self,
        key: ID,
        new_value: FreeData,
    ) -> Result<FreeData, IDFreeDataTError> {
        let freedata_filename = filename_from_id(key);
        let freedata_file_path = self.translations_dir.join(freedata_filename);

        let mut read_file = match File::open(&freedata_file_path) {
            Ok(file) => file,
            Err(io_error) if io_error.kind() == IoErrorKind::NotFound => {
                return Err(IDFreeDataTError::FreeDataDoesNotExist(key));
            }
            Err(io_error) => return Err(io_error.into()),
        };
        let mut serialized_old_freedata: Vec<u8> = Vec::new();
        read_file.read_to_end(&mut serialized_old_freedata)?;
        let old_freedata = bincode_deserialize(&serialized_old_freedata)?;
        drop(read_file);

        // Without catching `io_error` whose kind is `NotFount`. It is because of we already know
        // that file `progress_file_path` points on exists.
        let mut write_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&freedata_file_path)?;
        write_file.write_all(&bincode_serialize(&new_value)?)?;

        Ok(old_freedata)
    }

    // ...
}

impl Translator<ID, FreeData> for IDFreeDataTranslator {
    fn translate(&self, key: ID) -> Result<Option<FreeData>, StorageError> {
        Ok(self.translate_inner(key)?)
    }

    fn keys(&self) -> Result<Vec<ID>, StorageError> {
        Ok(self.keys_inner()?)
    }

    fn add_translation(&mut self, key: ID, value: FreeData) -> Result<(), StorageError> {
        Ok(self.add_translation_inner(key, value)?)
    }

    fn del_translation(&mut self, key: ID) -> Result<FreeData, StorageError> {
        Ok(self.del_translation_inner(key)?)
    }

    fn update_translation(
        &mut self,
        key: ID,
        new_value: FreeData,
    ) -> Result<FreeData, StorageError> {
        Ok(self.update_translation_inner(key, new_value)?)
    }

    fn load(&mut self) -> Result<(), StorageError> {
        // All `<Self as Translator>` functions works immediatly with file system.
        Ok(())
    }

    fn store(&mut self) -> Result<(), StorageError> {
        // All `<Self as Translator>` functions works immediatly with file system.
        Ok(())
    }
}
