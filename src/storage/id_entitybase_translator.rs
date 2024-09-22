use super::{filename_from_id, id_from_filename, StorageError, Translator};

use crate::error_ext::*;
use crate::types::{EntityBase, ID};

use std::fs::{File, OpenOptions};
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Read, Write};
use std::path::{Path, PathBuf};

use bincode::{
    deserialize as bincode_deserialize, serialize as bincode_serialize, Error as BincodeError,
};
use thiserror::Error;

const ID_EB_TRANSLATIONS_DIR: &str = "ideb_t";

#[derive(Debug, Error)]
pub enum IDEntitybaseTError {
    #[error("couldn't make a translator because it already exists")]
    TranslatorAlreadyExists,
    #[error("couldn't open a translator because it doesn't exist")]
    TranslatorDoesNotExist,
    #[error("couldn't find the directory: {0}")]
    DirectoryDoesNotExist(PathBuf),
    #[error("couldn't find the file: {0}")]
    FileDoesNotExist(PathBuf),
    #[error("the entitybase linked to the {0} ID already exists")]
    EntitybaseAlreadyExists(ID),
    #[error("there's no entitybase linked to {0} ID")]
    EntitybaseDoesNotExist(ID),
    #[error("serialization/deserialization error: {0}")]
    SerDeserError(#[from] BincodeError),
    #[error("an I/O error occured: {0}")]
    IO(#[from] IoError),

    #[error("{0}")]
    Other(#[from] ComError),
}

pub struct IDEntitybaseTranslator {
    translations_dir: PathBuf,
}

impl IDEntitybaseTranslator {
    pub fn open(working_dir: &Path) -> Result<Self, IDEntitybaseTError> {
        let translations_dir = working_dir.join(ID_EB_TRANSLATIONS_DIR);

        if !translations_dir.exists() {
            return Err(IDEntitybaseTError::TranslatorDoesNotExist.into());
        }

        Ok(IDEntitybaseTranslator { translations_dir })
    }

    pub fn create(working_dir: &Path) -> Result<Self, IDEntitybaseTError> {
        let translations_dir = working_dir.join(ID_EB_TRANSLATIONS_DIR);

        match std::fs::create_dir(&translations_dir) {
            Err(io_error) if io_error.kind() == IoErrorKind::AlreadyExists => {
                return Err(IDEntitybaseTError::TranslatorAlreadyExists)
            }
            Err(io_error) => return Err(io_error.into()),
            Ok(_) => (),
        }

        Ok(IDEntitybaseTranslator { translations_dir })
    }

    fn translate_inner(&self, key: ID) -> Result<Option<EntityBase>, IDEntitybaseTError> {
        let entitybase_filename = filename_from_id(key);
        let mut file = match File::open(self.translations_dir.join(entitybase_filename)) {
            Ok(file) => file,
            Err(io_error) if io_error.kind() == IoErrorKind::NotFound => return Ok(None),
            Err(io_error) => return Err(io_error.into()),
        };

        let mut serialized_entitybase: Vec<u8> = Vec::new();
        file.read_to_end(&mut serialized_entitybase)?;
        let entitybase: EntityBase = bincode_deserialize(&serialized_entitybase)?;

        Ok(Some(entitybase))
    }

    fn keys_inner(&self) -> Result<Vec<ID>, IDEntitybaseTError> {
        let mut translations = Vec::new();

        for entry in self.translations_dir.read_dir()? {
            let entry = entry?;

            let id = id_from_filename(entry.file_name()).commonize()?;
            translations.push(id);
        }

        Ok(translations)
    }

    fn add_translation_inner(
        &mut self,
        key: ID,
        value: EntityBase,
    ) -> Result<(), IDEntitybaseTError> {
        let entitybase_filename = filename_from_id(key);
        let mut file = match File::create_new(self.translations_dir.join(entitybase_filename)) {
            Ok(file) => file,
            Err(io_error) if io_error.kind() == IoErrorKind::AlreadyExists => {
                return Err(IDEntitybaseTError::EntitybaseAlreadyExists(key));
            }
            Err(io_error) => return Err(io_error.into()),
        };

        file.write_all(&bincode_serialize(&value)?)?;

        Ok(())
    }

    fn del_translation_inner(&mut self, key: ID) -> Result<EntityBase, IDEntitybaseTError> {
        let entitybase_filename = filename_from_id(key);
        let entitybase_file_path = self.translations_dir.join(entitybase_filename);

        let mut file = File::open(&entitybase_file_path)?;
        let mut serialized_entitybase: Vec<u8> = Vec::new();
        file.read_to_end(&mut serialized_entitybase)?;
        let entitybase = bincode_deserialize(&serialized_entitybase)?;

        std::fs::remove_file(&entitybase_file_path)?;

        Ok(entitybase)
    }

    fn update_translation_inner(
        &mut self,
        key: ID,
        new_value: EntityBase,
    ) -> Result<EntityBase, IDEntitybaseTError> {
        let entitybase_filename = filename_from_id(key);
        let entitybase_file_path = self.translations_dir.join(entitybase_filename);

        let mut read_file = match File::open(&entitybase_file_path) {
            Ok(file) => file,
            Err(io_error) if io_error.kind() == IoErrorKind::NotFound => {
                return Err(IDEntitybaseTError::EntitybaseDoesNotExist(key))
            }
            Err(io_error) => return Err(io_error.into()),
        };
        let mut serialized_old_entitybase: Vec<u8> = Vec::new();
        read_file.read_to_end(&mut serialized_old_entitybase)?;
        let old_entitybase = bincode_deserialize(&serialized_old_entitybase)?;
        drop(read_file);

        // Without catching `io_error` whose kind is `NotFount`. It is because of we already know
        // that file `entitybase_file_path` points on exists.
        let mut write_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&entitybase_file_path)?;
        write_file.write_all(&bincode_serialize(&new_value)?)?;

        Ok(old_entitybase)
    }
}

impl Translator<ID, EntityBase> for IDEntitybaseTranslator {
    fn translate(&self, key: ID) -> Result<Option<EntityBase>, StorageError> {
        Ok(self.translate_inner(key)?)
    }

    fn keys(&self) -> Result<Vec<ID>, StorageError> {
        Ok(self.keys_inner()?)
    }

    fn add_translation(&mut self, key: ID, value: EntityBase) -> Result<(), StorageError> {
        Ok(self.add_translation_inner(key, value)?)
    }

    fn del_translation(&mut self, key: ID) -> Result<EntityBase, StorageError> {
        Ok(self.del_translation_inner(key)?)
    }

    fn update_translation(
        &mut self,
        key: ID,
        new_value: EntityBase,
    ) -> Result<EntityBase, StorageError> {
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
