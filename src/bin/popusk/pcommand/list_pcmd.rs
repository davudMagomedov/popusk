use popusk::app::App;
use popusk::error_ext::{ComError, CommonizeResultExt};
use popusk::scripts::{ScriptsError, Context};
use popusk::types::{LibEntity, LibEntityMetaError, StyledText};
use popusk::library::LibraryError;
use popusk::storage::StorageError;

use super::{PCommand, PExecutionError};

use thiserror::Error as ThisError;

type CMDResult<T, E = CMDError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum CMDError {
    #[error("library entity: {0}")]
    LibEntityMeta(#[from] LibEntityMetaError),
    #[error("library: {0}")]
    Library(#[from] LibraryError),
    #[error("storage: {0}")]
    Storage(#[from] StorageError),
    #[error("scripts: {0}")]
    Scripts(#[from] ScriptsError),
    #[error("{0}")]
    Other(#[from] ComError),
}

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

    fn libentities(&self, app: &App) -> CMDResult<Vec<LibEntity>> {
        let paths = app.library().storage().borrow().keys_path()?;

        paths.into_iter()
            .map(|path| Ok(
                // `.unwrap()` is here because we know that `path` is valid.
                app.library().get_libentity(path)?.unwrap().into_canonical_libentity()?
            ))
            .collect::<Result<Vec<LibEntity>, _>>()
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
        let libentities = self.libentities(app)?;
        let context = self.make_context(app)?;

        let listed = match self.mode {
            ListMode::Wide => app.scripts().list_output_wide(libentities, context)?,
            ListMode::Narrow => app.scripts().list_output_narrow(libentities, context)?,
        };

        Ok(listed)
    }
}

impl PCommand for ListPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let listed = self.execute_inner(app).commonize()?;
        println!("{}", listed);

        Ok(())
    }
}
