use super::{PCommand, PEResult, PExecError};

use popusk::app::{App, WORKING_DIR};

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

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

/// Just returns `path == WORKING_DIR`.
fn is_working_directory(name: &OsStr) -> bool {
    name.to_str()
        .map(|path| path.contains(WORKING_DIR))
        .unwrap_or(false)
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

    fn include_entry(&self, entry: &DirEntry) -> bool {
        entry.file_name() == "."
            || !(self.hide_hidden() && is_hidden(entry.file_name())
                || self.ignore_list_contains(entry.file_name()))
    }

    fn is_tracked(&self, app: &App, entry: &DirEntry) -> bool {
        app.library().libentity_exists(entry.path())
    }

    fn print_untracked_paths(&self, untracked_paths: Vec<PathBuf>) {
        println!("Untracked files:");
        untracked_paths
            .into_iter()
            .for_each(|untracked_path| println!("    {}", untracked_path.to_string_lossy()));
    }

    fn directory_iterator<'a>(&'a self) -> impl Iterator<Item = PEResult<DirEntry>> + use<'a> {
        WalkDir::new(".")
            .into_iter()
            .filter_entry(|entry| self.include_entry(entry))
            .filter(|maybe_entry| match maybe_entry {
                Ok(ref entry) => {
                    !is_current_directory(entry.path())
                        && !(self.hide_directories() && entry.file_type().is_dir())
                }
                Err(_) => true,
            })
            .map(|direntry| direntry.map_err(Into::into))
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<Vec<PathBuf>> {
        let directory_files = self.directory_iterator();

        let mut untracked_files = Vec::new();
        for entry in directory_files {
            let entry = entry?;

            if !self.is_tracked(app, &entry) {
                untracked_files.push(entry.path().to_owned());
            }
        }

        Ok(untracked_files)
    }
}

impl PCommand for StatusPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecError> {
        let untracked_files = self.execute_inner(app)?;
        self.print_untracked_paths(untracked_files);

        Ok(())
    }
}
