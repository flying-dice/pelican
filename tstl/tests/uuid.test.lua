--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]
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
