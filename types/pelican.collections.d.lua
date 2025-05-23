---@meta

---@class pelican.collections
local collections = {}

---@class pelican.collections.Vec
local Vec = {}

---Creates a new Vec, optionally initialized with values from a Lua table.
---@param initial? table
---@return pelican.collections.Vec
function collections.Vec(initial) end

---Returns the number of elements in the Vec.
---@return number
function Vec:len() end

---Returns the value at the given zero-based index, or nil if out of bounds.
---@param index number
---@return any
function Vec:get(index) end

---Sets the value at the given zero-based index.
---@param index number
---@param value any
function Vec:set(index, value) end

---Appends a value to the end of the Vec.
---@param value any
function Vec:push(value) end

---Removes and returns the last element, or nil if empty.
---@return any
function Vec:pop() end

---Removes all elements from the Vec.
function Vec:clear() end

---Calls func(value) for each element in the Vec.
---@param func fun(value: any)
function Vec:for_each(func) end

---Returns a new Vec with each element mapped by func(value).
---@param func fun(value: any): any
---@return pelican.collections.Vec
function Vec:map(func) end

---Returns a new Vec containing only elements where func(value) returns true.
---@param func fun(value: any): boolean
---@return pelican.collections.Vec
function Vec:filter(func) end

---Reduces the Vec to a single value by applying func(acc, value) for each element.
---@param func fun(acc: any, value: any): any
---@param accumulator any
---@return any
function Vec:reduce(func, accumulator) end

---Returns a new Vec with the elements in reverse order.
---@return pelican.collections.Vec
function Vec:reverse() end

---Converts the Vec to a Lua table (1-based array).
---@return table
function Vec:to_lua_table() end

collections.Vec = Vec

return collections