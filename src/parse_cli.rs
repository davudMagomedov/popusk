use crate::comps_appearance::{progress_from_string, progress_update_from_string};
use crate::types::Progress;
use crate::types::ProgressUpdate;
use crate::types::ID;

use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
pub struct CLI {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Debug, Subcommand)]
// 'llc' prefix is 'Low-Level Command'.
pub enum CliCommand {
    /// Initialize current directory
    #[command(name = "init")]
    Init,
    /// Update the current library for compatibility with the new version
    #[command(name = "update")]
    Update,
    /// Add path to the storage of the current directory
    #[command(name = "llc_add_path")]
    AddPath { path: PathBuf },
    /// Add progress to the storage of the current directory
    #[command(name = "llc_add_progress")]
    AddProgress {
        id: ID,
        #[arg(value_parser = progress_from_string)]
        progress: Progress,
    },
    /// Add entity base for ID. Put serialized entitybase to stdin
    #[command(name = "llc_add_entitybase")]
    AddEntitybase { id: ID },
    /// Add description for ID
    #[command(name = "llc_add_description")]
    AddDescription { id: ID, description: String },
    /// Delete the path from the storage of current directory
    #[command(name = "llc_del_path")]
    DelPath { path: PathBuf },
    /// Delete the progress from the storage of current directory
    #[command(name = "llc_del_progress")]
    DelProgress { id: ID },
    /// Delete the entity base from the storage of current directory
    #[command(name = "llc_del_entitybase")]
    DelEntitybase { id: ID },
    /// Delete the description from the stroage of current directory
    #[command(name = "llc_del_description")]
    DelDescription { id: ID },
    /// Return ID of the library entity associated with the given path
    #[command(name = "get_id")]
    GetId { path: PathBuf },
    /// Return progress of the library entity associated wtih the given ID
    #[command(name = "get_progress")]
    GetProgress { id: ID },
    /// Return base of the library entity associated wtih the given ID
    #[command(name = "get_entitybase")]
    GetEntitybase { id: ID },
    /// Add library entity to the storage of current directory
    #[command(name = "add_libentity")]
    AddLibentity {
        path: PathBuf,
        #[arg(long, short = 'n')]
        name: Option<String>,
        #[arg(long, short = 't')]
        tags: Option<String>,
        #[arg(long, short = 'c')]
        prog_ceil: Option<usize>,
    },
    /// Delete library entity associated with the given path
    #[command(name = "del_libentity")]
    DelLibentity { path: PathBuf },
    /// Returns the "cover" of the library entity associated with the given path
    #[command(name = "look")]
    Look { path: PathBuf },
    /// Open the library entity associated with the given path
    ///
    /// The opening method is dictated in the configuration
    #[command(name = "open")]
    Open {
        path: PathBuf,
        #[arg(long = "just_look", short = 'j', action = ArgAction::SetTrue)]
        just_look: bool,
    },
    /// Return the list of all library entities
    #[command(name = "list")]
    List {
        #[arg(long, short = 'w', action = ArgAction::SetTrue)]
        wide: bool,
    },
    /// Return status of current directory: untracked files for example
    #[command(name = "status")]
    Status {
        #[arg(long = "hidden", action = ArgAction::SetTrue)]
        show_hidden: bool,
        #[arg(long = "ignore")]
        ignore: Option<String>,
    },
    /// Change progress associated with the given ID
    #[command(name = "change_progress")]
    ChangeProgress {
        id: ID,
        #[arg(value_parser = progress_update_from_string)]
        progress_update: ProgressUpdate,
    },
    /// Extend current set of tags by new ones
    #[command(name = "add_tags")]
    AddTags { id: ID, tags: String },
    /// Delete tags associated with the given ID
    #[command(name = "del_tags")]
    DelTags { id: ID },
}
