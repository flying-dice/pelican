# Collections Module

The `collections` module provides a set of data structures for use in Lua, with a focus on mimicking Rust like
collections.

---

## Usage

First, require the `pelican` module and access `collections`:

```lua
local P = require("pelican")
local Vec = P.collections.Vec
```

---

## Vec

A dynamic array supporting fast push, pop, indexing, and iteration.

### Creating a Vec

```lua
local v = Vec({ 1, 2, 3 }) -- from a Lua table
local empty = Vec()      -- empty Vec
```

---

### Methods

#### `Vec:len()`

Returns the number of elements.

```lua
local n = v:len()
```

#### `Vec:get(index)`

Returns the value at the given zero-based index, or `nil` if out of bounds.

```lua
local value = v:get(0) -- first element
```

#### `Vec:set(index, value)`

Sets the value at the given zero-based index.

```lua
v:set(1, "hello")
```

#### `Vec:push(value)`

Appends a value to the end.

```lua
v:push(42)
```

#### `Vec:pop()`

Removes and returns the last element, or `nil` if empty.

```lua
local last = v:pop()
```

#### `Vec:clear()`

Removes all elements.

```lua
v:clear()
```

#### `Vec:reverse()`

Returns a new Vec with the elements in reverse order.

```lua
local rev = v:reverse()
```

#### `Vec:for_each(func)`

Calls `func(value)` for each element.

```lua
v:for_each(function(x)
    print(x)
end)
```

#### `Vec:map(func)`

Returns a new Vec with each element mapped by `func(value)`.

```lua
local doubled = v:map(function(x)
    return x * 2 -- double each element
end)
```

#### `Vec:filter(func)`

Returns a new Vec containing only elements where `func(value)` returns true.

```lua
local evens = v:filter(function(x)
    return x % 2 == 0 -- filter even numbers
end)
```

#### `Vec:reduce(func, accumulator)`

Reduces the Vec to a single value by applying `func(acc, value)` for each element.

```lua
local sum = v:reduce(function(acc, x)
    return acc + x -- sum all elements
end, 0)
```

#### `Vec:to_lua_table()`

Converts the Vec to a Lua table (1-based array).

```lua
local t = v:to_lua_table()
```

---

## Tests

See [test_collections.lua](../ltests/test_collections.lua) for tests with examples.