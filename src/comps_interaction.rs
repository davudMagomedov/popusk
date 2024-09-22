//! Here is defined how components (progress, bases, etc) interacts without any context (such as
//! `Storage`, `Config`, etc).

use crate::types::EntityType;

pub fn libentity_has_progress(etype: EntityType) -> bool {
    etype == EntityType::Document
}
