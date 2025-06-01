local ____lualib = require("lualib_bundle")
local __TS__SourceMapTraceBack = ____lualib.__TS__SourceMapTraceBack
__TS__SourceMapTraceBack(debug.getinfo(1).short_src, {["5"] = 1,["6"] = 1,["7"] = 2,["8"] = 2,["9"] = 2,["10"] = 2,["11"] = 4,["12"] = 4,["13"] = 4,["14"] = 5,["15"] = 5,["16"] = 5,["17"] = 6,["18"] = 7,["19"] = 9,["20"] = 9,["21"] = 9,["22"] = 9,["23"] = 10,["24"] = 10,["25"] = 10,["26"] = 10,["27"] = 5,["28"] = 5,["29"] = 13,["30"] = 13,["31"] = 13,["32"] = 14,["33"] = 15,["34"] = 17,["35"] = 17,["36"] = 17,["37"] = 17,["38"] = 18,["39"] = 18,["40"] = 18,["41"] = 18,["42"] = 13,["43"] = 13,["44"] = 4,["45"] = 4});
local ____exports = {}
local ____pelican = require("pelican")
local uuid = ____pelican.uuid
local ____lester = require("tests.lester")
local describe = ____lester.describe
local expect = ____lester.expect
local it = ____lester.it
describe(
    "uuid",
    function()
        it(
            "should produce a valid uuid V4",
            function()
                local uuidv4 = uuid.v4()
                print("Generated uuid v4: " .. uuidv4)
                expect.equal(
                    type(uuidv4),
                    "string"
                )
                expect.equal(
                    string.len(uuidv4),
                    36
                )
            end
        )
        it(
            "should produce a valid uuid V7",
            function()
                local uuidv7 = uuid.v7()
                print("Generated uuid v7: " .. uuidv7)
                expect.equal(
                    type(uuidv7),
                    "string"
                )
                expect.equal(
                    string.len(uuidv7),
                    36
                )
            end
        )
    end
)
return ____exports
