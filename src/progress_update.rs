use crate::progress::Progress;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProgressUpdateError {
    #[error("invalid passed value {invalid_passed_val} whereas ceiling is {ceiling}")]
    InvalidPassedValue {
        invalid_passed_val: usize,
        ceiling: usize,
    },
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ProgressUpdate {
    Increase(usize),
    Decrease(usize),
    Set(usize),
    Leave,
}

impl ProgressUpdate {
    pub fn increase(val: usize) -> Self {
        ProgressUpdate::Increase(val)
    }

    pub fn decrease(val: usize) -> Self {
        ProgressUpdate::Decrease(val)
    }

    pub fn set(val: usize) -> Self {
        ProgressUpdate::Set(val)
    }

    pub fn leave() -> Self {
        ProgressUpdate::Leave
    }

    pub fn execute_for(&self, progress: &mut Progress) -> Result<(), ProgressUpdateError> {
        match self {
            ProgressUpdate::Increase(increase) => Ok(progress.forward_by(*increase)),
            ProgressUpdate::Decrease(decrease) => Ok(progress.backward_by(*decrease)),
            ProgressUpdate::Set(set) => {
                if *set > progress.ceiling() {
                    Err(ProgressUpdateError::InvalidPassedValue {
                        invalid_passed_val: *set,
                        ceiling: progress.ceiling(),
                    })
                } else {
                    Ok(progress.set_passed(*set))
                }
            }
            ProgressUpdate::Leave => Ok(()),
        }
    }
}
