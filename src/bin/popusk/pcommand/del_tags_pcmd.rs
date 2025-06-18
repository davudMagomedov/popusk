use super::{PCommand, PEResult, PExecError};

use popusk::app::App;
use popusk::error_ext::ErrorExt;
use popusk::types::{LibEntityMut, Tag};

use std::io::stdin;
use std::path::PathBuf;

fn delete_indexes_in_vector<T>(base_vector: Vec<T>, delete_indexes: &[usize]) -> Vec<T> {
    base_vector
        .into_iter()
        .enumerate()
        .filter_map(|(position, element)| {
            if !delete_indexes.contains(&position) {
                Some(element)
            } else {
                None
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct DelTagsPCMD {
    path: PathBuf,
}

fn print_tags(tags: &[Tag]) {
    tags.into_iter()
        .enumerate()
        .for_each(|(index, tag)| println!("{}: {}", index, tag))
}

fn read_indexes_to_delete() -> PEResult<Vec<usize>> {
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .map_err(|ioerr| PExecError::StdinStream { ioerr })?;
    buf = buf.trim().to_string();

    let indexes = buf
        .split_whitespace()
        .map(|stried_index| {
            stried_index
                .parse::<usize>()
                .map_err(|interr| PExecError::DeserError {
                    format: "string",
                    component: "index (unsigned integer)",
                    error: interr.into_box(),
                })
        })
        .collect::<PEResult<Vec<usize>>>()?;

    Ok(indexes)
}

impl DelTagsPCMD {
    pub fn new(path: PathBuf) -> Self {
        DelTagsPCMD { path }
    }

    fn get_libentity(&self, app: &mut App) -> PEResult<LibEntityMut> {
        app.library_mut()
            .get_libentity_mut(self.path.clone())
            .ok_or_else(|| PExecError::LibEntityWasNotFound {
                entitypath: self.path.clone(),
            })
    }

    fn execute_inner(&self, app: &mut App) -> PEResult<()> {
        let mut libentity = self.get_libentity(app)?;
        let tags = libentity.tags();

        print_tags(&tags);
        let indexes_to_delete = read_indexes_to_delete()?;

        let updated_tags = delete_indexes_in_vector(tags, &indexes_to_delete);
        libentity.set_tags(updated_tags);
        libentity.dump_to_storage();

        Ok(())
    }
}

impl PCommand for DelTagsPCMD {
    fn execute(&self, app: &mut App) -> PEResult<()> {
        self.execute_inner(app)?;

        println!("The selected tags were deleted");

        Ok(())
    }
}
