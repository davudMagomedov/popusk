use crate::comps_interaction::libentity_has_progress;
use crate::entity_base::{EntityBase, EntityType, Tag};
use crate::id::ID;
use crate::libentity::{LibEntity, LibEntityData};
use crate::progress::Progress;
use crate::storage::{Storage, StorageError};

use std::path::PathBuf;

use thiserror::Error;

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
}

/// Implements operations with library entities (`LibEntity`) through storage (`Storage`).
pub struct Library {
    storage: Storage,
}

impl Library {
    pub fn new(storage: Storage) -> Self {
        Library { storage }
    }

    pub fn get_libentity(&self, path: PathBuf) -> Result<Option<LibEntity>, LibraryError> {
        let id = match self.storage.get_id(path.clone())? {
            Some(id) => id,
            None => return Ok(None),
        };
        let base = match self.storage.get_entitybase(id)? {
            Some(base) => base,
            None => {
                return Err(LibraryError::CouldNotFindElement {
                    element: "entitybase".to_string(),
                    path,
                })
            }
        };
        let progress = if libentity_has_progress(base.etype()) {
            match self.storage.get_progress(id)? {
                Some(progress) => Some(progress),
                None => {
                    return Err(LibraryError::CouldNotFindElement {
                        element: "progress".to_string(),
                        path,
                    });
                }
            }
        } else {
            None
        };
        let description = self.storage.get_description(id)?;

        let libentity_data = LibEntityData {
            path,
            name: base.name().clone(),
            etype: base.etype(),
            tags: base.tags().clone(),
            progress,
            description,
        };
        let libentity = LibEntity::from_id_data(id, libentity_data);

        Ok(Some(libentity))
    }

    pub fn add_libentity(&mut self, libentity_data: LibEntityData) -> Result<(), LibraryError> {
        let LibEntityData {
            path,
            name,
            etype,
            tags,
            progress,
            description,
        } = libentity_data;

        let id = self.storage.link_id_to_path(path)?;
        self.storage
            .link_entitybase_to_id(id, EntityBase::new(id, name, etype, tags))?;

        if let Some(progress) = progress {
            self.storage.link_progress_to_id(id, progress)?;
        }

        if let Some(description) = description {
            self.storage.link_description_to_id(id, description)?;
        }

        Ok(())
    }

    pub fn del_libentity(&mut self, path: PathBuf) -> Result<LibEntity, LibraryError> {
        let id = self.storage.unlink_id_from_path(path.clone())?;
        let base = self.storage.unlink_entitybase_from_id(id)?;
        let progress = if libentity_has_progress(base.etype()) {
            Some(self.storage.unlink_progress_from_id(id)?)
        } else {
            None
        };
        let description = match self.storage.get_description(id)? {
            Some(_) => Some(self.storage.unlink_description_from_id(id)?),
            None => None,
        };

        let libentity_data = LibEntityData {
            path,
            progress,
            description,
            name: base.name().clone(),
            etype: base.etype(),
            tags: base.tags().clone(),
        };
        let libentity = LibEntity::from_id_data(id, libentity_data);

        Ok(libentity)
    }

    pub fn update_libentity(
        &mut self,
        new_libentity_data: LibEntityData,
    ) -> Result<LibEntity, LibraryError> {
        let old_libentity = self.del_libentity(new_libentity_data.path.clone())?;
        self.add_libentity(new_libentity_data)?;

        Ok(old_libentity)
    }

    pub fn get_id(&self, path: PathBuf) -> Result<Option<ID>, LibraryError> {
        Ok(self.storage.get_id(path)?)
    }

    pub fn get_progress(&self, id: ID) -> Result<Option<Progress>, LibraryError> {
        Ok(self.storage.get_progress(id)?)
    }

    pub fn get_description(&self, id: ID) -> Result<Option<String>, LibraryError> {
        Ok(self.storage.get_description(id)?)
    }

    pub fn get_name(&self, id: ID) -> Result<Option<String>, LibraryError> {
        Ok(self
            .storage
            .get_entitybase(id)?
            .map(|base| base.name().clone()))
    }

    pub fn get_etype(&self, id: ID) -> Result<Option<EntityType>, LibraryError> {
        Ok(self.storage.get_entitybase(id)?.map(|base| base.etype()))
    }

    pub fn get_tags(&self, id: ID) -> Result<Option<Vec<Tag>>, LibraryError> {
        Ok(self
            .storage
            .get_entitybase(id)?
            .map(|base| base.tags().clone()))
    }

    pub fn get_base(&self, id: ID) -> Result<Option<EntityBase>, LibraryError> {
        Ok(self.storage.get_entitybase(id)?)
    }

    pub unsafe fn storage(&self) -> &Storage {
        &self.storage
    }

    pub unsafe fn storage_mut(&mut self) -> &mut Storage {
        &mut self.storage
    }
}
