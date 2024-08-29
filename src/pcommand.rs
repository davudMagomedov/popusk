//! Here is all commands wrappers for CLI ones.
//!
//! ## Safety
//! There're low-level commands (add_path, add_entitybase, etc) that can broke invariants (they
//! will be listed below) and high-level commands (add_libentity, del_libentity, etc).
//!
//! There're following invariants.
//! 1. Each libentity has its own ID and EntityBase. A libentity without one the both is invalid.
//! 2. Libentity with type (EntityType) `Document` must have progress in any way.
//! 3. Libentity with type (EntityType) `Section` must not have progress.
//! 4. Libentity with type (EntityType) `Regular` must not have progress.

use crate::app::{App, AppError};
use crate::core_commands::CoreError;
use crate::error_ext::ComError;
use crate::library::LibraryError;
use crate::progress_update::ProgressUpdateError;
use crate::scripts::ScriptsError;
use crate::storage::StorageError;

use std::io::Error as IoError;

use bincode::Error as BincodeError;
use thiserror::Error;

mod add_entitybase_pcmd;
mod add_libentity_pcmd;
mod add_path_pcmd;
mod add_progress_pcmd;
mod add_tags_pcmd;
mod change_progress_pcmd;
mod del_entitybase_pcmd;
mod del_libentity_pcmd;
mod del_path_pcmd;
mod del_progress_pcmd;
mod del_tags_pcmd;
mod get_entitybase_pcmd;
mod get_id_pcmd;
mod get_progress_pcmd;
mod list_pcmd;
mod look_pcmd;
mod open_pcmd;
mod status_pcmd;

pub use add_entitybase_pcmd::*;
pub use add_libentity_pcmd::*;
pub use add_path_pcmd::*;
pub use add_progress_pcmd::*;
pub use add_tags_pcmd::*;
pub use change_progress_pcmd::*;
pub use del_entitybase_pcmd::*;
pub use del_libentity_pcmd::*;
pub use del_path_pcmd::*;
pub use del_progress_pcmd::*;
pub use del_tags_pcmd::*;
pub use get_entitybase_pcmd::*;
pub use get_id_pcmd::*;
pub use get_progress_pcmd::*;
pub use list_pcmd::*;
pub use look_pcmd::*;
pub use open_pcmd::*;
pub use status_pcmd::*;

#[derive(Debug, Error)]
pub enum PExecutionError {
    #[error("app error: {0}")]
    AppError(#[from] AppError),
    #[error("execution error: {0}")]
    CoreError(#[from] CoreError),
    #[error("io error: {0}")]
    IO(#[from] IoError),
    #[error("serialization/deserialization error: {0}")]
    SerDeser(#[from] BincodeError),
    #[error("storage error: {0}")]
    StorageError(#[from] StorageError),
    #[error("scripts error: {0}")]
    ScriptsError(#[from] ScriptsError),
    #[error("progress update error: {0}")]
    ProgressUpdateError(#[from] ProgressUpdateError),
    #[error("library error: {0}")]
    LibraryError(#[from] LibraryError),

    #[error("{0}")]
    Other(#[from] ComError),
}

/// `PCommand` (*P*opusk *C*ommand).
///
/// Implements abstract command of CLI. Iow `PCommand` gives interface the same as CLI command
/// implemention.
pub trait PCommand {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError>;
}
