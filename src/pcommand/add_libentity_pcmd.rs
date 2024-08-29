use crate::app::App;
use crate::comps_appearance::parse_string_to_tags;
use crate::comps_interaction::libentity_has_progress;
use crate::entity_base::{EntityType, Tag};
use crate::error_ext::ComResult;
use crate::libentity::LibEntityData;
use crate::progress::Progress;

use super::{PCommand, PExecutionError};

use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

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

    fn read_name(&self) -> ComResult<String> {
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

    fn read_tags(&self) -> ComResult<Vec<Tag>> {
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

    fn read_progceil(&self) -> ComResult<usize> {
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

    fn read_etype(&self, app: &App) -> ComResult<EntityType> {
        if !self.path.exists() {
            return Err(format!("the file doesn't exist: {}", self.path.to_string_lossy()).into());
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

    fn read_description(&self) -> ComResult<Option<String>> {
        let mut stdout = stdout();
        stdout.write_all(b"Description (leave empty if none): ")?;
        stdout.flush()?;

        let mut description = String::new();
        stdin().read_line(&mut description)?;

        if !description.trim().is_empty() {
            Ok(Some(description.trim().to_string()))
        } else {
            Ok(None)
        }
    }
}

impl PCommand for AddLibentityPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        // The attributes must be defined in the start of the function.
        let name = self.read_name()?;
        let tags = self.read_tags()?;
        let etype = self.read_etype(app)?;
        let progress = if libentity_has_progress(etype) {
            Some(Progress::new(self.read_progceil()?))
        } else {
            None
        };
        let description = self.read_description()?;

        let libentity_data = LibEntityData {
            path: self.path.clone(),
            description,
            etype,
            name,
            progress,
            tags,
        };

        app.library_mut().add_libentity(libentity_data)?;

        Ok(())
    }
}
