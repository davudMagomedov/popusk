use super::{PCommand, PEResult, PExecError};

use popusk::app::App;
use popusk::scripts::Context;
use popusk::types::{LibEntity, StyledText};

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

    fn libentities(&self, app: &App) -> Vec<LibEntity> {
        app.library()
            .get_all_libentities()
            .into_iter()
            .map(|con| con.produce_libentity())
            .collect()
    }

    fn make_context(&self, _app: &App) -> PEResult<Context> {
        match Context::auto() {
            Some(context) => Ok(context),
            None => Err(PExecError::CouldNotCreateContext),
        }
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<StyledText> {
        let libentities = self.libentities(app);
        let context = self.make_context(app)?;

        let listed = match self.mode {
            ListMode::Wide => app.scripts().list_output_wide(libentities, context)?,
            ListMode::Narrow => app.scripts().list_output_narrow(libentities, context)?,
        };

        Ok(listed)
    }
}

impl PCommand for ListPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecError> {
        let listed = self.execute_inner(app)?;
        print!("{}", listed);

        Ok(())
    }
}
