# JSON Module

The `json` module provides functions for encoding Lua tables to JSON and decoding JSON strings back to Lua tables.

---

## Usage

First, require the `pelican` module and access `json`:

```lua
local P = require("pelican")
local json = P.json
```

---

### Functions

#### `json.encode()`

Encodes a Lua table to a JSON string.

```lua
local json_string = json.encode({ key = "value", array = { 1, 2, 3 } })
print(json_string) -- {"key":"value","array":[1,2,3]}
```

#### `json.decode()`

Decodes a JSON string to a Lua table.

```lua
local lua_table = json.decode('{"key":"value","array":[1,2,3]}')
print(lua_table.key) -- "value"
```

---

## Tests

See [test_json.lua](https://github.com/flying-dice/pelican/tree/main/ltests/test_json.lua) for tests with examples.