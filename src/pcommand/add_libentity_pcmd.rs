use crate::app::App;
use crate::comps_appearance::parse_string_to_tags;
use crate::comps_interaction::libentity_has_progress;
use crate::entity_base::{EntityBase, EntityType, Tag};
use crate::progress::Progress;

use super::{PCommand, PExecutionError};

use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct AddLibentityPCMD {
    path: PathBuf,
    name: Option<String>,
    tags: Option<String>,
    prog_ceil: Option<usize>,
}

impl AddLibentityPCMD {
    pub fn new(
        path: PathBuf,
        name: Option<String>,
        tags: Option<String>,
        prog_ceil: Option<usize>,
    ) -> Self {
        AddLibentityPCMD {
            path,
            name,
            tags,
            prog_ceil,
        }
    }

    fn read_name(&self) -> Result<String, anyhow::Error> {
        if let Some(name) = &self.name {
            return Ok(name.clone());
        }

        let mut stdout = stdout();
        stdout.write_all(b"Name: ")?;
        stdout.flush()?;

        let mut name = String::new();
        stdin().read_line(&mut name)?;

        Ok(name.trim().to_string())
    }

    fn read_tags(&self) -> Result<Vec<Tag>, anyhow::Error> {
        if let Some(stringified_tags) = &self.tags {
            return Ok(parse_string_to_tags(stringified_tags)?);
        }

        let mut stdout = stdout();
        stdout.write_all(b"Tags (comma-separated): ")?;
        stdout.flush()?;

        let mut stringified_tags = String::new();
        stdin().read_line(&mut stringified_tags)?;

        Ok(parse_string_to_tags(stringified_tags.trim())?)
    }

    fn read_progceil(&self) -> Result<usize, anyhow::Error> {
        if let Some(prog_ceil) = self.prog_ceil {
            return Ok(prog_ceil);
        }

        let mut stdout = stdout();
        stdout.write_all(b"Progress ceiling: ")?;
        stdout.flush()?;

        let mut stringified_progceil = String::new();
        stdin().read_line(&mut stringified_progceil)?;

        let prog_ceil = stringified_progceil.trim().parse::<usize>()?;

        Ok(prog_ceil)
    }

    fn read_etype(&self, app: &App) -> Result<EntityType, anyhow::Error> {
        app.config().document_extension();

        if !self.path.exists() {
            return Err(anyhow!(
                "the file doesn't exist: {}",
                self.path.to_string_lossy()
            ));
        }

        if self.path.is_dir() {
            Ok(EntityType::Section)
        } else if self.path.is_file() {
            let extension = self
                .path
                .extension()
                .map(|t| t.to_string_lossy().to_string())
                .unwrap_or_else(|| "".to_string());

            if app.config().document_extension().contains(&extension) {
                Ok(EntityType::Document)
            } else {
                Ok(EntityType::Regular)
            }
        } else {
            Ok(EntityType::Regular)
        }
    }
}

impl PCommand for AddLibentityPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        // The attributes must be defined in the start of the function.
        let name = self.read_name()?;
        let tags = self.read_tags()?;
        let etype = self.read_etype(app)?;
        let maybe_progceil = if libentity_has_progress(etype) {
            Some(self.read_progceil()?)
        } else {
            None
        };

        // Critical section {

        // FIX: The function can raise error here and invariants will be violated. This section
        // must be executed atomically.

        let id = crate::core_commands::corecmd_add_path(app.storage_mut(), self.path.clone())?;

        let entitybase = EntityBase::new(id, name, etype, tags);
        app.storage_mut().link_entitybase_to_id(id, entitybase)?;

        if let Some(progceil) = maybe_progceil {
            app.storage_mut()
                .link_progress_to_id(id, Progress::new(progceil))?;
        }

        // } critical section

        Ok(())
    }
}
