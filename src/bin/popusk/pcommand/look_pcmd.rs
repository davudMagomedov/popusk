use popusk::app::App;
use popusk::error_ext::{ComError, CommonizeResultExt};
use popusk::scripts::{Context, ScriptsError};
use popusk::library::LibraryError;
use popusk::types::{LibEntityMetaError, LibEntity, StyledText};

use super::{PCommand, PExecutionError};

use std::path::PathBuf;

use thiserror::Error as ThisError;

type CMDResult<T, E = CMDError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum CMDError {
    #[error("could not find library entity with path '{path}'")]
    CouldNotFindLibEntity { path: PathBuf },

    #[error("library entity: {0}")]
    LibEntityMeta(#[from] LibEntityMetaError),
    #[error("library: {0}")]
    Library(#[from] LibraryError),
    #[error("scripts: {0}")]
    Scripts(#[from] ScriptsError),
    #[error("{0}")]
    Other(#[from] ComError),
}

#[derive(Debug, Clone)]
pub struct LookPCMD {
    path: PathBuf,
}

impl LookPCMD {
    pub fn new(path: PathBuf) -> Self {
        LookPCMD { path }
    }

    fn get_libentity(&self, app: &App) -> CMDResult<LibEntity> {
        match app.library().get_libentity(self.path.clone())? {
            Some(libentity) => Ok(libentity.into_canonical_libentity()?),
            None => Err(CMDError::CouldNotFindLibEntity { path: self.path.clone() })
        }
    }

    fn make_context(&self, _app: &App) -> CMDResult<Context> {
        match Context::auto() {
            Some(context) => Ok(context),
            None => {
                Err(
                    ComError::from("couldn't make context (Context object)").into(),
                )
            }
        }
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<StyledText> {
        let libentity = self.get_libentity(app)?;
        let context = self.make_context(app)?;
        let result = app.scripts().look_output(libentity, context)?;

        Ok(result)
    }
}

impl PCommand for LookPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let result = self.execute_inner(app).commonize()?;
        println!("{}", result);

        Ok(())
    }
}
