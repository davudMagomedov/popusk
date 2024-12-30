use super::{PCommand, PExecutionError};

use crate::app::App;
use crate::error_ext::{ComError, CommonizeResultExt};
use crate::scripts::{Context, ScriptsError};
use crate::types::{LibEntityMut, EntityType, Progress, LibEntityMetaError};
use crate::library::LibraryError;
use crate::comps_interaction::libentity_has_progress;

use std::path::PathBuf;

use thiserror::Error as ThisError;

type CMDResult<T, E = OpenError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum OpenError {
    #[error("could not find library entity with path '{path}'")]
    CouldNotFindLibEntity { path: PathBuf },
    #[error("expected new progress from 'open_libentity' script")]
    ExpectedNewProgress,
    #[error("unexpected new progress from 'open_libentity' script")]
    UnexpectedNewProgress,

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
pub struct OpenPCMD {
    path: PathBuf,
    just_look: bool,
}

impl OpenPCMD {
    pub fn new(path: PathBuf, just_look: bool) -> Self {
        OpenPCMD { path, just_look }
    }

    fn get_libentity(&self, app: &mut App) -> CMDResult<LibEntityMut> {
        match app.library_mut().get_libentity_mut(self.path.clone())? {
            Some(libentity) => Ok(libentity),
            None => Err(OpenError::CouldNotFindLibEntity { path: self.path.clone() })
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
    
    fn normalize_new_progress(
        &self,
        maybe_new_progress: Option<Progress>,
        etype: EntityType,
    ) -> CMDResult<Option<Progress>> {
        use OpenError::{ExpectedNewProgress, UnexpectedNewProgress};

        if libentity_has_progress(etype) {
            match maybe_new_progress {
                Some(new_progress) => Ok(Some(new_progress)),
                None => Err(ExpectedNewProgress),
            }
        } else {
            match maybe_new_progress {
                Some(_) => Err(UnexpectedNewProgress),
                None => Ok(None),
            }
        }
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<Option<Progress>> {
        let mut libentity = self.get_libentity(app)?;
        let libentity_canonic = libentity.into_canonical_libentity()?;
        let etype = libentity_canonic.etype();
        let context = self.make_context(app)?;

        let maybe_new_progress = app.scripts().open_libentity(libentity_canonic, context)?;
        let maybe_new_progress = self.normalize_new_progress(maybe_new_progress, etype)?;

        libentity.set_progress(maybe_new_progress)?;
        libentity.dump_to_storage()?;

        Ok(maybe_new_progress)
    }
}

impl PCommand for OpenPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let _maybe_new_progress = self.execute_inner(app).commonize()?;

        Ok(())
    }
}
