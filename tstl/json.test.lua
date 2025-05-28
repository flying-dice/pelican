--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]
local ____exports = {}
local ____pelican = require("pelican")
local json = ____pelican.json
local ____lester = require("lester")
local describe = ____lester.describe
local expect = ____lester.expect
local it = ____lester.it
describe(
    "json",
    function()
        it(
            "should successfully encode an object",
            function()
                local luaTable = {isActive = true, name = "John", age = 30}
                local jsonString = json.encode(luaTable)
                expect.equal(
                    type(jsonString),
                    "string"
                )
                expect.equal(jsonString, "{\"age\":30,\"isActive\":true,\"name\":\"John\"}")
            end
        )
        it(
            "should should successfully encode an array",
            function()
                local luaArray = {
                    "a",
                    "b",
                    "c",
                    "d",
                    "e"
                }
                local jsonString = json.encode(luaArray)
                expect.equal(
                    type(jsonString),
                    "string"
                )
                expect.equal(jsonString, "[\"a\",\"b\",\"c\",\"d\",\"e\"]")
            end
        )
        it(
            "should successfully encode a string",
            function()
                local luaString = "Hello World"
                local jsonString = json.encode(luaString)
                expect.equal(
                    type(jsonString),
                    "string"
                )
                expect.equal(jsonString, "\"Hello World\"")
            end
        )
        it(
            "should successfully encode an integer",
            function()
                local luaInteger = 42
                local jsonString = json.encode(luaInteger)
                expect.equal(
                    type(jsonString),
                    "string"
                )
                expect.equal(jsonString, "42")
            end
        )
        it(
            "should successfully encode a boolean",
            function()
                local luaBoolean = true
                local jsonString = json.encode(luaBoolean)
                expect.equal(
                    type(jsonString),
                    "string"
                )
                expect.equal(jsonString, "true")
            end
        )
        it(
            "should successfully encode a nil value",
            function()
                local luaNil = nil
                local jsonString = json.encode(luaNil)
                expect.equal(
                    type(jsonString),
                    "string"
                )
                expect.equal(jsonString, "null")
            end
        )
        it(
            "should successfully decode a JSON string to a Lua table",
            function()
                local jsonString = "{\"name\":\"John\",\"age\":30,\"isActive\":true}"
                local luaTable = json.decode(jsonString)
                expect.equal(
                    type(luaTable),
                    "table"
                )
                expect.equal(luaTable.name, "John")
                expect.equal(luaTable.age, 30)
                expect.equal(luaTable.isActive, true)
            end
        )
    end
)
return ____exports
