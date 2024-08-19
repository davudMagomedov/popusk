//! Here is defined how components (progress, bases, etc) interacts without any context (such as
//! `Storage`, `Config`, etc).

use crate::entity_base::EntityType;
use crate::scripts::LibEntity;
use crate::storage::{Storage, StorageError};

use std::path::PathBuf;

pub fn libentity_has_progress(etype: EntityType) -> bool {
    etype == EntityType::Document
}

pub fn get_libentity(storage: &Storage, path: PathBuf) -> Result<Option<LibEntity>, StorageError> {
    let id = match storage.get_id(path.clone())? {
        Some(id) => id,
        None => return Ok(None),
    };
    let base = match storage.get_entitybase(id)? {
        Some(base) => base,
        None => return Ok(None),
    };
    let progress = match storage.get_progress(id)? {
        Some(progress) => Some(progress),
        None if libentity_has_progress(base.etype()) => return Ok(None),
        None => None,
    };

    Ok(Some(LibEntity::new(path, base, progress)))
}
