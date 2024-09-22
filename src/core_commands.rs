use crate::app::AppError;
use crate::storage::{Storage, StorageError};
use crate::types::{EntityBase, Progress, ID};

use std::path::PathBuf;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum CoreError {
    #[error("storage: {0}")]
    StorageError(#[from] StorageError),
    #[error("application error: {0}")]
    AppError(#[from] AppError),
}

pub fn corecmd_add_path(storage: &mut Storage, path: PathBuf) -> Result<ID, CoreError> {
    Ok(storage.link_id_to_path(path)?)
}

pub fn corecmd_add_progress(
    storage: &mut Storage,
    id: ID,
    progress: Progress,
) -> Result<(), CoreError> {
    Ok(storage.link_progress_to_id(id, progress)?)
}

pub fn corecmd_add_entitybase(
    storage: &mut Storage,
    id: ID,
    entitybase: EntityBase,
) -> Result<(), CoreError> {
    Ok(storage.link_entitybase_to_id(id, entitybase)?)
}

pub fn corecmd_add_description(
    storage: &mut Storage,
    id: ID,
    description: String,
) -> Result<(), CoreError> {
    Ok(storage.link_description_to_id(id, description)?)
}

pub fn corecmd_del_path(storage: &mut Storage, path: PathBuf) -> Result<ID, CoreError> {
    Ok(storage.unlink_id_from_path(path)?)
}

pub fn corecmd_del_progress(storage: &mut Storage, id: ID) -> Result<Progress, CoreError> {
    Ok(storage.unlink_progress_from_id(id)?)
}

pub fn corecmd_del_entitybase(storage: &mut Storage, id: ID) -> Result<EntityBase, CoreError> {
    Ok(storage.unlink_entitybase_from_id(id)?)
}

pub fn corecmd_del_description(storage: &mut Storage, id: ID) -> Result<String, CoreError> {
    Ok(storage.unlink_description_from_id(id)?)
}

pub fn corecmd_init_current_directory() -> Result<(), CoreError> {
    Storage::create()?;
    Ok(())
}

pub fn corecmd_update_current_directory() -> Result<(), CoreError> {
    Storage::update()?;
    Ok(())
}
