--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]
local ____exports = {}
local ____pelican = require("pelican")
local json = ____pelican.json
local ____lester = require("tests.lester")
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
        it(
            "should return err for malfored json",
            function()
                local jsonString = "{\"name\": \"John\", \"age\": 30, \"isActive\": true"
                local luaTable, err = json.decode(jsonString)
                expect.equal(luaTable, nil)
                expect.equal(err, "EOF while parsing an object at line 1 column 44")
            end
        )
        describe(
            "safe_encode",
            function()
                it(
                    "should safe encode a string",
                    function()
                        local luaString = "Hello World"
                        local jsonString, err = json.safe_encode(luaString)
                        expect.equal(
                            type(jsonString),
                            "string"
                        )
                        expect.equal(jsonString, "\"Hello World\"")
                    end
                )
                it(
                    "should safe encode a number",
                    function()
                        local luaNumber = 42
                        local jsonString, err = json.safe_encode(luaNumber)
                        expect.equal(
                            type(jsonString),
                            "string"
                        )
                        expect.equal(jsonString, "42")
                    end
                )
                it(
                    "should safe encode a boolean",
                    function()
                        local luaBoolean = true
                        local jsonString, err = json.safe_encode(luaBoolean)
                        expect.equal(
                            type(jsonString),
                            "string"
                        )
                        expect.equal(jsonString, "true")
                    end
                )
                it(
                    "should safe encode a nil value",
                    function()
                        local luaNil = nil
                        local jsonString, err = json.safe_encode(luaNil)
                        expect.equal(
                            type(jsonString),
                            "string"
                        )
                        expect.equal(jsonString, "null")
                    end
                )
                it(
                    "should safe encode an array with all strings",
                    function()
                        local luaArray = {
                            "a",
                            "b",
                            "c",
                            "d",
                            "e"
                        }
                        local jsonString, err = json.safe_encode(luaArray)
                        expect.equal(
                            type(jsonString),
                            "string"
                        )
                        expect.equal(jsonString, "[\"a\",\"b\",\"c\",\"d\",\"e\"]")
                    end
                )
                it(
                    "should safe encode an array with mixed types",
                    function()
                        local luaArray = {"a", 1, true}
                        local jsonString, err = json.safe_encode(luaArray)
                        expect.equal(
                            type(jsonString),
                            "string"
                        )
                        expect.equal(jsonString, "[\"a\",1,true]")
                    end
                )
                it(
                    "should safe encode a table with a function",
                    function()
                        local luaTable = {
                            name = "John",
                            greet = function(self)
                                return "Hello, " .. self.name
                            end
                        }
                        local jsonString, err = json.safe_encode(luaTable)
                        print(err)
                        expect.equal(
                            type(jsonString),
                            "string"
                        )
                        expect.equal(jsonString, "{\"greet\":\"function\",\"name\":\"John\"}")
                    end
                )
                it(
                    "should safe encode a table with a thread",
                    function()
                        local coro = coroutine.create(function()
                        end)
                        local luaTable = {name = "John", coro = coro}
                        local jsonString, err = json.safe_encode(luaTable)
                        print(err)
                        expect.equal(
                            type(jsonString),
                            "string"
                        )
                        expect.equal(jsonString, "{\"coro\":\"thread\",\"name\":\"John\"}")
                    end
                )
            end
        )
    end
)
return ____exports
