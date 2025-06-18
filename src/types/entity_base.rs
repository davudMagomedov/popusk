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
    pub name: String,
    pub etype: EntityType,
    pub tags: Vec<Tag>,
}
