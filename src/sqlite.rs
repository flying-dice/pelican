use log::debug;
use mlua::prelude::{LuaResult, LuaTable, LuaValue};
use mlua::{ExternalError, IntoLuaMulti, Lua, Result, UserData, UserDataMethods, Value};
use sqlite::State;
use sqlite::{Connection, Statement};

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
    fn new(path: String) -> LuaResult<_SqliteConnection> {
        let connection = sqlite::open(path).map_err(|e| {
            debug!("Failed to open SQLite connection: {}", e);
            format!("SQLite error: {}", e).into_lua_err()
        })?;
        Ok(Self { connection })
    }
}

impl UserData for _SqliteConnection {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("exec", |lua, this, query: String| {
            match this.connection.execute(query) {
                Ok(_) => true.into_lua_multi(lua),
                Err(e) => {
                    let error_message = format!("SQLite error: {}", e);
                    debug!("{}", error_message);
                    (LuaValue::Nil, error_message).into_lua_multi(lua)
                }
            }
        });

        methods.add_method(
            "execute",
            |lua, this, (query, params): (String, Option<LuaTable>)| {
                debug!("Executing query: {}", query);
                let mut statement = match prepare_statement(&this.connection, query, params) {
                    Ok(statement) => {
                        debug!("Statement prepared successfully");
                        statement
                    }
                    Err(e) => {
                        debug!("Failed to prepare statement: {}", e);
                        return (LuaValue::Nil, e.to_string()).into_lua_multi(lua);
                    }
                };
                let result_table = match execute_and_map_result(lua, &mut statement) {
                    Ok(table) => table,
                    Err(e) => {
                        debug!("Failed to execute statement: {}", e);
                        return (LuaValue::Nil, e.to_string()).into_lua_multi(lua);
                    }
                };
                result_table.into_lua_multi(lua)
            },
        );
    }
}
fn open(_: &Lua, path: String) -> Result<_SqliteConnection> {
    _SqliteConnection::new(path)
}

/**
 * Prepare a statement for execution.
 * This function takes a connection, a query string, and an optional Lua table of parameters.
 * It returns a prepared statement or an error.
 */
fn prepare_statement(
    conn: &Connection,
    query: String,
    params: Option<LuaTable>,
) -> Result<Statement> {
    debug!("Preparing query: {}", query);
    let mut statement = conn.prepare(query).map_err(|e| {
        debug!("Failed to prepare statement: {}", e);
        format!("SQLite error: {}", e).into_lua_err()
    })?;

    debug!("Binding parameters: {:?}", params);
    if let Some(params) = params {
        bind_params(&mut statement, params)?;
    }

    Ok(statement)
}

/**
 * Bind parameters to a statement from a Lua table.
 * This function checks if the table is an array or a named parameter table.
 */
fn bind_params(statement: &mut Statement, params: LuaTable) -> Result<()> {
    if is_lua_array(&params)? {
        bind_array_params(statement, params)
    } else {
        bind_named_params(statement, params)
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

/**
 * Bind parameters to a statement from a Lua table.
 * This function assumes that the table contains integer keys starting from 1.
 */
fn bind_array_params(statement: &mut Statement, params: LuaTable) -> Result<()> {
    for entry in params.pairs::<usize, LuaValue>() {
        match entry {
            Ok((index, param)) => {
                debug!("Binding parameter {} to {:?}", index, param);
                let sqlite_value = to_bindable_value(param)?;

                debug!("Bound parameter {} to {:?}", index, sqlite_value);
                statement
                    .bind((index, sqlite_value))
                    .map_err(|e| format!("SQLite error: {}", e).into_lua_err())?;
            }
            Err(e) => {
                debug!("Failed to bind parameter: {}", e);
                return Err(format!("Failed to bind parameter: {}", e).into_lua_err());
            }
        }
    }
    Ok(())
}

/**
 * Bind parameters to a statement from a Lua table.
 * This function assumes that the table contains string keys.
 */
fn bind_named_params(statement: &mut Statement, params: LuaTable) -> Result<()> {
    for entry in params.pairs::<String, LuaValue>() {
        match entry {
            Ok((key, param)) => {
                debug!("Binding parameter {} to {:?}", key, param);
                let sqlite_value = to_bindable_value(param)?;

                debug!("Bound parameter {} to {:?}", key, sqlite_value);
                statement
                    .bind((format!(":{}", key).as_str(), sqlite_value))
                    .map_err(|e| format!("SQLite error: {}", e).into_lua_err())?;
            }
            Err(e) => {
                debug!("Failed to bind parameter: {}", e);
                return Err(format!("Failed to bind parameter: {}", e).into_lua_err());
            }
        }
    }
    Ok(())
}

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
            Err("Unsupported parameter type".to_string().into_lua_err())
        }
    }
}

/**
 * Execute a query and return the result as a Lua table.
 * This function takes a Lua state, a statement, and returns a Lua table.
 */
fn execute_and_map_result(lua: &Lua, statement: &mut Statement) -> Result<LuaTable> {
    let result_table = lua.create_table().expect("Failed to create result table");
    let mut row_index = 1;

    debug!("Executing statement");

    while let Ok(State::Row) = statement.next() {
        debug!("Row {}", row_index);
        let row_table = lua.create_table()?;
        for (i, col_name) in statement.column_names().iter().enumerate() {
            let value = match statement.read(i).expect("Failed to read column") {
                sqlite::Value::Integer(v) => Value::Integer(v),
                sqlite::Value::Float(v) => Value::Number(v),
                sqlite::Value::String(v) => Value::String(lua.create_string(&v)?),
                sqlite::Value::Binary(v) => Value::String(lua.create_string(&v)?),
                sqlite::Value::Null => LuaValue::Nil,
            };
            row_table.set(col_name.as_str(), value)?;
        }

        debug!("Row data: {:?}", row_table);

        result_table.set(row_index, row_table)?;
        row_index += 1;
    }

    debug!("Statement executed successfully");

    Ok(result_table)
}
