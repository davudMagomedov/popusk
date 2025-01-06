use crate::types::{EntityType, Progress, ID, FreeData, Tag};
use crate::comps_appearance::{entitytype_from_string, entitytype_to_string};

use std::path::PathBuf;

use mlua::{Lua, Result as LuaResult, Error as LuaError, FromLua, IntoLua, Value as LuaValue};

/// Contains mutable attributes of library entity.
#[derive(Debug, Clone)]
pub struct LibEntityData {
    pub path: PathBuf,
    pub name: String,
    pub etype: EntityType,
    pub tags: Vec<String>,
    pub progress: Option<Progress>,
    pub description: Option<String>,
    pub freedata: Option<FreeData>,
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

    pub fn freedata(&self) -> Option<&FreeData> {
        self.data.freedata.as_ref()
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

    pub fn data_mut(&mut self) -> &mut LibEntityData {
        &mut self.data
    }

    pub fn data(&self) -> &LibEntityData {
        &self.data
    }

    pub fn extract_data(self) -> LibEntityData {
        self.data
    }
}

impl IntoLua for LibEntity {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let libentity_table = lua.create_table()?;

        libentity_table.set("path", self.path().to_string_lossy())?;
        libentity_table.set("id", self.id().to_string())?;
        libentity_table.set("name", self.name().clone())?;
        libentity_table.set("tags", self.tags().clone())?;
        libentity_table.set("etype", entitytype_to_string(self.etype()))?;

        if let Some(progress) = self.progress() {
            libentity_table.set("progress", progress.into_lua(lua)?)?;
        }

        if let Some(description) = self.description() {
            libentity_table.set("description", description.clone())?;
        }

        if let Some(freedata) = self.freedata() {
            libentity_table.set("freedata", freedata.clone().into_lua(lua)?)?;
        }

        Ok(LuaValue::Table(libentity_table))
    }
}

impl IntoLua for LibEntityData {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let LibEntityData {
            path, name, etype, tags, progress, description, freedata
        } = self;

        let libentity_data_table = lua.create_table()?;

        libentity_data_table.set("path", path.to_string_lossy())?;
        libentity_data_table.set("name", name)?;
        libentity_data_table.set("tags", tags)?;
        libentity_data_table.set("etype", entitytype_to_string(etype))?;

        if let Some(progress) = progress {
            libentity_data_table.set("progress", progress.into_lua(lua)?)?;
        }

        if let Some(description) = description {
            libentity_data_table.set("description", description)?;
        }

        if let Some(freedata) = freedata {
            libentity_data_table.set("freedata", freedata.into_lua(lua)?)?;
        }

        Ok(LuaValue::Table(libentity_data_table))
    }
}

impl FromLua for LibEntityData {
    // fuck why is `lua` field even exists if it's quite useless.
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

        let data = LibEntityData {
            path, name, etype, tags, progress, description, freedata
        };

        return Ok(data);
    }
}
