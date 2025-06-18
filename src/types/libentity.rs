use crate::comps_appearance::{entitytype_from_string, entitytype_to_string};
use crate::types::{EntityType, FreeData, Progress, Tag};

use std::path::PathBuf;

use mlua::{Error as LuaError, FromLua, IntoLua, Lua, Result as LuaResult, Value as LuaValue};

/// Contains all attributes for library entity.
#[derive(Debug, Clone)]
pub struct LibEntity {
    pub path: PathBuf,
    pub name: String,
    pub etype: EntityType,
    pub tags: Vec<String>,
    pub progress: Option<Progress>,
    pub description: Option<String>,
    pub freedata: Option<FreeData>,
}

impl IntoLua for LibEntity {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let libentity_table = lua.create_table()?;

        libentity_table.set("path", self.path.to_string_lossy())?;
        libentity_table.set("name", self.name)?;
        libentity_table.set("tags", self.tags)?;
        libentity_table.set("etype", entitytype_to_string(self.etype))?;

        if let Some(progress) = self.progress {
            libentity_table.set("progress", progress.into_lua(lua)?)?;
        }

        if let Some(description) = self.description {
            libentity_table.set("description", description)?;
        }

        if let Some(freedata) = self.freedata {
            libentity_table.set("freedata", freedata.into_lua(lua)?)?;
        }

        Ok(LuaValue::Table(libentity_table))
    }
}

impl FromLua for LibEntity {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        let LuaValue::Table(table) = value else {
            return Err(LuaError::UserDataTypeMismatch);
        };

        let path: PathBuf = table.get::<String>("path")?.into();
        let name: String = table.get("name")?;
        let etype_str: String = table.get("etype")?;
        let etype = match entitytype_from_string(&etype_str) {
            Ok(etype) => etype,
            Err(_) => return Err(LuaError::UserDataTypeMismatch),
        };
        let tags: Vec<Tag> = table.get("tags")?;
        let progress: Option<Progress> = table.get("progress")?;
        let description: Option<String> = table.get("description")?;
        let freedata: Option<FreeData> = table.get("freedata")?;

        return Ok(LibEntity {
            path,
            name,
            etype,
            tags,
            progress,
            description,
            freedata,
        });
    }
}
