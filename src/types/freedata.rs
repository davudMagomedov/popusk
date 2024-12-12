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

use crate::error_ext::ComResult;

use std::collections::HashMap;

use mlua::{
    Error as LuaError, FromLua, IntoLua, Lua, Result as LuaResult, Table as LuaTable,
    Value as LuaValue,
};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

pub type FDataInteger = i64;
pub type FDataFloat = f64;
pub type FDataString = String;
pub type FDataBoolean = bool;
pub type FDataArray = Box<[FDataObject]>;
pub type FDataDictionary = HashMap<String, FDataObject>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FDataObject {
    Integer(FDataInteger),
    Float(FDataFloat),
    String(FDataString),
    Boolean(FDataBoolean),
    Dictionary(FDataDictionary),
    Array(FDataArray),
    Null,
}

impl IntoLua for FDataObject {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        match self {
            FDataObject::Integer(integer) => Ok(LuaValue::Integer(integer)),
            FDataObject::String(string) => Ok(string.into_lua(lua)?),
            FDataObject::Float(float) => Ok(float.into_lua(lua)?),
            FDataObject::Boolean(boolean) => Ok(LuaValue::Boolean(boolean)),
            FDataObject::Dictionary(dictionary) => Ok(dictionary.into_lua(lua)?),
            FDataObject::Array(array) => Ok(array.into_lua(lua)?),
            FDataObject::Null => Ok(mlua::Nil),
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

const fn float_is_finite(fl: f64) -> bool {
    fl != f64::INFINITY && fl != f64::NEG_INFINITY
}

fn freedata_object_to_jsonvalue(fdata_object: FDataObject) -> ComResult<JsonValue> {
    match fdata_object {
        FDataObject::Null => Ok(JsonValue::Null),
        FDataObject::Float(fl) if float_is_finite(fl) => Ok(
            JsonValue::Number(serde_json::Number::from_f64(fl).unwrap())
        ),
        FDataObject::Integer(integer) => Ok(
            JsonValue::Number(serde_json::Number::from(integer))
        ),
        FDataObject::String(string) => Ok(JsonValue::String(string)),
        FDataObject::Boolean(boolean) => Ok(JsonValue::Bool(boolean)),
        FDataObject::Array(array) => {
            let json_array = array
                .into_vec()
                .into_iter()
                .map(|fdata_object| freedata_object_to_jsonvalue(fdata_object))
                .collect::<ComResult<_>>()?;
            Ok(JsonValue::Array(json_array))
        }
        FDataObject::Dictionary(dictionary) => {
            let json_dictionary = dictionary
                .into_iter()
                .map(|(key, fdata_object)| Ok(
                    (key, freedata_object_to_jsonvalue(fdata_object)?)
                )).collect::<ComResult<_>>()?;

            Ok(JsonValue::Object(json_dictionary))
        }
        _ => Err(format!("couldn't recognize FreeData as json value").into())
    }
}

pub fn freedata_to_json(freedata: FreeData) -> ComResult<JsonValue> {
    let json = freedata_object_to_jsonvalue(FDataObject::Dictionary(freedata.root))?;
    Ok(json)
}

fn jsonvalue_to_freedata_object(value: JsonValue) -> ComResult<FDataObject> {
    match value {
        JsonValue::String(string) => Ok(FDataObject::String(string.clone())),
        JsonValue::Number(num) if num.is_i64() => {
            Ok(FDataObject::Integer(num.as_i64().unwrap()))
        },
        JsonValue::Number(num) if num.is_f64() => {
            Ok(FDataObject::Float(num.as_f64().unwrap()))
        }
        JsonValue::Bool(b) => Ok(FDataObject::Boolean(b)),
        JsonValue::Array(array) => {
            let freedata_array: Box<[_]> = array
                .into_iter()
                .map(|json_val| jsonvalue_to_freedata_object(json_val))
                .collect::<ComResult<_>>()?;
            Ok(FDataObject::Array(freedata_array))
        }
        JsonValue::Object(map) => {
            let freedata_map: HashMap<_, _> = map
                .into_iter()
                .map(|(key, value)| {
                    Ok((key, jsonvalue_to_freedata_object(value)?))
                })
                .collect::<ComResult<_>>()?;
            Ok(FDataObject::Dictionary(freedata_map))
        }
        JsonValue::Null => Ok(FDataObject::Null),
        _ => Err(format!("couldn't recognize json value as freedata value").into())

    }
}

pub fn json_to_freedata(json: JsonValue) -> ComResult<FreeData> {
    if !json.is_object() {
        return Err(format!("json value must be object").into());
    }

    let FDataObject::Dictionary(root) = jsonvalue_to_freedata_object(json)? else {
        unreachable!();
    };

    Ok(FreeData { root })
}
