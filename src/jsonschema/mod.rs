use mlua::prelude::{LuaResult, LuaTable};
use mlua::Lua;
use validator::_Validator;

mod validator;

pub fn inject_module(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
    let m = lua.create_table()?;

    m.set("Validator", lua.create_proxy::<_Validator>()?)?;

    table.set("jsonschema", m)?;
    Ok(())
}
