// use popusk::entity_data::{EntityData, EDError};
use popusk::app::{App, AppError};

use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WAExexutionError {
    #[error("{0}")]
    AppError(#[from] AppError),
}

/// `WACommand` (`W`ithout `A`pplication) is command that doesn't need `App` for being executed.
pub trait WACommand {
    fn execute(&self) -> Result<(), WAExexutionError>;
}

pub struct InitWACMD;

impl InitWACMD {
    pub fn new() -> Self {
        InitWACMD
    }
}

impl WACommand for InitWACMD {
    fn execute(&self) -> Result<(), WAExexutionError> {
        App::create_new(&PathBuf::from("."))?;
        Ok(())
    }
}
