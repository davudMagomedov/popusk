use popusk::comps_appearance::{progress_from_string, progress_update_from_string};
use popusk::{Progress, ProgressUpdate};

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
    /// Add path to the storage of the current directory
    #[command(name = "llc_add_path")]
    AddPath { path: PathBuf },
    /// Add progress to the storage of the current directory
    #[command(name = "set_progress")]
    SetProgress {
        path: PathBuf,
        #[arg(value_parser = progress_from_string)]
        progress: Progress,
    },
    /// Add entity base for ID. Put serialized entitybase to stdin
    #[command(name = "set_entitybase")]
    SetEntitybase { path: PathBuf },
    /// Add freedata for ID. Takes JSON dictionary.
    #[command(name = "set_freedata")]
    SetFreeData { path: PathBuf, file: Option<PathBuf> },
    /// Add description for ID
    #[command(name = "set_description")]
    SetDescription { path: PathBuf, description: String },
    /// Delete the progress from the storage of current directory
    #[command(name = "del_progress")]
    DelProgress { path: PathBuf },
    /// Delete the description from the storage of current directory
    #[command(name = "del_description")]
    DelDescription { path: PathBuf },
    /// Return progress of the library entity associated wtih the given ID
    #[command(name = "get_progress")]
    GetProgress { path: PathBuf },
    /// Return base of the library entity associated wtih the given ID
    #[command(name = "get_entitybase")]
    GetEntitybase { path: PathBuf },
    #[command(name = "get_description")]
    GetDescription { path: PathBuf },
    #[command(name = "get_tags")]
    GetTags { path: PathBuf },
    /// Add library entity to the storage of current directory
    #[command(name = "add_libentity")]
    AddLibentity { path: PathBuf },
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
        #[arg(long, short = 'j', action = ArgAction::SetTrue)]
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
        /// Include hidden files
        #[arg(long, short = 'm', action = ArgAction::SetTrue)]
        show_hidden: bool,
        /// Include directories
        #[arg(long, short = 'd', action = ArgAction::SetTrue)]
        show_directories: bool,
        ignore: Option<String>,
    },
    /// Change progress associated with the given ID
    #[command(name = "change_progress")]
    ChangeProgress {
        path: PathBuf,
        #[arg(value_parser = progress_update_from_string)]
        progress_update: ProgressUpdate,
    },
    /// Extend current set of tags by new ones
    #[command(name = "add_tags")]
    AddTags { path: PathBuf, tags: String },
    /// Delete tags associated with the given ID
    #[command(name = "del_tags")]
    DelTags { path: PathBuf },
    /// Move library entity to new place
    #[command(name = "rename")]
    RenameLibentity { path: PathBuf, new_path: PathBuf },
}
