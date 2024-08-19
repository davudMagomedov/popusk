use super::{StorageError, Translator};

use crate::id::ID;

use std::ffi::OsStr;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::{
    ffi::OsString,
    fs::{File, OpenOptions},
    hint::unreachable_unchecked,
    io::{Error as IoError, ErrorKind as IoErrorKind, Read, Write},
    path::{Component, Path, PathBuf},
};

use anyhow::Result;
use bincode::{
    deserialize as bincode_deserialize, serialize as bincode_serialize, Error as BincodeError,
};
use thiserror::Error;

const PATH_ID_TRANSLATIONS_DIR: &str = "pathid_t";

const NAME_SEPARATOR: u8 = 0x0A;

#[derive(Debug, Error)]
pub enum PathIDTError {
    #[error("using an absolute path: {0}")]
    UsingAbsolutePath(PathBuf),
    #[error("using a path that goes beyond the current directory: {0}")]
    UsingUnlocalPath(PathBuf),
    #[error("couldn't find the directory: {0}")]
    DirectoryDoesNotExist(PathBuf),
    #[error("couldn't find the file: {0}")]
    FileDoesNotExist(PathBuf),
    #[error("serialization/deserialization error: {0}")]
    SerDeserError(#[from] BincodeError),
    #[error("an I/O error occured: {0}")]
    IO(#[from] IoError),

    #[error("{0}")]
    Anyhow(#[from] anyhow::Error),
}

/// Translates various variations of essentially the same paths into one common form.
///
/// ## Examples
/// 1. `./dir/../dir2/file` => `dir2/file`
/// 2. `a/./b/../c/` => `a/c`
fn simplify_path_to_local(path: &Path) -> Result<PathBuf, PathIDTError> {
    let mut new_components: Vec<Component> = Vec::new();

    // '... a/./c/ ...' => '... a/c ...'.
    // '... a/../c ...' => '... c/ ...'.
    // '/ ...' => Error
    // '../ ...' => Error
    for component in path.components() {
        match component {
            Component::CurDir => (),
            Component::ParentDir if (new_components.len() > 0) => {
                new_components.pop();
            }
            Component::ParentDir => return Err(PathIDTError::UsingUnlocalPath(path.to_path_buf())),
            Component::RootDir => return Err(PathIDTError::UsingAbsolutePath(path.to_path_buf())),
            Component::Normal(name) => new_components.push(Component::Normal(name)),
            // Because of #![cfg(unix)] in the start of the main.rs file.
            _ => unsafe { unreachable_unchecked() },
        }
    }

    Ok(PathBuf::from_iter(new_components))
}

/// Converts given path to string (`OsString`) which can be used as file name.
///
/// ## Assertions
/// - Given path contains of only names.
fn serialize_path(path: &Path) -> OsString {
    debug_assert!(path.components().all(|c| matches!(c, Component::Normal(_))));

    let mut bytes = Vec::new();
    let mut components = path.components();

    if let Some(Component::Normal(name)) = components.next() {
        bytes.extend(name.as_bytes());
    }
    while let Some(Component::Normal(name)) = components.next() {
        bytes.push(NAME_SEPARATOR);
        bytes.extend(name.as_bytes());
    }

    OsString::from_vec(bytes)
}

fn standart_path_form(path: &Path) -> Result<PathBuf, PathIDTError> {
    let simplified_local = simplify_path_to_local(path)?;
    let serialized = serialize_path(&simplified_local);

    Ok(PathBuf::from(serialized))
}

fn deserialize_spf(spf: OsString) -> PathBuf {
    PathBuf::from_iter(
        spf.as_bytes()
            .split(|t| *t == NAME_SEPARATOR)
            .map(|name| Component::Normal(OsStr::from_bytes(name))),
    )
}

// FS Structure:
// <working_dir>
//     <translations_dir>
//         <polished_path_1>: id_1[8 bytes]
//         <polished_path_2>: id_2[8 bytes]
//         ...
pub struct PathIdTranslator {
    translations_dir: PathBuf,
}

// TODO: The most of functions return Error in the wrong way.
impl PathIdTranslator {
    pub fn open(working_directory: &Path) -> Result<Self, PathIDTError> {
        let translations_dir = working_directory.join(PATH_ID_TRANSLATIONS_DIR);

        if !translations_dir.exists() {
            return Err(PathIDTError::DirectoryDoesNotExist(translations_dir).into());
        }

        Ok(PathIdTranslator { translations_dir })
    }

    pub fn create(working_directory: &Path) -> Result<Self, PathIDTError> {
        let translations_dir = working_directory.join(PATH_ID_TRANSLATIONS_DIR);

        std::fs::create_dir(&translations_dir)?;

        Ok(PathIdTranslator { translations_dir })
    }

    fn translate_inner(&self, key: PathBuf) -> Result<Option<ID>, PathIDTError> {
        let standart = self.translations_dir.join(standart_path_form(&key)?);

        let mut file = match File::open(&standart) {
            Ok(file) => file,
            Err(io_error) if io_error.kind() == IoErrorKind::NotFound => return Ok(None),
            Err(io_error) => return Err(io_error.into()),
        };
        let mut serialized_id = Vec::new();
        file.read_to_end(&mut serialized_id)?;
        let id: ID = bincode_deserialize(&serialized_id)?;

        Ok(Some(id))
    }

    fn keys_inner(&self) -> Result<Vec<PathBuf>, PathIDTError> {
        let mut translations = Vec::new();

        for entry in self.translations_dir.read_dir()? {
            let entry = entry?;

            let path = deserialize_spf(entry.file_name());
            translations.push(path);
        }

        Ok(translations)
    }

    fn add_translation_inner(&mut self, key: PathBuf, value: ID) -> Result<(), PathIDTError> {
        let standart = self.translations_dir.join(standart_path_form(&key)?);

        let mut file = File::create_new(&standart)?;
        let serialized_id = bincode_serialize(&value)?;
        file.write_all(&serialized_id)?;

        Ok(())
    }

    fn del_translation_inner(&mut self, key: PathBuf) -> Result<ID, PathIDTError> {
        let standart = self.translations_dir.join(standart_path_form(&key)?);

        let mut file = File::open(&standart)?;
        let mut serialized_id = Vec::new();
        file.read_to_end(&mut serialized_id)?;
        let id: ID = bincode_deserialize(&serialized_id)?;

        std::fs::remove_file(standart)?;

        Ok(id)
    }

    fn update_translation_inner(
        &mut self,
        key: PathBuf,
        new_value: ID,
    ) -> Result<ID, PathIDTError> {
        let standart = self.translations_dir.join(standart_path_form(&key)?);

        let mut read_file = File::open(&standart)?;
        let mut serialized_old_id = Vec::new();
        read_file.read_to_end(&mut serialized_old_id)?;
        let old_id: ID = bincode_deserialize(&serialized_old_id)?;
        drop(read_file);

        let mut write_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&standart)?;
        let serialized_new_id = bincode_serialize(&new_value)?;
        write_file.write_all(&serialized_new_id)?;

        Ok(old_id)
    }
}

impl Translator<PathBuf, ID> for PathIdTranslator {
    fn translate(&self, key: PathBuf) -> Result<Option<ID>, StorageError> {
        Ok(self.translate_inner(key)?)
    }

    fn keys(&self) -> Result<Vec<PathBuf>, StorageError> {
        Ok(self.keys_inner()?)
    }

    fn add_translation(&mut self, key: PathBuf, value: ID) -> Result<(), StorageError> {
        Ok(self.add_translation_inner(key, value)?)
    }

    fn del_translation(&mut self, key: PathBuf) -> Result<ID, StorageError> {
        Ok(self.del_translation_inner(key)?)
    }

    fn update_translation(&mut self, key: PathBuf, new_value: ID) -> Result<ID, StorageError> {
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
