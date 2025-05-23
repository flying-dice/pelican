---@meta

---@class pelican.json
local json = {}

---Encodes a Lua value to a JSON string.
---@param value any
---@return string|nil, nil|string
function json.encode(value) end

---Decodes a JSON string to a Lua value.
---@param str string
---@return any|nil, nil|string
function json.decode(str) end

return json
