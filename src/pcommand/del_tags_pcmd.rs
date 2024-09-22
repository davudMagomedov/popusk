use crate::app::App;
use crate::error_ext::ComError;
use crate::error_ext::CommonizeResultExt;
use crate::types::{Tag, ID};

use super::{PCommand, PExecutionError};

use std::io::stdin;
use std::num::ParseIntError;

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

impl DelTagsPCMD {
    pub fn new(id: ID) -> Self {
        DelTagsPCMD { id }
    }

    fn print_tags(&self, tags: &Vec<Tag>) {
        tags.into_iter()
            .enumerate()
            .for_each(|(index, tag)| println!("{}: {}", index, tag))
    }

    fn get_indexes_to_delete(&self) -> Result<Vec<usize>, PExecutionError> {
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;
        buf = buf.trim().to_string();

        let indexes = buf
            .split_whitespace()
            .map(|stried_index| stried_index.parse::<usize>())
            .collect::<Result<Vec<usize>, ParseIntError>>()
            .commonize()?;

        Ok(indexes)
    }
}

impl PCommand for DelTagsPCMD {
    fn execute(&self, app: &mut App) -> Result<(), PExecutionError> {
        let mut entitybase = match unsafe { app.library().storage() }.get_entitybase(self.id)? {
            Some(entitybase) => entitybase,
            None => {
                return Err(
                    ComError::from(format!("couldn't find entitybase for ID {}", self.id)).into(),
                )
            }
        };

        self.print_tags(entitybase.tags());

        let indexes_to_delete = self.get_indexes_to_delete()?;

        let old_tags = std::mem::replace(entitybase.tags_mut(), Vec::new());
        let done_tags = delete_indexes_in_vector(old_tags, &indexes_to_delete);
        _ = std::mem::replace(entitybase.tags_mut(), done_tags);

        unsafe { app.library_mut().storage_mut() }.update_entitybase(self.id, entitybase)?;

        Ok(())
    }
}
