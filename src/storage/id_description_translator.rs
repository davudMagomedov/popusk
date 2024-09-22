use super::{filename_from_id, id_from_filename, StorageError, Translator};

use crate::error_ext::{ComError, CommonizeResultExt};
use crate::types::ID;

use std::fs::{File, OpenOptions};
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Read, Write};
use std::path::{Path, PathBuf};

use bincode::{
    deserialize as bincode_deserialize, serialize as bincode_serialize, Error as BincodeError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IDDescTError {
    #[error("couldn't make a translator because it already exists")]
    TranslatorAlreadyExists,
    #[error("couldn't open a translator because it doesn't exist")]
    TranslatorDoesNotExist,
    #[error("directory doesn't exist: {0}")]
    DirectoryDoesNotExist(PathBuf),
    #[error("couldn't find the file: {0}")]
    FileDoesNotExist(PathBuf),
    #[error("description doesn't exist for ID {id}")]
    DescDoesNotExist { id: ID },
    #[error("description already exists for ID {id}")]
    DescAlreadyExists { id: ID },
    #[error("serialization/deserialization error: {0}")]
    SerDeserError(#[from] BincodeError),
    #[error("an I/O error occured: {0}")]
    IO(#[from] IoError),

    #[error("{0}")]
    Other(#[from] ComError),
}

const ID_DESC_TRANSLATIONS_DIR: &str = "iddesc_t";

fn translations_dir(working_dir: &Path) -> PathBuf {
    working_dir.join(ID_DESC_TRANSLATIONS_DIR)
}

pub struct IDDescriptionTranslator {
    translations_dir: PathBuf,
}

impl IDDescriptionTranslator {
    pub fn open(working_dir: &Path) -> Result<Self, IDDescTError> {
        let translations_dir = translations_dir(working_dir);

        if !translations_dir.exists() {
            return Err(IDDescTError::TranslatorDoesNotExist);
        }

        Ok(IDDescriptionTranslator { translations_dir })
    }

    pub fn create(working_dir: &Path) -> Result<Self, IDDescTError> {
        let translations_dir = translations_dir(working_dir);

        match std::fs::create_dir(&translations_dir) {
            Err(io_error) if io_error.kind() == IoErrorKind::AlreadyExists => {
                return Err(IDDescTError::TranslatorAlreadyExists)
            }
            Err(io_error) => return Err(io_error.into()),
            Ok(_) => (),
        }

        Ok(IDDescriptionTranslator { translations_dir })
    }

    fn translate_inner(&self, key: ID) -> Result<Option<String>, IDDescTError> {
        let desc_filename = filename_from_id(key);
        let desc_file_path = self.translations_dir.join(desc_filename);

        let mut file = match File::open(&desc_file_path) {
            Ok(file) => file,
            Err(io_error) if io_error.kind() == IoErrorKind::NotFound => {
                return Ok(None);
            }
            Err(io_error) => return Err(io_error.into()),
        };
        let mut serialized_description: Vec<u8> = Vec::new();
        file.read_to_end(&mut serialized_description)?;
        let description: String = bincode_deserialize(&serialized_description)?;

        Ok(Some(description))
    }

    fn keys_inner(&self) -> Result<Vec<ID>, IDDescTError> {
        let mut translations = Vec::new();

        for entry in self.translations_dir.read_dir()? {
            let entry = entry?;

            let id = id_from_filename(entry.file_name()).commonize()?;
            translations.push(id);
        }

        Ok(translations)
    }

    fn add_translation_inner(&mut self, id: ID, desc: String) -> Result<(), IDDescTError> {
        let desc_filename = filename_from_id(id);
        let desc_file_path = self.translations_dir.join(desc_filename);

        let serialized_desc = bincode_serialize(&desc)?;
        let mut file = match File::create_new(&desc_file_path) {
            Ok(file) => file,
            Err(io_error) if io_error.kind() == IoErrorKind::AlreadyExists => {
                return Err(IDDescTError::DescAlreadyExists { id })
            }

            Err(io_error) => return Err(io_error.into()),
        };
        file.write_all(&serialized_desc)?;

        Ok(())
    }

    fn del_translation_inner(&mut self, id: ID) -> Result<String, IDDescTError> {
        let desc_filename = filename_from_id(id);
        let desc_file_path = self.translations_dir.join(desc_filename);

        let mut file = match File::open(&desc_file_path) {
            Ok(file) => file,
            Err(io_error) if io_error.kind() == IoErrorKind::NotFound => {
                return Err(IDDescTError::DescDoesNotExist { id });
            }
            Err(io_error) => {
                return Err(io_error.into());
            }
        };
        let mut serialized_desc: Vec<u8> = Vec::new();
        file.read_to_end(&mut serialized_desc)?;
        let description: String = bincode_deserialize(&serialized_desc)?;

        std::fs::remove_file(&desc_file_path)?;

        Ok(description)
    }

    fn update_translation_inner(
        &mut self,
        id: ID,
        new_desc: String,
    ) -> Result<String, IDDescTError> {
        let desc_filename = filename_from_id(id);
        let desc_file_path = self.translations_dir.join(desc_filename);

        let mut read_file = match File::open(&desc_file_path) {
            Ok(file) => file,
            Err(io_error) if io_error.kind() == IoErrorKind::NotFound => {
                return Err(IDDescTError::DescDoesNotExist { id });
            }
            Err(io_error) => return Err(io_error.into()),
        };
        let mut serialized_old_desc: Vec<u8> = Vec::new();
        read_file.read_to_end(&mut serialized_old_desc)?;
        let old_desc = bincode_deserialize(&serialized_old_desc)?;
        drop(read_file);

        // Without catching `io_error` whose kind is `NotFount`. It is because of we already know
        // that file `entitybase_file_path` points on exists.
        let mut write_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&desc_file_path)?;
        write_file.write_all(&bincode_serialize(&new_desc)?)?;

        Ok(old_desc)
    }
}

impl Translator<ID, String> for IDDescriptionTranslator {
    fn translate(&self, key: ID) -> Result<Option<String>, StorageError> {
        Ok(self.translate_inner(key)?)
    }

    fn keys(&self) -> Result<Vec<ID>, StorageError> {
        Ok(self.keys_inner()?)
    }

    fn add_translation(&mut self, key: ID, value: String) -> Result<(), StorageError> {
        Ok(self.add_translation_inner(key, value)?)
    }

    fn del_translation(&mut self, key: ID) -> Result<String, StorageError> {
        Ok(self.del_translation_inner(key)?)
    }

    fn update_translation(&mut self, key: ID, new_value: String) -> Result<String, StorageError> {
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
