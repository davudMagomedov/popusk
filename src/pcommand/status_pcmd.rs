use crate::app::App;
use crate::storage::DEFAULT_WORKING_DIR;

use super::{PCommand, PExecutionError};

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use walkdir::{DirEntry, WalkDir};

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

/// Just returns `path == ".popusk"`.
fn is_working_directory(name: &OsStr) -> bool {
    name.to_str()
        .map(|path| path.contains(DEFAULT_WORKING_DIR))
        .unwrap_or(false)
}

#[derive(Debug, Clone)]
pub struct StatusPCMD {
    show_hidden: bool,
    ignore: Vec<String>,
}

impl StatusPCMD {
    pub fn new(show_hidden: bool, ignore_str: Option<String>) -> Self {
        let ignore = ignore_str
            .unwrap_or_else(|| String::new())
            .split(',')
            .map(|ignore_path| ignore_path.to_string())
            .collect();

        StatusPCMD {
            show_hidden,
            ignore,
        }
    }

    fn hide_hidden(&self) -> bool {
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

        // hide_hidden, is_hidden, ignore, is_working_directory
        //
        // - hide_hidden && is_hidden => false
        // - hide_hidden && !is_hidden && ignore => false
        // - hide_hidden && !is_hidden && !ignore => true
        // - !hide_hidden && is_working_directory => false
        // - !hide_hidden && !is_working_directory && ignore => false
        // - !hide_hidden && !is_working_directory && !ignore => true
        if self.hide_hidden() {
            if is_hidden(name) {
                false
            } else {
                if self.ignore_list_contains(name) {
                    false
                } else {
                    true
                }
            }
        } else if is_working_directory(name) {
            false
        } else {
            if self.ignore_list_contains(name) {
                false
            } else {
                true
            }
        }
    }

    fn check_on_tracked(&self, app: &App, entry: &DirEntry) -> Result<bool, PExecutionError> {
        Ok(app.storage().get_id(entry.path().to_owned())?.is_some())
    }

    fn print_untracked_paths(&self, untracked_paths: Vec<PathBuf>) {
        println!("Untracked files:");
        untracked_paths
            .into_iter()
            .for_each(|untracked_path| println!("    {}", untracked_path.to_string_lossy()));
    }
}

impl PCommand for StatusPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let directory_rec_iterator = WalkDir::new(".")
            .into_iter()
            .filter_entry(|entry| self.check_entry(entry))
            .filter(|maybe_entry| match maybe_entry {
                Ok(ref entry) => !is_current_directory(entry.path()),
                Err(_) => true,
            });

        let mut untracked_paths = Vec::new();
        for entry in directory_rec_iterator {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => return Err(anyhow!("couldn't get directory entry: {0}", err).into()),
            };

            if !self.check_on_tracked(app, &entry)? {
                untracked_paths.push(entry.path().to_owned());
            }
        }

        self.print_untracked_paths(untracked_paths);

        Ok(())
    }
}
