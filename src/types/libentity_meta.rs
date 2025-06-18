use crate::entity_data::{EDError, EntityData};
use crate::error_ext::ResultExt;
use crate::types::{EntityBase, EntityType, FreeData, LibEntity, Progress, Tag};

use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::rc::Rc;

type LEMResult<T> = Result<T, LEMError>;

#[derive(Debug)]
pub enum LEMError {
    CouldNotFindLibEntity,
}

#[derive(Debug)]
struct LibEntityDataCached {
    name: Option<String>,
    etype: Option<EntityType>,
    tags: Option<Vec<Tag>>,
    progress: Option<Option<Progress>>,
    description: Option<Option<String>>,
    freedata: Option<Option<FreeData>>,
}

impl LibEntityDataCached {
    fn empty() -> Self {
        LibEntityDataCached {
            name: None,
            etype: None,
            tags: None,
            progress: None,
            description: None,
            freedata: None,
        }
    }
}

/// Invariant: `entity_data.borrow().exists(path)` is always true. It's not necessary all
/// elements to be in the entity data directory.
pub struct LibEntityMeta {
    entity_data: Rc<RefCell<EntityData>>,
    path: PathBuf,
    data_cached: RefCell<LibEntityDataCached>,
}

impl LibEntityMeta {
    /// ## Error
    /// Returns LEMError::CouldNotFindLibEntity if library entity was not found.
    pub fn new(entity_data: Rc<RefCell<EntityData>>, path: PathBuf) -> LEMResult<Self> {
        if !entity_data.borrow().exists(&path) {
            return Err(LEMError::CouldNotFindLibEntity);
        }

        Ok(LibEntityMeta {
            entity_data,
            path,
            data_cached: RefCell::new(LibEntityDataCached::empty()),
        })
    }

    pub unsafe fn new_unchecked(entity_data: Rc<RefCell<EntityData>>, path: PathBuf) -> Self {
        LibEntityMeta {
            entity_data,
            path,
            data_cached: RefCell::new(LibEntityDataCached::empty()),
        }
    }

    /// If library entity with `libentity.path` already exists, then all the other fields will be
    /// replaced with the ones in `libentity` field.
    pub fn create_from_static_libentity(
        entity_data: Rc<RefCell<EntityData>>,
        libentity: LibEntity,
    ) -> Self {
        let borrowed_entity_data = entity_data.borrow_mut();
        if !borrowed_entity_data.exists(&libentity.path) {
            borrowed_entity_data
                .touch(&libentity.path)
                .unwrap_or_explosion_with(|| {
                    format!(
                        "creating library entity '{}'",
                        libentity.path.to_string_lossy()
                    )
                });
        }
        drop(borrowed_entity_data);

        // SAFETY: entity data for `path` was created earlier.
        let mut meta = unsafe { LibEntityMeta::new_unchecked(entity_data, libentity.path) };
        meta.set_name(libentity.name);
        meta.set_tags(libentity.tags);
        meta.set_etype(libentity.etype);
        meta.set_progress(libentity.progress);
        meta.set_description(libentity.description);
        meta.set_freedata(libentity.freedata);

        meta
    }

