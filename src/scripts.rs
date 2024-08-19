use crate::comps_appearance::entitytype_to_string;
use crate::entity_base::EntityBase;
use crate::global_conf_directory::{configdir, GlobalConfError};
use crate::progress::Progress;

use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::path::PathBuf;

use mlua::{
    Error as LuaError, Function as LuaFunction, IntoLua, Lua, LuaOptions, Result as LuaResult,
    StdLib, Value as LuaValue,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScriptsError {
    #[error("global conf: {0}")]
    GlobalConfError(#[from] GlobalConfError),
    #[error("couldn't find scripts file in the '{0}' path")]
    ScriptsFileWasNotFound(PathBuf),
    #[error("lua runtime: {0}")]
    LuaRuntimeError(String),
    #[error("lua error: {0}")]
    LuaError(#[from] LuaError),
    #[error("io error with scripts file: {0}")]
    IOErrorWithScriptsFile(IoError),
}

#[derive(Debug, Clone)]
pub struct Context {
    tshape_w: u16,
    tshape_h: u16,
}

impl Context {
    pub fn new(tshape_w: u16, tshape_h: u16) -> Self {
        Context { tshape_w, tshape_h }
    }

    pub fn auto() -> Option<Self> {
        let (w, h) = term_size::dimensions()?;
        Some(Context {
            tshape_w: w as u16,
            tshape_h: h as u16,
        })
    }
}

impl IntoLua for Context {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let context_table = lua.create_table()?;

        context_table.set("tshape_w", self.tshape_w)?;
        context_table.set("tshape_h", self.tshape_h)?;

        Ok(LuaValue::Table(context_table))
    }
}

#[derive(Debug, Clone)]
pub struct LibEntity {
    path: PathBuf,
    base: EntityBase,
    progress: Option<Progress>,
}

impl LibEntity {
    pub fn new(path: PathBuf, base: EntityBase, progress: Option<Progress>) -> Self {
        LibEntity {
            path,
            base,
            progress,
        }
    }
}

impl IntoLua for LibEntity {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let libentity_table = lua.create_table()?;

        libentity_table.set("path", self.path.to_string_lossy())?;
        libentity_table.set("id", self.base.id().to_string())?;
        libentity_table.set("name", self.base.name().clone())?;
        libentity_table.set("tags", self.base.tags().clone())?;
        libentity_table.set("etype", entitytype_to_string(self.base.etype()))?;

        if let Some(progress) = self.progress {
            let progress_table = lua.create_table()?;

            progress_table.set("passed", progress.passed())?;
            progress_table.set("ceiling", progress.ceiling())?;

            libentity_table.set("progress", progress_table)?;
        }

        Ok(LuaValue::Table(libentity_table))
    }
}

#[derive(Debug)]
// INVARIATNS:
// - `lua` field is used only for getting variables.
pub struct Scripts {
    lua: Lua,
}

impl Scripts {
    pub fn look_output(
        &self,
        libentity: LibEntity,
        context: Context,
    ) -> Result<String, ScriptsError> {
        let look_output_func = self.lua.globals().get::<LuaFunction>("look_output")?;
        match look_output_func.call::<String>((libentity, context)) {
            Ok(string) => Ok(string),
            Err(LuaError::RuntimeError(runtime_err_msg)) => {
                return Err(ScriptsError::LuaRuntimeError(runtime_err_msg))
            }
            Err(lua_error) => return Err(lua_error.into()),
        }
    }

    pub fn list_output_narrow(
        &self,
        libentities: Vec<LibEntity>,
        context: Context,
    ) -> Result<String, ScriptsError> {
        let list_output_narrow_func = self
            .lua
            .globals()
            .get::<LuaFunction>("list_output_narrow")?;
        match list_output_narrow_func.call::<String>((libentities, context)) {
            Ok(string) => Ok(string),
            Err(LuaError::RuntimeError(runtime_err_msg)) => {
                return Err(ScriptsError::LuaRuntimeError(runtime_err_msg))
            }
            Err(lua_error) => return Err(lua_error.into()),
        }
    }

    pub fn list_output_wide(
        &self,
        libentities: Vec<LibEntity>,
        context: Context,
    ) -> Result<String, ScriptsError> {
        let list_output_wide_func = self.lua.globals().get::<LuaFunction>("list_output_wide")?;
        match list_output_wide_func.call::<String>((libentities, context)) {
            Ok(string) => Ok(string),
            Err(LuaError::RuntimeError(runtime_err_msg)) => {
                return Err(ScriptsError::LuaRuntimeError(runtime_err_msg))
            }
            Err(lua_error) => return Err(lua_error.into()),
        }
    }
}

fn scriptfile() -> Result<PathBuf, ScriptsError> {
    Ok(configdir()?.join("scripts.lua"))
}

pub fn open_scripts() -> Result<Scripts, ScriptsError> {
    let lua = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::new())?;

    let scriptfile = scriptfile()?;
    let lua_file_content = match std::fs::read_to_string(&scriptfile) {
        Ok(lfc) => lfc,
        Err(io_error) if io_error.kind() == IoErrorKind::NotFound => {
            return Err(ScriptsError::ScriptsFileWasNotFound(scriptfile));
        }
        Err(io_error) => return Err(ScriptsError::IOErrorWithScriptsFile(io_error)),
    };

    lua.load(lua_file_content).exec()?;

    Ok(Scripts { lua })
}
