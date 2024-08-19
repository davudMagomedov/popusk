use crate::comps_appearance::{progress_from_string, progress_update_from_string};
use crate::id::ID;
use crate::progress::Progress;
use crate::progress_update::ProgressUpdate;

use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct CLI {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Debug, Subcommand)]
// 'llc' prefix is 'Low-Level Command'.
pub enum CliCommand {
    #[command(name = "init")]
    Init,
    #[command(name = "llc_add_path")]
    AddPath { path: PathBuf },
    #[command(name = "llc_add_progress")]
    AddProgress {
        id: ID,
        #[arg(value_parser = progress_from_string)]
        progress: Progress,
    },
    #[command(name = "llc_add_entitybase")]
    /// Input form stdin.
    AddEntitybase { id: ID },
    #[command(name = "llc_del_path")]
    DelPath { path: PathBuf },
    #[command(name = "llc_del_progress")]
    DelProgress { id: ID },
    #[command(name = "llc_del_entitybase")]
    DelEntitybase { id: ID },
    #[command(name = "get_id")]
    GetId { path: PathBuf },
    #[command(name = "get_progress")]
    GetProgress { id: ID },
    #[command(name = "get_entitybase")]
    GetEntitybase { id: ID },
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
    #[command(name = "del_libentity")]
    DelLibentity { path: PathBuf },
    #[command(name = "look")]
    Look { path: PathBuf },
    #[command(name = "open")]
    Open {
        path: PathBuf,
        #[arg(long = "just_look", short = 'j', action = ArgAction::SetTrue)]
        just_look: bool,
    },
    #[command(name = "list")]
    List {
        #[arg(long, short = 'w', action = ArgAction::SetTrue)]
        wide: bool,
    },
    #[command(name = "status")]
    Status {
        #[arg(long = "hidden", action = ArgAction::SetTrue)]
        show_hidden: bool,
        #[arg(long = "ignore")]
        ignore: Option<String>,
    },
    #[command(name = "change_progress")]
    ChangeProgress {
        id: ID,
        #[arg(value_parser = progress_update_from_string)]
        progress_update: ProgressUpdate,
    },
    #[command(name = "add_tags")]
    AddTags { id: ID, tags: String },
}