    pub fn produce_libentity(&self) -> LibEntity {
        let path = self.path.clone();
        let name = self.name();
        let etype = self.etype();
        let tags = self.tags();
        let progress = self.progress();
        let description = self.description();
        let freedata = self.freedata();
        LibEntity {
            path,
            name,
            etype,
            tags,
            progress,
            description,
            freedata,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn name(&self) -> String {
        if self.name_cached() {
            // SAFETY: cached.
            unsafe { self.name_raw().unwrap_unchecked() }
        } else {
            self.cache_ebase(self.read_ebase_from_storage());
            // SAFETY: was cached one line above.
            unsafe { self.name_raw().unwrap_unchecked() }
        }
    }

    pub fn etype(&self) -> EntityType {
        if self.etype_cached() {
            // SAFETY: cached.
            unsafe { self.etype_raw().unwrap_unchecked() }
        } else {
            self.cache_ebase(self.read_ebase_from_storage());
            // SAFETY: was cached one line above.
            unsafe { self.etype_raw().unwrap_unchecked() }
        }
    }

    pub fn tags(&self) -> Vec<Tag> {
        if self.tags_cached() {
            // SAFETY: cached.
            unsafe { self.tags_raw().unwrap_unchecked() }
        } else {
            self.cache_ebase(self.read_ebase_from_storage());
            // SAFETY: was cached one line above.
            unsafe { self.tags_raw().unwrap_unchecked() }
        }
    }

    pub fn progress(&self) -> Option<Progress> {
        if self.progress_cached() {
            // SAFETY: cached.
            unsafe { self.progress_raw().unwrap_unchecked() }
        } else {
            self.cache_progress(self.read_progress_from_storage());
            // SAFETY: was cached one line above.
            unsafe { self.progress_raw().unwrap_unchecked() }
        }
    }

    pub fn description(&self) -> Option<String> {
        if self.description_cached() {
            // SAFETY: cached.
            unsafe { self.description_raw().unwrap_unchecked() }
        } else {
            self.cache_description(self.read_description_from_storage());
            // SAFETY: was cached one line above.
            unsafe { self.description_raw().unwrap_unchecked() }
        }
    }

    pub fn freedata(&self) -> Option<FreeData> {
        if self.freedata_cached() {
            // SAFETY: cached.
            unsafe { self.freedata_raw().unwrap_unchecked() }
        } else {
            self.cache_freedata(self.read_freedata_from_storage());
            // SAFETY: was cached one line above.
            unsafe { self.freedata_raw().unwrap_unchecked() }
        }
    }

    pub fn set_name(&mut self, name: String) {
        let _ = self.data_cached.borrow_mut().name.insert(name);
    }

    pub fn set_etype(&mut self, etype: EntityType) {
        let _ = self.data_cached.borrow_mut().etype.insert(etype);
    }

    pub fn set_tags(&mut self, tags: Vec<Tag>) {
        let _ = self.data_cached.borrow_mut().tags.insert(tags);
    }

    /// Keep in mind that progress cannot be in library entity with entity type != Document.
    pub fn set_progress(&mut self, progress: Option<Progress>) {
        let _ = self.data_cached.borrow_mut().progress.insert(progress);
    }

    pub fn set_description(&mut self, description: Option<String>) {
        let _ = self
            .data_cached
            .borrow_mut()
            .description
            .insert(description);
    }

    pub fn set_freedata(&mut self, freedata: Option<FreeData>) {
        let _ = self.data_cached.borrow_mut().freedata.insert(freedata);
    }

    pub fn dump_to_storage(&self) {
        self.dump_ebase_to_storage().unwrap_or_explosion_with(|| {
            format!("dumping entity base for '{}'", self.path.to_string_lossy())
        });
        self.dump_progress_to_storage()
            .unwrap_or_explosion_with(|| {
                format!("dumping progress for '{}'", self.path.to_string_lossy())
            });
        self.dump_description_to_storage()
            .unwrap_or_explosion_with(|| {
                format!("dumping description for '{}'", self.path.to_string_lossy())
            });
        self.dump_freedata_to_storage()
            .unwrap_or_explosion_with(|| {
                format!("dumping freedata for '{}'", self.path.to_string_lossy())
            });
    }

    fn delete(self) {
        self.entity_data
            .borrow_mut()
            .delete(&self.path)
            .unwrap_or_explosion_with(|| {
                format!("deleting library entity '{}'", self.path.to_string_lossy())
            });
    }

    pub fn load(&self) {
        self.cache_ebase(self.read_ebase_from_storage());
        self.cache_progress(self.read_progress_from_storage());
        self.cache_description(self.read_description_from_storage());
        self.cache_freedata(self.read_freedata_from_storage());
    }

    fn dump_ebase_to_storage(&self) -> Result<(), EDError> {
        let mut borrowed_entitydata = self.entity_data.borrow_mut();
        if let Some(ebase) = self.ebase_raw() {
            borrowed_entitydata.set_ebase(&self.path, ebase)
        } else {
            Ok(())
        }
    }

    fn dump_progress_to_storage(&self) -> Result<(), EDError> {
        let mut borrowed_entitydata = self.entity_data.borrow_mut();
        match self.progress_raw() {
            Some(Some(progress)) => borrowed_entitydata.set_progress(&self.path, progress),
            Some(None) => borrowed_entitydata.unset_progress(&self.path),
            _ => Ok(()),
        }
    }

    fn dump_description_to_storage(&self) -> Result<(), EDError> {
        let mut borrowed_entitydata = self.entity_data.borrow_mut();
        match self.description_raw() {
            Some(Some(description)) => borrowed_entitydata.set_description(&self.path, description),
            Some(None) => borrowed_entitydata.unset_description(&self.path),
            _ => Ok(()),
        }
    }

    fn dump_freedata_to_storage(&self) -> Result<(), EDError> {
        let mut borrowed_entitydata = self.entity_data.borrow_mut();
        match self.freedata_raw() {
            Some(Some(freedata)) => borrowed_entitydata.set_freedata(&self.path, freedata),
            Some(None) => borrowed_entitydata.unset_freedata(&self.path),
            _ => Ok(()),
        }
    }

    fn ebase_raw(&self) -> Option<EntityBase> {
        let datacached_borrowed = self.data_cached.borrow();
        Some(EntityBase {
            name: datacached_borrowed.name.clone()?,
            etype: datacached_borrowed.etype.clone()?,
            tags: datacached_borrowed.tags.clone()?,
        })
    }

    fn name_raw(&self) -> Option<String> {
        self.data_cached.borrow().name.clone()
    }

    fn etype_raw(&self) -> Option<EntityType> {
        self.data_cached.borrow().etype.clone()
    }

    fn tags_raw(&self) -> Option<Vec<Tag>> {
        self.data_cached.borrow().tags.clone()
    }

    fn progress_raw(&self) -> Option<Option<Progress>> {
        self.data_cached.borrow().progress.clone()
    }

    fn description_raw(&self) -> Option<Option<String>> {
        self.data_cached.borrow().description.clone()
    }

    fn freedata_raw(&self) -> Option<Option<FreeData>> {
        self.data_cached.borrow().freedata.clone()
    }

    fn name_cached(&self) -> bool {
        self.data_cached.borrow().name.is_some()
    }

    fn etype_cached(&self) -> bool {
        self.data_cached.borrow().etype.is_some()
    }

    fn tags_cached(&self) -> bool {
        self.data_cached.borrow().tags.is_some()
    }

    fn progress_cached(&self) -> bool {
        self.data_cached.borrow().progress.is_some()
    }

    fn description_cached(&self) -> bool {
        self.data_cached.borrow().description.is_some()
    }

    fn freedata_cached(&self) -> bool {
        self.data_cached.borrow().freedata.is_some()
    }

    fn cache_ebase(&self, ebase: EntityBase) {
        let _ = self.data_cached.borrow_mut().name.insert(ebase.name);
        let _ = self.data_cached.borrow_mut().etype.insert(ebase.etype);
        let _ = self.data_cached.borrow_mut().tags.insert(ebase.tags);
    }

    fn cache_progress(&self, progress: Option<Progress>) {
        let _ = self.data_cached.borrow_mut().progress.insert(progress);
    }

    fn cache_description(&self, description: Option<String>) {
        let _ = self
            .data_cached
            .borrow_mut()
            .description
            .insert(description);
    }

    fn cache_freedata(&self, freedata: Option<FreeData>) {
        let _ = self.data_cached.borrow_mut().freedata.insert(freedata);
    }

    fn read_ebase_from_storage(&self) -> EntityBase {
        self.entity_data
            .borrow()
            .get_ebase(&self.path)
            .unwrap_or_explosion_with(|| {
                format!("reading entity base of '{}'", self.path.to_string_lossy())
            })
    }

    fn read_progress_from_storage(&self) -> Option<Progress> {
        let borrowed_storage = self.entity_data.borrow();
        if borrowed_storage.progress_exists(&self.path) {
            Some(
                borrowed_storage
                    .get_progress(&self.path)
                    .unwrap_or_explosion_with(|| {
                        format!("reading progress of '{}'", self.path.to_string_lossy())
                    }),
            )
        } else {
            None
        }
    }

    fn read_description_from_storage(&self) -> Option<String> {
        let borrowed_storage = self.entity_data.borrow();
        if borrowed_storage.description_exists(&self.path) {
            Some(
                borrowed_storage
                    .get_description(&self.path)
                    .unwrap_or_explosion_with(|| {
                        format!("reading description of '{}'", self.path.to_string_lossy())
                    }),
            )
        } else {
            None
        }
    }

    fn read_freedata_from_storage(&self) -> Option<FreeData> {
        let borrowed_storage = self.entity_data.borrow();
        if borrowed_storage.freedata_exists(&self.path) {
            Some(
                borrowed_storage
                    .get_freedata(&self.path)
                    .unwrap_or_explosion_with(|| {
                        format!("reading freedata of '{}'", self.path.to_string_lossy())
                    }),
            )
        } else {
            None
        }
    }
}

pub struct LibEntityMut {
    meta: LibEntityMeta,
}

impl LibEntityMut {
    pub fn new(entity_data: Rc<RefCell<EntityData>>, path: PathBuf) -> LEMResult<Self> {
        LibEntityMeta::new(entity_data, path).map(|meta| LibEntityMut { meta })
    }

    pub fn delete(self) {
        self.meta.delete();
    }
}

impl Deref for LibEntityMut {
    type Target = LibEntityMeta;

    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}

impl DerefMut for LibEntityMut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.meta
    }
}

pub struct LibEntityConst {
    meta: LibEntityMeta,
}

impl LibEntityConst {
    pub fn new(entity_data: Rc<RefCell<EntityData>>, path: PathBuf) -> LEMResult<Self> {
        LibEntityMeta::new(entity_data, path).map(|meta| LibEntityConst { meta })
    }
}

impl Deref for LibEntityConst {
    type Target = LibEntityMeta;

    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}

/// If library entity with `libentity.path` already exists, then all the other fields will be
/// replaced with the ones in `libentity` field.
pub fn create_from_static_libentity(
    entity_data: Rc<RefCell<EntityData>>,
    static_libentity: LibEntity,
) -> LibEntityMut {
    LibEntityMut {
        meta: LibEntityMeta::create_from_static_libentity(entity_data, static_libentity),
    }
}
