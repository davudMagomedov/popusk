use super::{filename_from_id, id_from_filename, StorageError, Translator};

use crate::error_ext::{ComError, CommonizeResultExt};
use crate::id::ID;
use crate::progress::Progress;

use std::fs::{File, OpenOptions};
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Read, Write};
use std::path::{Path, PathBuf};

use bincode::{
    deserialize as bincode_deserialize, serialize as bincode_serialize, Error as BincodeError,
};
use thiserror::Error;

const ID_PROGRESS_TRANSLATIONS_DIR: &str = "idprog_t";

fn translations_dir(working_dir: &Path) -> PathBuf {
    working_dir.join(ID_PROGRESS_TRANSLATIONS_DIR)
}

#[derive(Debug, Error)]
pub enum IDProgressTError {
    #[error("couldn't find the directory: {0}")]
    DirectoryDoesNotExist(PathBuf),
    #[error("couldn't find the file: {0}")]
    FileDoesNotExist(PathBuf),
    #[error("the progress associated with the {0} ID could not be found")]
    ProgressDoesNotExist(ID),
    #[error("progress related to the {0} ID already exists")]
    ProgressAlreadyExists(ID),
    #[error("serialization/deserialization error: {0}")]
    SerDeserError(#[from] BincodeError),
    #[error("an I/O error occured: {0}")]
    IO(#[from] IoError),

    #[error("{0}")]
    Other(#[from] ComError),
}

pub struct IDProgressTranslator {
    translations_dir: PathBuf,
}

impl IDProgressTranslator {
    pub fn open(working_dir: &Path) -> Result<Self, IDProgressTError> {
        let translations_dir = translations_dir(working_dir);

        if !translations_dir.exists() {
            return Err(IDProgressTError::DirectoryDoesNotExist(translations_dir));
        }

        Ok(IDProgressTranslator { translations_dir })
    }

    pub fn create(working_dir: &Path) -> Result<Self, IDProgressTError> {
        let translations_dir = translations_dir(working_dir);

        std::fs::create_dir(&translations_dir)?;

        Ok(IDProgressTranslator { translations_dir })
    }

    fn translate_inner(&self, key: ID) -> Result<Option<Progress>, IDProgressTError> {
        let progress_filename = filename_from_id(key);
        let progress_file_path = self.translations_dir.join(progress_filename);

        let mut file = match File::open(&progress_file_path) {
            Ok(file) => file,
            Err(io_error) if io_error.kind() == IoErrorKind::NotFound => {
                return Ok(None);
            }
            Err(io_error) => return Err(io_error.into()),
        };
        let mut serialized_progress: Vec<u8> = Vec::new();
        file.read_to_end(&mut serialized_progress)?;
        let progress: Progress = bincode_deserialize(&serialized_progress)?;

        Ok(Some(progress))
    }

    fn keys_inner(&self) -> Result<Vec<ID>, IDProgressTError> {
        let mut translations = Vec::new();

        for entry in self.translations_dir.read_dir()? {
            let entry = entry?;

            let id = id_from_filename(entry.file_name()).commonize()?;
            translations.push(id);
        }

        Ok(translations)
    }

    fn add_translation_inner(&mut self, key: ID, value: Progress) -> Result<(), IDProgressTError> {
        let progress_filename = filename_from_id(key);
        let progress_file_path = self.translations_dir.join(progress_filename);

        let serialized_progress = bincode_serialize(&value)?;
        let mut file = match File::create_new(&progress_file_path) {
            Ok(file) => file,
            Err(io_error) if io_error.kind() == IoErrorKind::AlreadyExists => {
                return Err(IDProgressTError::ProgressAlreadyExists(key))
            }

            Err(io_error) => return Err(io_error.into()),
        };
        file.write_all(&serialized_progress)?;

        Ok(())
    }

    fn del_translation_inner(&mut self, key: ID) -> Result<Progress, IDProgressTError> {
        let progress_filename = filename_from_id(key);
        let progress_file_path = self.translations_dir.join(progress_filename);

        let mut file = match File::open(&progress_file_path) {
            Ok(file) => file,
            Err(io_error) if io_error.kind() == IoErrorKind::NotFound => {
                return Err(IDProgressTError::ProgressDoesNotExist(key));
            }
            Err(io_error) => {
                return Err(io_error.into());
            }
        };
        let mut serialized_progress: Vec<u8> = Vec::new();
        file.read_to_end(&mut serialized_progress)?;
        let progress: Progress = bincode_deserialize(&serialized_progress)?;

        std::fs::remove_file(&progress_file_path)?;

        Ok(progress)
    }

    fn update_translation_inner(
        &mut self,
        key: ID,
        new_value: Progress,
    ) -> Result<Progress, IDProgressTError> {
        let progress_filename = filename_from_id(key);
        let progress_file_path = self.translations_dir.join(progress_filename);

        let mut read_file = match File::open(&progress_file_path) {
            Ok(file) => file,
            Err(io_error) if io_error.kind() == IoErrorKind::NotFound => {
                return Err(IDProgressTError::ProgressDoesNotExist(key));
            }
            Err(io_error) => return Err(io_error.into()),
        };
        let mut serialized_old_progress: Vec<u8> = Vec::new();
        read_file.read_to_end(&mut serialized_old_progress)?;
        let old_entitybase = bincode_deserialize(&serialized_old_progress)?;
        drop(read_file);

        // Without catching `io_error` whose kind is `NotFount`. It is because of we already know
        // that file `entitybase_file_path` points on exists.
        let mut write_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&progress_file_path)?;
        write_file.write_all(&bincode_serialize(&new_value)?)?;

        Ok(old_entitybase)
    }
}

impl Translator<ID, Progress> for IDProgressTranslator {
    fn translate(&self, key: ID) -> Result<Option<Progress>, StorageError> {
        Ok(self.translate_inner(key)?)
    }

    fn keys(&self) -> Result<Vec<ID>, StorageError> {
        Ok(self.keys_inner()?)
    }

    fn add_translation(&mut self, key: ID, value: Progress) -> Result<(), StorageError> {
        Ok(self.add_translation_inner(key, value)?)
    }

    fn del_translation(&mut self, key: ID) -> Result<Progress, StorageError> {
        Ok(self.del_translation_inner(key)?)
    }

    fn update_translation(
        &mut self,
        key: ID,
        new_value: Progress,
    ) -> Result<Progress, StorageError> {
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
