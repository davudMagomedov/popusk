use crate::entity_base::EntityType;
use crate::id::ID;
use crate::progress::Progress;

use std::path::PathBuf;

/// Contains mutable attributes of library entity.
#[derive(Debug, Clone)]
pub struct LibEntityData {
    pub path: PathBuf,
    pub name: String,
    pub etype: EntityType,
    pub tags: Vec<String>,
    pub progress: Option<Progress>,
    pub description: Option<String>,
}

/// Contains all attributes from `LibEntityData` + id.
#[derive(Debug, Clone)]
pub struct LibEntity {
    id: ID,
    data: LibEntityData,
}

impl LibEntity {
    pub fn from_id_data(id: ID, data: LibEntityData) -> Self {
        LibEntity { data, id }
    }

    pub fn path(&self) -> &PathBuf {
        &self.data.path
    }

    // снилс копия
    // паспорт копия
    // 3 этаж, 33 кабинет

    pub fn progress(&self) -> Option<&Progress> {
        self.data.progress.as_ref()
    }

    pub fn description(&self) -> Option<&String> {
        self.data.description.as_ref()
    }

    pub fn name(&self) -> &String {
        &self.data.name
    }

    pub fn id(&self) -> ID {
        self.id
    }

    pub fn etype(&self) -> EntityType {
        self.data.etype
    }

    pub fn tags(&self) -> &Vec<String> {
        &self.data.tags
    }

    pub fn progress_mut(&mut self) -> Option<&mut Progress> {
        self.data.progress.as_mut()
    }

    pub fn description_mut(&mut self) -> Option<&mut String> {
        self.data.description.as_mut()
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.data.name
    }

    pub fn set_id(&mut self, new_id: ID) {
        self.id = new_id;
    }

    pub fn set_etype(&mut self, new_etype: EntityType) {
        self.data.etype = new_etype;
    }

    pub fn tags_mut(&mut self) -> &mut Vec<String> {
        &mut self.data.tags
    }
}
