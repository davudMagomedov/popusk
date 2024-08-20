use crate::entity_base::*;
use crate::error_ext::ComError;
use crate::id::{IDError, ID};
use crate::progress::Progress;

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use thiserror::Error;

mod available_id_list;
mod id_entitybase_translator;
mod id_progress_translator;
mod path_id_translator;

use available_id_list::AvailableIDList;
use id_entitybase_translator::{IDEntitybaseTError, IDEntitybaseTranslator};
use id_progress_translator::{IDProgressTError, IDProgressTranslator};
use path_id_translator::{PathIDTError, PathIdTranslator};

/// Name of working directory. Must contain dot in the start to be hidden.
pub const DEFAULT_WORKING_DIR: &str = ".popusk";

pub(self) fn filename_from_id(id: ID) -> OsString {
    OsString::from(id.to_string())
}

pub(self) fn id_from_filename(string: OsString) -> Result<ID, IDError> {
    ID::from_str(&string.to_string_lossy())
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("id->entitybase translator: {0}")]
    IDEntitybaseT(#[from] IDEntitybaseTError),
    #[error("id->progress translator: {0}")]
    IDProgressT(#[from] IDProgressTError),
    #[error("path->id translator: {0}")]
    PathIDT(#[from] PathIDTError),

    #[error("{0}")]
    Other(#[from] ComError),
}

/// `Translator` provides interface for filesystem entities which keeps (or make view of keeping)
/// translation `From -> To`.
///
/// Functions `translate`, `add_translation` and `translate_mut` may work with RAM only and `load`
/// and `store` function may interconnect RAM and Disk.
trait Translator<From, To> {
    fn translate(&self, key: From) -> Result<Option<To>, StorageError>;
    fn keys(&self) -> Result<Vec<From>, StorageError>;
    fn add_translation(&mut self, key: From, value: To) -> Result<(), StorageError>;
    fn update_translation(&mut self, key: From, new_value: To) -> Result<To, StorageError>;
    fn del_translation(&mut self, key: From) -> Result<To, StorageError>;
    fn load(&mut self) -> Result<(), StorageError>;
    fn store(&mut self) -> Result<(), StorageError>;
}

/// Each filesystem entity should care about not to open files or directories immediatly after
/// creating them.
pub struct Storage {
    path_id_translator: Box<dyn Translator<PathBuf, ID>>,
    id_entitybase_translator: Box<dyn Translator<ID, EntityBase>>,
    id_progress_translator: Box<dyn Translator<ID, Progress>>,
    ail: AvailableIDList,
}

impl Storage {
    pub fn open() -> Result<Self, StorageError> {
        let working_dir = PathBuf::from(DEFAULT_WORKING_DIR);

        Ok(Storage {
            path_id_translator: Box::new(PathIdTranslator::open(&working_dir)?),
            id_entitybase_translator: Box::new(IDEntitybaseTranslator::open(&working_dir)?),
            id_progress_translator: Box::new(IDProgressTranslator::open(&working_dir)?),
            ail: AvailableIDList::open(&working_dir)?,
        })
    }

    pub fn create() -> Result<Self, StorageError> {
        let working_dir = PathBuf::from(DEFAULT_WORKING_DIR);

        Ok(Storage {
            path_id_translator: Box::new(PathIdTranslator::create(&working_dir)?),
            id_entitybase_translator: Box::new(IDEntitybaseTranslator::create(&working_dir)?),
            id_progress_translator: Box::new(IDProgressTranslator::create(&working_dir)?),
            ail: AvailableIDList::create(&working_dir)?,
        })
    }

    pub fn open_with_working_dir(working_dir: &Path) -> Result<Self, StorageError> {
        Ok(Storage {
            path_id_translator: Box::new(PathIdTranslator::open(working_dir)?),
            id_entitybase_translator: Box::new(IDEntitybaseTranslator::open(&working_dir)?),
            id_progress_translator: Box::new(IDProgressTranslator::open(&working_dir)?),
            ail: AvailableIDList::open(&working_dir)?,
        })
    }

    pub fn create_with_working_dir(working_dir: &Path) -> Result<Self, StorageError> {
        Ok(Storage {
            path_id_translator: Box::new(PathIdTranslator::create(working_dir)?),
            id_entitybase_translator: Box::new(IDEntitybaseTranslator::create(working_dir)?),
            id_progress_translator: Box::new(IDProgressTranslator::create(working_dir)?),
            ail: AvailableIDList::create(working_dir)?,
        })
    }

    pub fn link_id_to_path(&mut self, path: PathBuf) -> Result<ID, StorageError> {
        let unique_id = self.ail.grab_id()?;
        self.path_id_translator.add_translation(path, unique_id)?;

        Ok(unique_id)
    }

    pub fn unlink_id_from_path(&mut self, path: PathBuf) -> Result<ID, StorageError> {
        let id = self.path_id_translator.del_translation(path)?;
        self.ail.release_id(id)?;

        Ok(id)
    }

    pub fn link_progress_to_id(&mut self, id: ID, progress: Progress) -> Result<(), StorageError> {
        self.id_progress_translator.add_translation(id, progress)
    }

    pub fn unlink_progress_from_id(&mut self, id: ID) -> Result<Progress, StorageError> {
        self.id_progress_translator.del_translation(id)
    }

    pub fn update_progress(
        &mut self,
        id: ID,
        new_progress: Progress,
    ) -> Result<Progress, StorageError> {
        self.id_progress_translator
            .update_translation(id, new_progress)
    }

    pub fn link_entitybase_to_id(
        &mut self,
        id: ID,
        entitybase: EntityBase,
    ) -> Result<(), StorageError> {
        self.id_entitybase_translator
            .add_translation(id, entitybase)
    }

    pub fn unlink_entitybase_from_id(&mut self, id: ID) -> Result<EntityBase, StorageError> {
        self.id_entitybase_translator.del_translation(id)
    }

    pub fn update_entitybase(
        &mut self,
        id: ID,
        new_entitybase: EntityBase,
    ) -> Result<EntityBase, StorageError> {
        self.id_entitybase_translator
            .update_translation(id, new_entitybase)
    }

    pub fn get_id(&self, path: PathBuf) -> Result<Option<ID>, StorageError> {
        self.path_id_translator.translate(path)
    }

    pub fn get_progress(&self, id: ID) -> Result<Option<Progress>, StorageError> {
        self.id_progress_translator.translate(id)
    }

    pub fn get_entitybase(&self, id: ID) -> Result<Option<EntityBase>, StorageError> {
        self.id_entitybase_translator.translate(id)
    }

    pub fn keys_path(&self) -> Result<Vec<PathBuf>, StorageError> {
        self.path_id_translator.keys()
    }
}
