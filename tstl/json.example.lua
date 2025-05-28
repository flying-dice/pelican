--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]
local ____exports = {}
local ____pelican = require("pelican")
local json = ____pelican.json
local luaTable = {name = "John", age = 30, isActive = true}
local jsonString = json.encode(luaTable)
assert(jsonString == "{\"isActive\":true,\"name\":\"John\",\"age\":30}")
local decodedTable = json.decode(jsonString)
assert(decodedTable.name == "John")
return ____exports
