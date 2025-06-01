local ____lualib = require("lualib_bundle")
local __TS__SourceMapTraceBack = ____lualib.__TS__SourceMapTraceBack
__TS__SourceMapTraceBack(debug.getinfo(1).short_src, {["5"] = 1,["6"] = 1,["7"] = 2,["8"] = 2,["9"] = 2,["10"] = 2,["11"] = 4,["12"] = 4,["13"] = 4,["14"] = 5,["15"] = 5,["16"] = 5,["17"] = 6,["18"] = 7,["19"] = 9,["20"] = 9,["21"] = 9,["22"] = 9,["23"] = 10,["24"] = 5,["25"] = 5,["26"] = 13,["27"] = 13,["28"] = 13,["29"] = 14,["30"] = 14,["31"] = 14,["32"] = 14,["33"] = 14,["34"] = 14,["35"] = 14,["36"] = 15,["37"] = 17,["38"] = 17,["39"] = 17,["40"] = 17,["41"] = 18,["42"] = 13,["43"] = 13,["44"] = 21,["45"] = 21,["46"] = 21,["47"] = 22,["48"] = 23,["49"] = 25,["50"] = 25,["51"] = 25,["52"] = 25,["53"] = 26,["54"] = 21,["55"] = 21,["56"] = 29,["57"] = 29,["58"] = 29,["59"] = 30,["60"] = 31,["61"] = 33,["62"] = 33,["63"] = 33,["64"] = 33,["65"] = 34,["66"] = 29,["67"] = 29,["68"] = 37,["69"] = 37,["70"] = 37,["71"] = 38,["72"] = 39,["73"] = 41,["74"] = 41,["75"] = 41,["76"] = 41,["77"] = 42,["78"] = 37,["79"] = 37,["80"] = 45,["81"] = 45,["82"] = 45,["83"] = 46,["84"] = 47,["85"] = 49,["86"] = 49,["87"] = 49,["88"] = 49,["89"] = 50,["90"] = 45,["91"] = 45,["92"] = 53,["93"] = 53,["94"] = 53,["95"] = 54,["96"] = 55,["97"] = 57,["98"] = 57,["99"] = 57,["100"] = 57,["101"] = 58,["102"] = 59,["103"] = 60,["104"] = 53,["105"] = 53,["106"] = 4,["107"] = 4});
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
    end
)
return ____exports
