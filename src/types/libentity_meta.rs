use crate::storage::{Storage, StorageError};
use crate::types::{EntityType, Progress, FreeData, ID, Tag, EntityBase};
use crate::types::LibEntityData;

use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::path::PathBuf;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum LibEntityMetaError {
    #[error("there's no library entity with path '{path}'")]
    CouldNotFindLibEntityWithPath { path: PathBuf },
    #[error("there's no library entity with id '{id}'")]
    CouldNotFindLibEntityWithID { id: ID },
    #[error("couldn't find entity base for ID {id}")]
    CouldNotFindEntityBaseForID { id: ID },

    #[error("storage error: {0}")]
    Storage(#[from] StorageError),
}

#[derive(Debug)]
struct LibEntityDataCached {
    ebase: Option<EntityBase>,
    progress: Option<Option<Progress>>,
    description: Option<Option<String>>,
    freedata: Option<Option<FreeData>>,
}

impl LibEntityDataCached {
    fn empty() -> Self {
        LibEntityDataCached {
            ebase: None,
            progress: None,
            description: None,
            freedata: None,
        }
    }
}

pub struct LibEntityMeta {
    /* Invariants:
     * 1. ID definetly exists in the storage.
     * 2. If the path is `Some(PathBuf)` then `path` is linked to the `id`
     *    and, therefore, exists in the storage. */

    storage: Rc<RefCell<Storage>>,
    path: Option<PathBuf>,
    id: ID,
    data_cached: RefCell<LibEntityDataCached>,
}

impl LibEntityMeta {
    pub fn new_with_path(storage: Rc<RefCell<Storage>>, path: PathBuf)
    -> Result<Self, LibEntityMetaError> {
        let id = match storage.borrow().get_id(path.clone())? {
            Some(id) => id,
            None => return Err(LibEntityMetaError::CouldNotFindLibEntityWithPath { path })
        };

        Ok(LibEntityMeta {
            storage, id,
            path: Some(path),
            data_cached: RefCell::new(LibEntityDataCached::empty()),
        })
    }

    pub fn new(storage: Rc<RefCell<Storage>>, id: ID)
    -> Result<Self, LibEntityMetaError> {
        if !storage.borrow().id_exists(id)? {
            return Err(LibEntityMetaError::CouldNotFindLibEntityWithID { id });
        }

        Ok(LibEntityMeta {
            storage, id,
            path: None,
            data_cached: RefCell::new(LibEntityDataCached::empty()),
        })
    }

    pub fn ebase(&self) -> Result<EntityBase, LibEntityMetaError> {
        if self.ebase_cached() {
            Ok(self.ebase_raw().unwrap())
        } else {
            self.cache_ebase(self.read_ebase_from_storage()?);
            Ok(self.ebase_raw().unwrap())
        }
    }

    pub fn name(&self) -> Result<String, LibEntityMetaError> {
        if self.name_cached() {
            Ok(self.name_raw().unwrap())
        } else {
            self.cache_name_with_ebase(
                self.read_ebase_from_storage()?,
                self.read_name_from_storage()?
            );
            Ok(self.name_raw().unwrap())
        }
    }

    pub fn etype(&self) -> Result<EntityType, LibEntityMetaError> {
        if self.etype_cached() {
            Ok(self.etype_raw().unwrap())
        } else {
            self.cache_etype_with_ebase(
                self.ebase()?,
                self.read_etype_from_storage()?
            );
            Ok(self.etype_raw().unwrap())
        }
    }

    pub fn tags(&self) -> Result<Vec<Tag>, LibEntityMetaError> {
        if self.tags_cached() {
            Ok(self.tags_raw().unwrap())
        } else {
            self.cache_tags_with_ebase(
                self.ebase()?,
                self.read_tags_from_storage()?,
            );
            Ok(self.tags_raw().unwrap())
        }
    }

    pub fn progress(&self) -> Result<Option<Progress>, LibEntityMetaError> {
        if self.progress_cached() {
            Ok(self.progress_raw().unwrap())
        } else {
            self.cache_progress(self.read_progress_from_storage()?);
            Ok(self.progress_raw().unwrap())
        }
    }

    pub fn description(&self) -> Result<Option<String>, LibEntityMetaError> {
        if self.description_cached() {
            Ok(self.description_raw().unwrap())
        } else {
            self.cache_description(self.read_description_from_storage()?);
            Ok(self.description_raw().unwrap())
        }
    }

    pub fn freedata(&self) -> Result<Option<FreeData>, LibEntityMetaError> {
        if self.freedata_cached() {
            Ok(self.freedata_raw().unwrap())
        } else {
            self.cache_freedata(self.read_freedata_from_storage()?);
            Ok(self.freedata_raw().unwrap())
        }
    }

    pub fn set_ebase(&mut self, ebase: EntityBase) -> Result<(), LibEntityMetaError> {
        self.data_cached.borrow_mut().ebase.replace(ebase);
        Ok(())
    }

    pub fn set_name(&mut self, name: String) -> Result<(), LibEntityMetaError> {
        let mut ebase = self.ebase()?;
        *ebase.name_mut() = name;
        self.set_ebase(ebase)?;

        Ok(())
    }

    pub fn set_etype(&mut self, etype: EntityType) -> Result<(), LibEntityMetaError> {
        let mut ebase = self.ebase()?;
        ebase.set_etype(etype);
        self.set_ebase(ebase)?;

        Ok(())
    }

    pub fn set_tags(&mut self, tags: Vec<Tag>) -> Result<(), LibEntityMetaError> {
        let mut ebase = self.ebase()?;
        *ebase.tags_mut() = tags;
        self.set_ebase(ebase)?;

        Ok(())
    }

    pub fn set_progress(&mut self, progress: Option<Progress>)
    -> Result<(), LibEntityMetaError> {
        self.data_cached.borrow_mut().progress.replace(progress);
        Ok(())
    }

    pub fn set_description(&mut self, description: Option<String>)
    -> Result<(), LibEntityMetaError> {
        self.data_cached.borrow_mut().description.replace(description);
        Ok(())
    }

    pub fn set_freedata(&mut self, freedata: Option<FreeData>)
    -> Result<(), LibEntityMetaError> {
        self.data_cached.borrow_mut().freedata.replace(freedata);
        Ok(())
    }

    pub fn dump_to_storage(&self) -> Result<(), LibEntityMetaError> {
        self.dump_ebase_to_storage()?;
        self.dump_progress_to_storage()?;
        self.dump_description_to_storage()?;
        self.dump_freedata_to_storage()?;

        Ok(())
    }

    fn dump_ebase_to_storage(&self) -> Result<(), LibEntityMetaError> {
        let ebase = self.ebase()?;
        self.storage.borrow_mut().update_entitybase(self.id, ebase)?;

        Ok(())
    }

    fn dump_progress_to_storage(&self) -> Result<(), LibEntityMetaError> {
        if let Some(progress) = self.progress()? {
            if self.storage.borrow().get_progress(self.id)?.is_some() {
                self.storage.borrow_mut().update_progress(self.id, progress)?;
            } else {
                self.storage.borrow_mut().link_progress_to_id(self.id, progress)?;
            }
        } else if self.storage.borrow().get_progress(self.id)?.is_some() {
            self.storage.borrow_mut().unlink_progress_from_id(self.id)?;
        }

        Ok(())
    }

    fn dump_description_to_storage(&self) -> Result<(), LibEntityMetaError> {
        if let Some(description) = self.description()? {
            if self.storage.borrow().get_description(self.id)?.is_some() {
                self.storage.borrow_mut().update_description(self.id, description)?;
            } else {
                self.storage.borrow_mut().link_description_to_id(self.id, description)?;
            }
        } else if self.storage.borrow().get_description(self.id)?.is_some() {
            self.storage.borrow_mut().unlink_description_from_id(self.id)?;
        }

        Ok(())
    }

    fn dump_freedata_to_storage(&self) -> Result<(), LibEntityMetaError> {
        if let Some(freedata) = self.freedata()? {
            if self.storage.borrow().get_freedata(self.id)?.is_some() {
                self.storage.borrow_mut().update_freedata(self.id, freedata)?;
            } else {
                self.storage.borrow_mut().link_freedata_to_id(self.id, freedata)?;
            }
        } else if self.storage.borrow().get_freedata(self.id)?.is_some() {
            self.storage.borrow_mut().unlink_freedata_to_id(self.id)?;
        }

        Ok(())
    }


    fn ebase_raw(&self) -> Option<EntityBase> {
        self.data_cached.borrow().ebase.clone()
    }

    fn name_raw(&self) -> Option<String> {
        self.data_cached.borrow().ebase.clone().map(|ebase| ebase.extract_name())
    }

    fn etype_raw(&self) -> Option<EntityType> {
        self.data_cached.borrow().ebase.clone().map(|ebase| ebase.etype())
    }

    fn tags_raw(&self) -> Option<Vec<Tag>> {
        self.data_cached.borrow().ebase.clone().map(|ebase| ebase.extract_tags())
    }

    fn progress_raw(&self) -> Option<Option<Progress>> {
        self.data_cached.borrow().progress.clone()
    }

    fn description_raw(&self) -> Option<Option<String>> {
        self.data_cached.borrow().description.clone()
    }

    fn freedata_raw(&self) -> Option<Option<FreeData>> {
        self.data_cached.borrow().freedata.clone()
    }

    fn ebase_cached(&self) -> bool { self.data_cached.borrow().ebase.is_some() }

    fn name_cached(&self) -> bool { self.ebase_cached() }

    fn etype_cached(&self) -> bool { self.ebase_cached() }

    fn tags_cached(&self) -> bool { self.ebase_cached() }

    fn progress_cached(&self) -> bool {
        self.data_cached.borrow().progress.is_some()
    }

    fn description_cached(&self) -> bool {
        self.data_cached.borrow().description.is_some()
    }

    fn freedata_cached(&self) -> bool {
        self.data_cached.borrow().freedata.is_some()
    }

    fn cache_ebase(&self, ebase: EntityBase) -> Option<EntityBase> {
        self.data_cached.borrow_mut().ebase.replace(ebase)
    }

    fn cache_name_with_ebase(&self,
        mut ebase: EntityBase,
        name: String)
    -> Option<String> {
        *ebase.name_mut() = name;
        self.cache_ebase(ebase).map(|old_ebase| old_ebase.extract_name())
    }

    fn cache_etype_with_ebase(&self,
        mut ebase: EntityBase,
        etype: EntityType)
    -> Option<EntityType> {
        ebase.set_etype(etype);
        self.cache_ebase(ebase).map(|old_ebase| old_ebase.etype())
    }

    fn cache_tags_with_ebase(&self,
        mut ebase: EntityBase,
        tags: Vec<Tag>
    ) -> Option<Vec<Tag>> {
        *ebase.tags_mut() = tags;
        self.cache_ebase(ebase).map(|old_ebase| old_ebase.extract_tags())
    }

    fn cache_progress(&self, progress: Option<Progress>) -> Option<Option<Progress>> {
        self.data_cached.borrow_mut().progress.replace(progress)
    }

    fn cache_description(&self, description: Option<String>) -> Option<Option<String>> {
        self.data_cached.borrow_mut().description.replace(description)
    }

    fn cache_freedata(&self, freedata: Option<FreeData>) -> Option<Option<FreeData>> {
        self.data_cached.borrow_mut().freedata.replace(freedata)
    }

    fn read_ebase_from_storage(&self) -> Result<EntityBase, LibEntityMetaError> {
        Ok(
            self.storage.borrow()
                .get_entitybase(self.id)?
                .ok_or_else(|| LibEntityMetaError::CouldNotFindEntityBaseForID {
                    id: self.id
                })?
        )
    }

    fn read_name_from_storage(&self) -> Result<String, LibEntityMetaError> {
        Ok(self.read_ebase_from_storage()?.destruct().1) // get `name` field
    }

    fn read_etype_from_storage(&self) -> Result<EntityType, LibEntityMetaError> {
        Ok(self.read_ebase_from_storage()?.destruct().2) // get `etype` field
    }
    
    fn read_tags_from_storage(&self) -> Result<Vec<Tag>, LibEntityMetaError> {
        Ok(self.read_ebase_from_storage()?.destruct().3) // get `tags` field
    }

    fn read_progress_from_storage(&self) -> Result<Option<Progress>, LibEntityMetaError> {
        Ok(self.storage.borrow().get_progress(self.id)?)
    }

    fn read_description_from_storage(&self) -> Result<Option<String>, LibEntityMetaError> {
        Ok(self.storage.borrow().get_description(self.id)?)
    }

    fn read_freedata_from_storage(&self) -> Result<Option<FreeData>, LibEntityMetaError> {
        Ok(self.storage.borrow().get_freedata(self.id)?)
    }
}
