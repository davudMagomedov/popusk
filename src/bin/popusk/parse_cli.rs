use popusk::comps_appearance::{
    entitytype_from_string, progress_from_string, progress_update_from_string,
};
use popusk::types::{EntityType, Progress, ProgressUpdate};

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
    /// Add the path to popusk storage (low-level command)
    #[command(name = "llc_add_path")]
    AddPath { path: PathBuf },
    /// Set the progress for the library entity
    #[command(name = "set_progress")]
    SetProgress {
        path: PathBuf,
        #[arg(value_parser = progress_from_string)]
        progress: Progress,
    },
    /// Set the base for the library entity. Takes JSON structure with fields 'name', 'etype' and
    /// 'tags'
    #[command(name = "set_entitybase")]
    SetEntitybase { path: PathBuf },
    /// Set the name for the library entity
    #[command(name = "set_name")]
    SetName { path: PathBuf, name: String },
    /// Set the type for the library entity
    #[command(name = "set_etype")]
    SetEtype {
        path: PathBuf,
        #[arg(value_parser = entitytype_from_string)]
        etype: EntityType,
    },
    /// Set the freedata for the library entity. Takes JSON structure from stdin or a file
    #[command(name = "set_freedata")]
    SetFreeData {
        path: PathBuf,
        file: Option<PathBuf>,
    },
    /// Set the description for the library entity
    #[command(name = "set_description")]
    SetDescription { path: PathBuf, description: String },
    /// Delete progress of the library entity
    #[command(name = "del_progress")]
    DelProgress { path: PathBuf },
    /// Delete description of the library entity
    #[command(name = "del_description")]
    DelDescription { path: PathBuf },
    /// Output a progress of the library entity
    #[command(name = "get_progress")]
    GetProgress { path: PathBuf },
    /// Output a base of the library entity
    #[command(name = "get_entitybase")]
    GetEntitybase { path: PathBuf },
    /// Output a description of the library entity
    #[command(name = "get_description")]
    GetDescription { path: PathBuf },
    /// Output tags of the library entity
    #[command(name = "get_tags")]
    GetTags { path: PathBuf },
    /// Add library entity. Path must exist in the directory and absent in popusk storage
    #[command(name = "add_libentity")]
    AddLibentity { path: PathBuf },
    /// Delete the library entity from popusk storage (don't affect a filesystem)
    #[command(name = "del_libentity")]
    DelLibentity { path: PathBuf },
    /// Output the "cover" of the library entity
    #[command(name = "look")]
    Look { path: PathBuf },
    /// Open the library entity
    ///
    /// The opening method is dictated in the configuration
    #[command(name = "open")]
    Open {
        path: PathBuf,
        #[arg(long, short = 'j', action = ArgAction::SetTrue)]
        just_look: bool,
    },
    /// Output the list of all library entities
    #[command(name = "list")]
    List {
        #[arg(long, short = 'w', action = ArgAction::SetTrue)]
        wide: bool,
    },
    /// Output information of the current directory: untracked files for example
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
    /// Change progress for the library entity
    #[command(name = "change_progress")]
    ChangeProgress {
        path: PathBuf,
        #[arg(value_parser = progress_update_from_string)]
        progress_update: ProgressUpdate,
    },
    /// Extend current set of tags by new ones
    #[command(name = "add_tags")]
    AddTags { path: PathBuf, tags: String },
    /// Delete selected tags of the library entity
    #[command(name = "del_tags")]
    DelTags { path: PathBuf },
    /// Move library entity to new place
    #[command(name = "move")]
    RenameLibentity { path: PathBuf, new_path: PathBuf },
}
