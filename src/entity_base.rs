use crate::id::ID;

use serde_derive::{Deserialize, Serialize};

pub type Tag = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    Section,
    Document,
    Regular,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityBase {
    id: ID,
    name: String,
    etype: EntityType,
    tags: Vec<Tag>,
}

impl EntityBase {
    pub fn new(id: ID, name: String, etype: EntityType, tags: Vec<Tag>) -> EntityBase {
        EntityBase {
            id,
            name,
            etype,
            tags,
        }
    }

    pub fn id(&self) -> ID {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn etype(&self) -> EntityType {
        self.etype
    }

    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn tags_mut(&mut self) -> &mut Vec<Tag> {
        &mut self.tags
    }

    pub fn destruct(self) -> (ID, String, EntityType, Vec<Tag>) {
        let EntityBase {
            id,
            name,
            etype,
            tags,
        } = self;

        (id, name, etype, tags)
    }
}
