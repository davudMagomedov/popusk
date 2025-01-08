use crate::app::App;
use crate::error_ext::CommonizeResultExt;
use crate::storage::DEFAULT_WORKING_DIR;
use crate::storage::StorageError;

use super::{PCommand, PExecutionError};

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::io::Error as IoError;

use walkdir::{DirEntry, WalkDir, Error as WDError};
use thiserror::Error as ThisError;

fn is_hidden(name: &OsStr) -> bool {
    name.to_str()
        .map(|s| s != "." && s != ".." && s.starts_with('.'))
        .unwrap_or(false)
}

/// Just returns `path == "."`
fn is_current_directory(path: &Path) -> bool {
    path.to_str()
        .map(|path_as_str| path_as_str == ".")
        .unwrap_or(false)
}

/// Just returns `path == DEFAULT_WORKING_DIR`.
fn is_working_directory(name: &OsStr) -> bool {
    name.to_str()
        .map(|path| path.contains(DEFAULT_WORKING_DIR))
        .unwrap_or(false)
}

type CMDResult<T, E = CMDError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum CMDError {
    #[error("could not get dir entry: {0}")]
    CouldNotGetDirEntry(WDError),

    #[error("storage: {0}")]
    Storage(#[from] StorageError),
    #[error("i/o: {0}")]
    IO(#[from] IoError),
}

#[derive(Debug, Clone)]
pub struct StatusPCMD {
    show_hidden: bool,
    show_directories: bool,
    ignore: Vec<String>,
}

impl StatusPCMD {
    pub fn new(show_hidden: bool, show_directories: bool, ignore_str: Option<String>) -> Self {
        let ignore = ignore_str
            .unwrap_or_else(|| String::new())
            .split(',')
            .map(|ignore_path| ignore_path.to_string())
            .collect();

        StatusPCMD {
            show_hidden,
            show_directories,
            ignore,
        }
    }

    fn hide_hidden(&self) -> bool {
        !self.show_hidden
    }

    fn hide_directories(&self) -> bool {
        !self.show_hidden
    }

    fn ignore_list_contains(&self, name: &OsStr) -> bool {
        self.ignore
            .iter()
            .position(|a| name == OsStr::new(a))
            .is_some()
    }

    fn check_entry(&self, entry: &DirEntry) -> bool {
        let name = entry.file_name();

        // hide_hidden, is_hidden, ignore, is_working_directory, hide_directories, is_directory
        //
        // - hide_hidden && is_hidden => false
        // - hide_hidden && !is_hidden && ignore => false
        // - hide_hidden && !is_hidden && !ignore && hide_directories && is_directory => false
        // - hide_hidden && !is_hidden && !ignore && hide_directories && !is_directory => true
        // - !hide_hidden && is_working_directory => false
        // - !hide_hidden && !is_working_directory && hide_directories && is_directory => false
        // - !hide_hidden && !is_working_directory && hide_directories && !is_directory => true
        if self.hide_hidden() && is_hidden(name) {
            true
        } else if !self.hide_hidden()
                && !is_working_directory(name)
                && self.hide_directories()
                && entry.file_type().is_dir() {
            true
        } else {
            false
        }
    }

    fn check_on_tracked(&self, app: &App, entry: &DirEntry) -> CMDResult<bool> {
        Ok(
            app.library().storage()
                .borrow()
                .get_id(entry.path().to_owned())?
                .is_some()
        )
    }

    fn print_untracked_paths(&self, untracked_paths: Vec<PathBuf>) {
        println!("Untracked files:");
        untracked_paths
            .into_iter()
            .for_each(|untracked_path| println!("    {}", untracked_path.to_string_lossy()));
    }

    fn directory_iterator<'a>(&'a self) -> impl Iterator<Item = CMDResult<DirEntry>> + use<'a> {
        WalkDir::new(".")
            .into_iter()
            .filter_entry(|entry| self.check_entry(entry))
            .filter(|maybe_entry| match maybe_entry {
                Ok(ref entry) => !is_current_directory(entry.path()),
                Err(_) => true,
            })
            .map(|direntry| direntry.map_err(|wderr| CMDError::CouldNotGetDirEntry(wderr)))
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<Vec<PathBuf>> {
        let directory_files = self.directory_iterator();

        let mut untracked_files = Vec::new();
        for entry in directory_files {
            let entry = entry?;

            if !self.check_on_tracked(app, &entry)? {
                untracked_files.push(entry.path().to_owned());
            }
        }

        Ok(untracked_files)
    }
}

impl PCommand for StatusPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let untracked_files = self.execute_inner(app).commonize()?;
        self.print_untracked_paths(untracked_files);

        Ok(())
    }
}
