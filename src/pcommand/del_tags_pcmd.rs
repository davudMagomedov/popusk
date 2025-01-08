use crate::app::App;
use crate::error_ext::CommonizeResultExt;
use crate::types::{Tag, ID, LibEntityMut, LibEntityMetaError};
use crate::library::LibraryError;

use super::{PCommand, PExecutionError};

use std::io::{stdin, Error as IoError};
use std::num::ParseIntError;

use thiserror::Error as ThisError;

type CMDResult<T, E = CMDError> = Result<T, E>;

#[derive(Debug, ThisError)]
enum CMDError {
    #[error("library entity with ID {id} wasn't found")]
    LibEntityWasNotFound { id: ID },

    #[error("could not parse integer: {0}")]
    ParseInt(#[from] ParseIntError),
    #[error("library entity: {0}")]
    LibEntityMeta(#[from] LibEntityMetaError),
    #[error("library: {0}")]
    Library(#[from] LibraryError),
    #[error("i/o: {0}")]
    IO(#[from] IoError),
}

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
    id: ID,
}

fn print_tags(tags: &[Tag]) {
    tags.into_iter()
        .enumerate()
        .for_each(|(index, tag)| println!("{}: {}", index, tag))
}

fn get_indexes_to_delete() -> CMDResult<Vec<usize>> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    buf = buf.trim().to_string();

    let indexes = buf
        .split_whitespace()
        .map(|stried_index| stried_index.parse::<usize>())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    Ok(indexes)
}

impl DelTagsPCMD {
    pub fn new(id: ID) -> Self {
        DelTagsPCMD { id }
    }

    fn get_libentity(&self, app: &mut App) -> CMDResult<LibEntityMut> {
        app.library_mut()
            .get_libentity_mut_by_id(self.id)?
            .ok_or_else(|| CMDError::LibEntityWasNotFound { id: self.id })
    }

    fn execute_inner(&self, app: &mut App) -> CMDResult<()> {
        let mut libentity = self.get_libentity(app)?;
        let tags = libentity.tags()?;

        print_tags(&tags);
        let indexes_to_delete = get_indexes_to_delete()?;

        let updated_tags = delete_indexes_in_vector(tags, &indexes_to_delete);
        libentity.set_tags(updated_tags)?;
        libentity.dump_to_storage()?;

        Ok(())
    }
}

impl PCommand for DelTagsPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        self.execute_inner(app).commonize()?;
        println!("The selected tags ware deleted");

        Ok(())
    }
}
