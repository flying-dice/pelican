# Pelican Documentation Index

Pelican provides a set of Lua modules backed by Rust for high-performance scripting, data handling, and web integration.
The following modules are available:

---

## Modules

### `collections`

Rust-style collections with methods for manipulation and iteration.

See [collections.md](collections.md) for more details.

### `json`

Encode Lua tables to JSON and decode JSON strings to Lua tables.

### `json_schema`

Validate Lua tables against JSON schemas.

### `logger`

Logging utilities with configurable output.

### `requests`

Perform HTTP requests (GET, POST, PUT, DELETE) and handle responses.

### `sqlite`

Interact with SQLite databases (execute queries, bind parameters).

### `uuid`

Generate UUID version 4 strings.

### `web`

Start HTTP/WebSocket servers, define routes, and handle RPC.

---

## Tests

There are several test files located in the `ltests/` directory, each corresponding to a module. The tests are designed
to validate the functionality of the modules and ensure they work as expected.

Reference the tests for examples of how to use the modules and their functions.

- **Test Framework:** `lester` (see `ltests/lester.lua`)
- **Test Suites:** Located in `ltests/` (e.g., `test_json.lua`, `test_sqlite.lua`, etc.)

---

## Type Definitions

- **Lua types:** See `types/` for `.d.lua` files describing module APIs.