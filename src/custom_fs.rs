use std::path::{Path, PathBuf};
use std::io::Error as IoError;
use std::fs::{OpenOptions, File};

use thiserror::Error;

type FSResult<T> = Result<T, FSError>;

#[derive(Debug, Error)]
#[error("{action} {file_or_dir}: {ioerr}")]
pub struct FSError {
    pub action: &'static str,
    pub file_or_dir: PathBuf,
    pub ioerr: IoError,
}

pub fn create_file(path: &Path) -> FSResult<File> {
    let opened = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path);
    opened.map_err(|ioerr| FSError {
        action: "opening",
        file_or_dir: path.to_path_buf(),
        ioerr,
    })
}

pub fn create_new_file(path: &Path) -> FSResult<File> {
    let opened = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(path);
    opened.map_err(|ioerr| FSError {
        action: "creating",
        file_or_dir: path.to_path_buf(),
        ioerr,
    })
}

pub fn open_file_with_truncation(path: &Path) -> FSResult<File> {
    let opened = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open(path);
    opened.map_err(|ioerr| FSError {
        action: "opening with truncation",
        file_or_dir: path.to_path_buf(),
        ioerr,
    })
}

pub fn open_file_with_append(path: &Path) -> FSResult<File> {
    let opened = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(path);
    opened.map_err(|ioerr| FSError {
        action: "opening",
        file_or_dir: path.to_path_buf(),
        ioerr,
    })
}

pub fn open_readfile(path: &Path) -> FSResult<File> {
    let opened = OpenOptions::new().read(true).append(true).open(path);
    opened.map_err(|ioerr| FSError {
        action: "read-only opening",
        file_or_dir: path.to_path_buf(),
        ioerr,
    })
}

pub fn create_new_dir(path: &Path) -> FSResult<()> {
    std::fs::create_dir(path).map_err(|ioerr| FSError {
        action: "creating",
        file_or_dir: path.to_path_buf(),
        ioerr,
    })
}

pub fn remove_file(path: &Path) -> FSResult<()> {
    std::fs::remove_file(path).map_err(|ioerr| FSError {
        action: "removing",
        file_or_dir: path.to_path_buf(),
        ioerr,
    })
}

pub fn remove_dir(path: &Path) -> FSResult<()> {
    std::fs::remove_dir(path).map_err(|ioerr| FSError {
        action: "removing",
        file_or_dir: path.to_path_buf(),
        ioerr,
    })
}
