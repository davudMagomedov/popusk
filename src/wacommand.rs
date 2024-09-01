use crate::app::AppError;
use crate::core_commands::CoreError;

use std::io::Error as IoError;

use bincode::Error as BincodeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WAExexutionError {
    #[error("app error: {0}")]
    AppError(#[from] AppError),
    #[error("execution error: {0}")]
    CoreError(#[from] CoreError),
    #[error("io error: {0}")]
    IO(#[from] IoError),
    #[error("serialization/deserialization error: {0}")]
    SerDeser(#[from] BincodeError),
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
        crate::core_commands::corecmd_init_current_directory()?;
        Ok(())
    }
}

pub struct UpdateWAPCMD;

impl UpdateWAPCMD {
    pub fn new() -> Self {
        UpdateWAPCMD
    }
}

impl WACommand for UpdateWAPCMD {
    fn execute(&self) -> Result<(), WAExexutionError> {
        crate::core_commands::corecmd_update_current_directory()?;
        Ok(())
    }
}
