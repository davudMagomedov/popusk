use crate::comps_appearance::entitytype_to_string;
use crate::global_conf_directory::GlobalConfError;
use crate::types::LibEntity;
use crate::types::Progress;

use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::path::{Path, PathBuf};

use mlua::{
    Error as LuaError, FromLua, Function as LuaFunction, IntoLua, Lua, LuaOptions,
    Result as LuaResult, StdLib, Value as LuaValue,
};
use thiserror::Error;

const SCRIPTS_FILE_NAME: &str = "scripts.lua";

const LOOK_SCRIPT_FUNCTION_NAME: &str = "look_output";
const OPEN_SCRIPT_FUNCTION_NAME: &str = "open_libentity";
const LIST_NARROW_SCRIPT_FUNCTION_NAME: &str = "list_output_narrow";
const LIST_WIDE_SCRIPT_FUNCTION_NAME: &str = "list_output_wide";
const IS_DOCUMENT_SCRIPT_FUNCTION_NAME: &str = "is_document";

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

impl IntoLua for Progress {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let progress_table = lua.create_table()?;

        progress_table.set("passed", self.passed())?;
        progress_table.set("ceiling", self.ceiling())?;

        Ok(LuaValue::Table(progress_table))
    }
}

impl FromLua for Progress {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        let LuaValue::Table(table) = value else {
            return Err(LuaError::UserDataTypeMismatch);
        };

        let passed = table.get::<usize>("passed")?;
        let ceiling = table.get::<usize>("ceiling")?;

        Ok(Progress::with_passed(passed, ceiling))
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
        let look_output_func = self
            .lua
            .globals()
            .get::<LuaFunction>(LOOK_SCRIPT_FUNCTION_NAME)?;
        match look_output_func.call::<String>((libentity, context)) {
            Ok(string) => Ok(string),
            Err(LuaError::RuntimeError(runtime_err_msg)) => {
                return Err(ScriptsError::LuaRuntimeError(runtime_err_msg))
            }
            Err(lua_error) => return Err(lua_error.into()),
        }
    }

    pub fn open_libentity(
        &self,
        libentity: LibEntity,
        context: Context,
    ) -> Result<Progress, ScriptsError> {
        let open_libentity_func = self
            .lua
            .globals()
            .get::<LuaFunction>(OPEN_SCRIPT_FUNCTION_NAME)?;
        match open_libentity_func.call::<Progress>((libentity, context)) {
            Ok(progress) => Ok(progress),
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
            .get::<LuaFunction>(LIST_NARROW_SCRIPT_FUNCTION_NAME)?;
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
        let list_output_wide_func = self
            .lua
            .globals()
            .get::<LuaFunction>(LIST_WIDE_SCRIPT_FUNCTION_NAME)?;
        match list_output_wide_func.call::<String>((libentities, context)) {
            Ok(string) => Ok(string),
            Err(LuaError::RuntimeError(runtime_err_msg)) => {
                return Err(ScriptsError::LuaRuntimeError(runtime_err_msg))
            }
            Err(lua_error) => return Err(lua_error.into()),
        }
    }

    pub fn is_document(&self, extension: String) -> Result<bool, ScriptsError> {
        let is_document_func = self
            .lua
            .globals()
            .get::<LuaFunction>(IS_DOCUMENT_SCRIPT_FUNCTION_NAME)?;

        match is_document_func.call::<bool>(extension) {
            Ok(is_document) => Ok(is_document),
            Err(LuaError::RuntimeError(runtime_err_msg)) => {
                return Err(ScriptsError::LuaRuntimeError(runtime_err_msg))
            }
            Err(lua_error) => return Err(lua_error.into()),
        }
    }
}

/// alias to `open_scripts_with_directory(&directory.join(SCRIPTS_FILE_NAME))`
pub fn open_scripts_from_directory(directory: &Path) -> Result<Scripts, ScriptsError> {
    open_scripts_from_file(&directory.join(SCRIPTS_FILE_NAME))
}

pub fn open_scripts_from_file(scriptfile: &Path) -> Result<Scripts, ScriptsError> {
    let lua = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::new())?;
    let lua_file_content = match std::fs::read_to_string(scriptfile) {
        Ok(lfc) => lfc,
        Err(io_error) if io_error.kind() == IoErrorKind::NotFound => {
            return Err(ScriptsError::ScriptsFileWasNotFound(scriptfile.to_owned()));
        }
        Err(io_error) => return Err(ScriptsError::IOErrorWithScriptsFile(io_error)),
    };
    lua.load(lua_file_content).exec()?;

    Ok(Scripts { lua })
}
