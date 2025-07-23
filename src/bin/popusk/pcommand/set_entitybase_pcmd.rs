use super::minilib::{verify_etype, verify_tags};
use super::{PCommand, PEResult, PExecError};

use crate::io_ext::IoExt;

use popusk::app::App;
use popusk::error_ext::ErrorExt;
use popusk::types::{EntityBase, LibEntityMut};

use std::io::stdin;
use std::path::PathBuf;

use serde_json::from_str as from_json_str;

/// UNSAFE COMMAND
#[derive(Debug, Clone)]
pub struct SetEntitybasePCMD {
    path: PathBuf,
}

impl SetEntitybasePCMD {
    pub fn new(path: PathBuf) -> Self {
        SetEntitybasePCMD { path }
    }

    fn get_libentity(&self, app: &mut App) -> PEResult<LibEntityMut> {
        app.library_mut()
            .get_libentity_mut(self.path.clone())
            .ok_or_else(|| PExecError::LibEntityWasNotFound {
                entitypath: self.path.clone(),
            })
    }

    fn read_ebase(&self) -> PEResult<EntityBase> {
        let json_ebase = match stdin().read_to_end_string() {
            Ok(js) => js,
            Err(ioerr) => return Err(PExecError::StdinStream { ioerr }),
        };
        let ebase =
            from_json_str::<EntityBase>(&json_ebase).map_err(|error| PExecError::DeserError {
                format: "json",
                component: "entity base",
                error: error.into_box(),
            })?;

        Ok(ebase)
    }

    fn verify_ebase(&self, ebase: &EntityBase, libentity: &LibEntityMut) -> PEResult<()> {
        let mut errs = Vec::with_capacity(2);
        if let Some(err) = verify_tags(&ebase.tags).err() {
            errs.push(err);
        }
        if let Some(err) = verify_etype(ebase.etype, &libentity).err() {
            errs.push(err);
        }
        match errs.is_empty() {
            true => Ok(()),
            false => Err(PExecError::Multiple(errs)),
        }
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<()> {
        let mut libentity = self.get_libentity(app)?;
        let ebase = self.read_ebase()?;

        self.verify_ebase(&ebase, &libentity)?;

        libentity.set_name(ebase.name);
        libentity.set_etype(ebase.etype);
        libentity.set_tags(ebase.tags);
        libentity.dump_to_storage();

        Ok(())
    }
}

impl PCommand for SetEntitybasePCMD {
    fn execute(&self, app: &mut App) -> PEResult<()> {
        self.execute_inner(app)?;

        println!(
            "The entity base was added in library entity '{}'",
            self.path.to_string_lossy()
        );

        Ok(())
    }
}
