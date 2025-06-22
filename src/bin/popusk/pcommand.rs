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

mod add_libentity_pcmd;
mod add_path_pcmd;
mod add_tags_pcmd;
mod change_progress_pcmd;
mod del_description_pcmd;
mod del_libentity_pcmd;
mod del_progress_pcmd;
mod del_tags_pcmd;
mod get_description_pcmd;
mod get_entitybase_pcmd;
mod get_progress_pcmd;
mod get_tags_pcmd;
mod list_pcmd;
mod look_pcmd;
mod open_pcmd;
mod rename_libentity_pcmd;
mod set_description_pcmd;
mod set_entitybase_pcmd;
mod set_freedata_pcmd;
mod set_progress_pcmd;
mod status_pcmd;

pub use add_libentity_pcmd::*;
pub use add_path_pcmd::*;
pub use add_tags_pcmd::*;
pub use change_progress_pcmd::*;
pub use del_description_pcmd::*;
pub use del_libentity_pcmd::*;
pub use del_progress_pcmd::*;
pub use del_tags_pcmd::*;
pub use get_description_pcmd::*;
pub use get_entitybase_pcmd::*;
pub use get_progress_pcmd::*;
pub use get_tags_pcmd::*;
pub use list_pcmd::*;
pub use look_pcmd::*;
pub use open_pcmd::*;
pub use rename_libentity_pcmd::*;
pub use set_description_pcmd::*;
pub use set_entitybase_pcmd::*;
pub use set_freedata_pcmd::*;
pub use set_progress_pcmd::*;
pub use status_pcmd::*;

use popusk::app::App;
use popusk::custom_fs::FSError;
use popusk::entity_data::EDError;
use popusk::error_ext::ComError;
use popusk::scripts::ScriptsError;
use popusk::types::ProgressUpdateError;

use std::io::Error as IoError;
use std::path::PathBuf;

use thiserror::Error;
use walkdir::Error as WDError;

type PEResult<T> = Result<T, PExecError>;

#[derive(Debug, Error)]
pub enum PExecError {
    #[error("{0}")]
    FSError(#[from] FSError),
    #[error("{0}")]
    Scripts(#[from] ScriptsError),
    #[error("{0}")]
    EDError(#[from] EDError),
    #[error("'{path}' does not exist")]
    PathDoesNotExist { path: PathBuf },
    #[error("'{path}' already exists")]
    PathAlreadyExists { path: PathBuf },
    #[error("{component} for library entity '{entitypath}' was not found")]
    ComponentWasNotFound {
        component: &'static str,
        entitypath: PathBuf,
    },
    #[error("reading stdin: {ioerr}")]
    StdinStream { ioerr: IoError },
    #[error("reading '{path}': {ioerr}")]
    FileStream { path: PathBuf, ioerr: IoError },
    #[error("library entity '{entitypath}' was not found")]
    LibEntityWasNotFound { entitypath: PathBuf },
    #[error("library entity '{entitypath}' already exists")]
    LibEntityAlreadyExists { entitypath: PathBuf },
    #[error("could not deserialize {format} value as {component}: {error}")]
    DeserError {
        format: &'static str,
        component: &'static str,
        error: ComError,
    },
    #[error("could not serialize {component} to {format} value: {error}")]
    SeriaError {
        format: &'static str,
        component: &'static str,
        error: ComError,
    },
    #[error("{0}")]
    ProgressUpdateError(#[from] ProgressUpdateError),
    #[error("could not create 'context' object")]
    CouldNotCreateContext,
    #[error("{0}")]
    WDError(#[from] WDError),
}

/// `PCommand` (*P*opusk *C*ommand).
///
/// Implements abstract command of CLI. Iow `PCommand` gives interface the same as CLI command
/// implemention.
pub trait PCommand {
    fn execute(&self, app: &mut App) -> Result<(), PExecError>;
}
