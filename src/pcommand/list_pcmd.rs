use crate::app::App;
use crate::comps_interaction::get_libentity;
use crate::error_ext::ComError;
use crate::scripts::Context;

use super::{PCommand, PExecutionError};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ListMode {
    Wide,
    Narrow,
}

impl ListMode {
    pub fn wide(is_wide: bool) -> Self {
        if is_wide {
            ListMode::Wide
        } else {
            ListMode::Narrow
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListPCMD {
    mode: ListMode,
}

impl ListPCMD {
    pub fn new(mode: ListMode) -> Self {
        ListPCMD { mode }
    }
}

impl PCommand for ListPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let paths = app.storage().keys_path()?;

        let mut libentities = Vec::with_capacity(paths.len());
        for path in paths {
            let libentity = match get_libentity(app.storage(), path)? {
                Some(libentity) => libentity,
                None => return Err(ComError::from(format!("invalid library entity")).into()),
            };
            libentities.push(libentity);
        }

        let context = match Context::auto() {
            Some(context) => context,
            None => {
                return Err(
                    ComError::from(format!("couldn't make context (Context object)")).into(),
                )
            }
        };

        let result = match self.mode {
            ListMode::Wide => app.scripts().list_output_wide(libentities, context)?,
            ListMode::Narrow => app.scripts().list_output_narrow(libentities, context)?,
        };
        println!("{}", result.trim());

        Ok(())
    }
}
