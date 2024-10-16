//! Free data is an additional information for library entities. For example, free data can keep a
//! name of author, date of adding the library entity, etc.
//!
//! Free data is like the others types (id, progress, etc), but a more massive. There're no
//! restrictions on how to store your data.
//!
//! ## Available types
//!
//! Free data (structure `FreeData`) keeps root node - a *dictionary*.
//!
//! There're following types of free data objects:
//! 1. Integer, aka `i64`.
//! 2. String
//! 3. Boolean
//! 4. Array
//! 5. Dictionary, aka `HashMap<String, FDataObject>`

use std::collections::HashMap;

use mlua::{
    Error as LuaError, FromLua, IntoLua, Lua, Result as LuaResult, Table as LuaTable,
    Value as LuaValue,
};
use serde_derive::{Deserialize, Serialize};

pub type FDataInteger = i64;
pub type FDataString = String;
pub type FDataBoolean = bool;
pub type FDataArray = Box<[FDataObject]>;
pub type FDataDictionary = HashMap<String, FDataObject>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FDataObject {
    Integer(FDataInteger),
    String(FDataString),
    Boolean(FDataBoolean),
    Dictionary(FDataDictionary),
    Array(FDataArray),
}

impl IntoLua for FDataObject {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        match self {
            FDataObject::Integer(integer) => Ok(LuaValue::Integer(integer)),
            FDataObject::String(string) => Ok(string.into_lua(lua)?),
            FDataObject::Boolean(boolean) => Ok(LuaValue::Boolean(boolean)),
            FDataObject::Dictionary(dictionary) => Ok(dictionary.into_lua(lua)?),
            FDataObject::Array(array) => Ok(array.into_lua(lua)?),
        }
    }
}

fn array_from_table(table: LuaTable) -> LuaResult<FDataArray> {
    let elements_count = table.len()? as usize;

    let mut maximum = 1;
    let mut values = Vec::with_capacity(elements_count);
    for pair in table.pairs::<usize, FDataObject>() {
        let (index, value) = pair?;
        if index == 0 {
            return Err(LuaError::UserDataTypeMismatch); // TODO: Error
        }
        values.push(value);
        maximum = maximum.max(index);
    }

    if maximum != elements_count {
        return Err(LuaError::UserDataTypeMismatch); // TODO: Error
    }

    Ok(Box::from(values))
}

fn dictionary_from_table(table: LuaTable) -> LuaResult<FDataDictionary> {
    table
        .pairs::<String, FDataObject>()
        .collect::<LuaResult<FDataDictionary>>()
}

impl FromLua for FDataObject {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Integer(integer) => Ok(FDataObject::Integer(integer)),
            LuaValue::String(string) => Ok(FDataObject::String(string.to_string_lossy())),
            LuaValue::Boolean(boolean) => Ok(FDataObject::Boolean(boolean)),
            LuaValue::Table(table) => {
                if table.contains_key(1)? {
                    // Then the table is an array
                    Ok(FDataObject::Array(array_from_table(table)?))
                } else {
                    // Then the table is dictionary
                    Ok(FDataObject::Dictionary(dictionary_from_table(table)?))
                }
            }
            _ => Err(LuaError::UserDataTypeMismatch),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FreeData {
    root: FDataDictionary,
}

impl FreeData {
    pub fn new() -> Self {
        FreeData {
            root: FDataDictionary::default(),
        }
    }

    pub fn new_with(root: FDataDictionary) -> Self {
        FreeData { root }
    }
}
