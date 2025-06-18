use crate::entity_data::EntityData;
use crate::error_ext::ResultExt;
use crate::types::{
    create_from_static_libentity, LEMError, LibEntity, LibEntityConst, LibEntityMut,
};

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Implements operations with library entities (`LibEntity`) through storage (`EntityData`).
pub struct Library {
    entity_data: Rc<RefCell<EntityData>>,
}

impl Library {
    pub fn new(storage: EntityData) -> Self {
        Library {
            entity_data: Rc::new(RefCell::new(storage)),
        }
    }

    pub fn get_libentity(&self, path: PathBuf) -> Option<LibEntityConst> {
        let maybe_libentity = LibEntityConst::new(self.entity_data(), path);
        match maybe_libentity {
            Ok(libentity) => Some(libentity),
            Err(LEMError::CouldNotFindLibEntity) => None,
        }
    }

    pub fn get_libentity_mut(&mut self, path: PathBuf) -> Option<LibEntityMut> {
        let maybe_libentity = LibEntityMut::new(self.entity_data(), path);
        match maybe_libentity {
            Ok(libentity) => Some(libentity),
            Err(LEMError::CouldNotFindLibEntity) => None,
        }
    }

    pub fn rename(&mut self, path: &Path, new_path: &Path) {
        self.entity_data
            .borrow_mut()
            .rename(path, new_path)
            .unwrap_or_explosion()
    }

    /// Creates library entity without dumping to storage.
    ///
    /// If library entity with `libentity.path` already exists, then all the other fields will be
    /// replaced with the ones in `libentity` field.
    pub fn create_from_static_libentity(&mut self, libentity: LibEntity) -> LibEntityMut {
        create_from_static_libentity(self.entity_data(), libentity)
    }

    pub fn entity_data(&self) -> Rc<RefCell<EntityData>> {
        Rc::clone(&self.entity_data)
    }

    pub fn libentity_exists(&self, path: &Path) -> bool {
        self.entity_data.borrow().exists(path)
    }

    pub fn get_all_libentities(&self) -> Vec<LibEntityConst> {
        self.entity_data
            .borrow()
            .list()
            .unwrap_or_explosion()
            .map(|path| self.get_libentity(path.unwrap_or_explosion()).unwrap())
            .collect()
    }
}
