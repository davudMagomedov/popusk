use crate::storage::{Storage, StorageError};
use crate::types::{ID, LibEntityMetaError, LibEntityMut,
                   LibEntityConst, LibEntityData, EntityBase};

use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;

use thiserror::Error;

pub type LibraryResult<T> = Result<T, LibraryError>;

#[derive(Debug, Error)]
pub enum LibraryError {
    #[error("storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("couldn't find library entity with path '{path}'")]
    CouldNotFindLibEntity { path: PathBuf },
    #[error("couldn't find {element} for library entity with path {path}")]
    CouldNotFindElement { element: String, path: PathBuf },
    #[error("couldn't find {element} for library entity with ID {id}")]
    CouldNotFindElementWithID { element: String, id: ID },

    #[error("library entity: {0}")]
    LibEntityMeta(#[from] LibEntityMetaError),
}

/// Implements operations with library entities (`LibEntity`) through storage (`Storage`).
pub struct Library {
    storage: Rc<RefCell<Storage>>,
}

impl Library {
    pub fn new(storage: Storage) -> Self {
        Library { storage: Rc::new(RefCell::new(storage)) }
    }

    pub fn get_libentity(&self, path: PathBuf) -> LibraryResult<Option<LibEntityConst>> {
        let maybe_libentity = LibEntityConst::new_with_path(self.storage(), path);
        match maybe_libentity {
            Ok(libentity) => Ok(Some(libentity)),
            Err(LibEntityMetaError::CouldNotFindLibEntityWithPath { .. }) => Ok(None),
            Err(other_error) => Err(other_error.into()),
        }
    }

    pub fn get_libentity_by_id(&self, id: ID) -> LibraryResult<Option<LibEntityConst>> {
        let maybe_libentity = LibEntityConst::new_with_id(self.storage(), id);
        match maybe_libentity {
            Ok(libentity) => Ok(Some(libentity)),
            Err(LibEntityMetaError::CouldNotFindLibEntityWithPath { .. }) => Ok(None),
            Err(other_error) => Err(other_error.into()),
        }
    }

    pub fn get_libentity_mut(&mut self, path: PathBuf) -> LibraryResult<Option<LibEntityMut>> {
        let maybe_libentity = LibEntityMut::new_with_path(self.storage(), path);
        match maybe_libentity {
            Ok(libentity) => Ok(Some(libentity)),
            Err(LibEntityMetaError::CouldNotFindLibEntityWithPath { .. }) => Ok(None),
            Err(other_error) => Err(other_error.into()),
        }
    }

    pub fn get_libentity_mut_by_id(&mut self, id: ID) -> LibraryResult<Option<LibEntityMut>> {
        let maybe_libentity = LibEntityMut::new_with_id(self.storage(), id);
        match maybe_libentity {
            Ok(libentity) => Ok(Some(libentity)),
            Err(LibEntityMetaError::CouldNotFindLibEntityWithPath { .. }) => Ok(None),
            Err(other_error) => Err(other_error.into()),
        }
    }

    /// Creates bare library entity, without progress and even without entitybase.
    ///
    /// SAFETY: satisfy library entity rules such as necessary entitybase.
    pub unsafe fn create_empty_libentity(&mut self, path: PathBuf) -> LibraryResult<LibEntityMut> {
        let id = self.link_id_to_path(path.clone())?;
        let libentity = unsafe { 
            LibEntityMut::new_with_path_id_unchecked(self.storage(), id, path)?
        };

        Ok(libentity)
    }

    /// Creates library entity without dumping to storage.
    pub fn create_libentity_from_libentitydata(&mut self, libentity_data: LibEntityData)
    -> LibraryResult<LibEntityMut> {
        // TODO: This is shit code. It uses LibEntityMut for creating a libentity, whereas
        // LibEntityMut is created with suggestion that the libentity already exists! This
        // definetly should be changed.

        let LibEntityData {
            path, name, etype, tags, progress, description, freedata
        } = libentity_data;

        let mut libentity = unsafe { self.create_empty_libentity(path)? };
        let id = libentity.id();

        self.link_ebase_to_id(id, EntityBase::new(id, name, etype, tags))?;
        libentity.set_progress(progress)?;
        libentity.set_description(description)?;
        libentity.set_freedata(freedata)?;

        Ok(libentity)
    }

    pub fn storage(&self) -> Rc<RefCell<Storage>> {
        Rc::clone(&self.storage)
    }

    pub fn libentity_exists(&self, path: PathBuf) -> LibraryResult<bool> {
        Ok(self.storage.borrow().get_id(path)?.is_some())
    }

    unsafe fn link_id_to_path(&mut self, path: PathBuf) -> LibraryResult<ID> {
        Ok(self.storage().borrow_mut().link_id_to_path(path.clone())?)
    }
    
    fn link_ebase_to_id(&mut self, id: ID, ebase: EntityBase) -> LibraryResult<()> {
        Ok(self.storage.borrow_mut().link_entitybase_to_id(id, ebase)?)
    }
}
