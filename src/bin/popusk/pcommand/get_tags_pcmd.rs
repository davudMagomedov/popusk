use super::{PCommand, PEResult, PExecError};

use popusk::{App, LibEntityConst, Tag};

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GetTagsPCMD {
    path: PathBuf,
}

impl GetTagsPCMD {
    pub fn new(path: PathBuf) -> Self {
        GetTagsPCMD { path }
    }

    fn get_libentity(&self, app: &App) -> PEResult<LibEntityConst> {
        app.library()
            .get_libentity(self.path.clone())
            .ok_or_else(|| PExecError::LibEntityWasNotFound {
                entitypath: self.path.clone(),
            })
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<Vec<Tag>> {
        Ok(self.get_libentity(app)?.tags())
    }

    fn print_tags(&self, tags: &[Tag]) {
        if tags.is_empty() {
            println!("<no tag>");
            return;
        }

        print!("{}", tags[0]);

        for i in 1..tags.len() {
            print!(" {}", tags[i]);
        }
        println!();
    }
}

impl PCommand for GetTagsPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecError> {
        let tags = self.execute_inner(app)?;
        self.print_tags(&tags);

        Ok(())
    }
}
