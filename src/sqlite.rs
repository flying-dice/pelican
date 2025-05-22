use log::{debug, info};
use mlua::prelude::{LuaError, LuaTable, LuaValue};
use mlua::{IntoLuaMulti, Lua, Result, UserData, UserDataMethods, Value};
use sqlite::Connection;
use sqlite::State;

/**
* Convert a Lua value to an SQLite value for binding.
*/
fn to_bindable_value(value: LuaValue) -> Result<sqlite::Value> {
    match value {
        LuaValue::Nil => Ok(sqlite::Value::Null),
        LuaValue::Integer(v) => Ok(sqlite::Value::Integer(v)),
        LuaValue::Number(v) => Ok(sqlite::Value::Float(v)),
        LuaValue::String(v) => Ok(sqlite::Value::String(v.to_str()?.to_string())),
        _ => {
            debug!("Unsupported parameter type: {:?}", value);
            Err(LuaError::RuntimeError(
                "Unsupported parameter type".to_string(),
            ))
        }
    }
}

/**
 * Check if a Lua table is an array by checking if it has contiguous integer keys starting from 1.
 */
fn is_lua_array(table: &LuaTable) -> Result<bool> {
    let mut last_index = 0;

    for pair in table.pairs::<LuaValue, LuaValue>() {
        let (key, _) = pair?;

        let index = match key {
            LuaValue::Integer(i) if i > 0 => i,
            _ => return Ok(false), // Non-integer or non-positive index
        };

        if index != last_index + 1 {
            return Ok(false); // Not contiguous
        }

        last_index = index;
    }

    Ok(true)
}

pub fn inject_module(lua: &Lua, table: &LuaTable) -> Result<()> {
    let m = lua.create_table()?;

    m.set("open", lua.create_function(open)?)?;

    table.set("sqlite", m)?;

    Ok(())
}

struct _SqliteConnection {
    connection: Connection,
}

impl _SqliteConnection {
    fn new(path: String) -> Result<_SqliteConnection> {
        let connection = sqlite::open(path).expect("Failed to open SQLite database");
        Ok(Self { connection })
    }
}

impl UserData for _SqliteConnection {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("execute", |lua, this, query: String| {
            match this.connection.execute(query) {
                Ok(_) => true.into_lua_multi(lua),
                Err(e) => {
                    let error_message = format!("SQLite error: {}", e);
                    debug!("{}", error_message);
                    (false, error_message).into_lua_multi(lua)
                }
            }
        });

        methods.add_method("query", |lua, this, (query, params): (String, LuaTable)| {
            debug!("Preparing query: {}", query);
            let mut statement = this
                .connection
                .prepare(query)
                .expect("Failed to prepare query");

            if is_lua_array(&params).expect("Failed to check if params is an array") {
                debug!("Binding parameters as array");
                for entry in params.pairs::<usize, LuaValue>() {
                    match entry {
                        Ok((index, param)) => {
                            debug!("Binding parameter {} to {:?}", index, param);
                            let sqlite_value =
                                to_bindable_value(param).expect("Failed to bind parameter");

                            debug!("Bound parameter {} to {:?}", index, sqlite_value);
                            statement
                                .bind((index, sqlite_value))
                                .expect("Failed to bind parameter");
                        }
                        Err(e) => {
                            debug!("Failed to bind parameter: {}", e);
                            return (false, format!("Failed to bind parameter: {}", e))
                                .into_lua_multi(lua);
                        }
                    }
                }
            } else {
                debug!("Binding parameters as table");
                for entry in params.pairs::<String, LuaValue>() {
                    match entry {
                        Ok((key, param)) => {
                            debug!("Binding parameter {} to {:?}", key, param);
                            let sqlite_value =
                                to_bindable_value(param).expect("Failed to bind parameter");

                            debug!("Bound parameter {} to {:?}", key, sqlite_value);
                            statement
                                .bind((format!(":{}", key).as_str(), sqlite_value))
                                .expect("Failed to bind parameter");
                        }
                        Err(e) => {
                            debug!("Failed to bind parameter: {}", e);
                            return (false, format!("Failed to bind parameter: {}", e))
                                .into_lua_multi(lua);
                        }
                    }
                }
            }

            let result_table = lua.create_table().expect("Failed to create result table");
            let mut row_index = 1;

            while let Ok(State::Row) = statement.next() {
                let row_table = lua.create_table()?;
                for (i, col_name) in statement.column_names().iter().enumerate() {
                    let value = match statement.read(i).expect("Failed to read column") {
                        sqlite::Value::Integer(v) => Value::Integer(v),
                        sqlite::Value::Float(v) => Value::Number(v),
                        sqlite::Value::String(v) => Value::String(lua.create_string(&v)?),
                        sqlite::Value::Binary(v) => Value::String(lua.create_string(&v)?),
                        sqlite::Value::Null => Value::Nil,
                    };
                    row_table.set(col_name.as_str(), value)?;
                }

                result_table.set(row_index, row_table)?;
                row_index += 1;
            }

            result_table.into_lua_multi(lua)
        });
    }
}
fn open(_: &Lua, path: String) -> Result<_SqliteConnection> {
    _SqliteConnection::new(path)
}
