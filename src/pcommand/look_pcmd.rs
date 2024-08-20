use crate::app::App;
use crate::comps_interaction::get_libentity;
use crate::error_ext::ComError;
use crate::scripts::Context;

use super::{PCommand, PExecutionError};

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct LookPCMD {
    path: PathBuf,
}

impl LookPCMD {
    pub fn new(path: PathBuf) -> Self {
        LookPCMD { path }
    }
}

impl PCommand for LookPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let libentity = match get_libentity(app.storage(), self.path.clone())? {
            Some(libentity) => libentity,
            None => {
                return Err(ComError::from(format!(
                    "couldn't get library entity (LibEntity object)"
                ))
                .into())
            }
        };
        let context = match Context::auto() {
            Some(context) => context,
            None => {
                return Err(
                    ComError::from(format!("couldn't make context (Context object)")).into(),
                )
            }
        };

        let result = app.scripts().look_output(libentity, context)?;
        println!("{}", result.trim_end());

        Ok(())
    }
}
