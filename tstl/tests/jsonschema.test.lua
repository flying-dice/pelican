--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]
local ____exports = {}
local ____pelican = require("pelican")
local jsonschema = ____pelican.jsonschema
local ____lester = require("tests.lester")
local describe = ____lester.describe
local expect = ____lester.expect
local it = ____lester.it
describe(
    "jsonschema",
    function()
        it(
            "should successfully instantiate using the new keyword",
            function()
                local validator = jsonschema.Validator.new({type = "string"})
                expect.equal(
                    tostring(validator),
                    "Validator({\"type\":\"string\"})"
                )
            end
        )
        it(
            "should successfully instantiate using the static new method",
            function()
                local validator = jsonschema.Validator.new({type = "string"})
                expect.equal(
                    tostring(validator),
                    "Validator({\"type\":\"string\"})"
                )
            end
        )
        it(
            "should validate a string against a string schema",
            function()
                local validator = jsonschema.Validator.new({type = "string"})
                local isValid, ____error = validator:validate("Hello World")
                expect.equal(isValid, true)
                expect.equal(____error, nil)
            end
        )
        it(
            "should validate a number against a number schema",
            function()
                local validator = jsonschema.Validator.new({type = "number"})
                local isValid, ____error = validator:validate(42)
                expect.equal(isValid, true)
                expect.equal(____error, nil)
            end
        )
        it(
            "should fail to validate a number against a string schema",
            function()
                local validator = jsonschema.Validator.new({type = "string"})
                local isValid, ____error = validator:validate(42)
                expect.equal(isValid, false)
                expect.equal(____error, "42 is not of type \"string\"")
            end
        )
        it(
            "should validate an object against an object schema",
            function()
                local validator = jsonschema.Validator.new({type = "object", properties = {name = {type = "string"}, age = {type = "number"}}, required = {"name", "age"}})
                local isValid, ____error = validator:validate({name = "John", age = 30})
                expect.equal(isValid, true)
                expect.equal(____error, nil)
            end
        )
        it(
            "should fail to compile an invalid schema",
            function()
                do
                    local function ____catch(e)
                        expect.equal((string.match(
                            tostring(e),
                            "([^\r\n]+)"
                        )), "\"invalidType\" is not valid under any of the schemas listed in the 'anyOf' keyword")
                    end
                    local ____try, ____hasReturned = pcall(function()
                        jsonschema.Validator.new({type = "invalidType"})
                        assert(false, "Expected an error to be thrown for invalid schema")
                    end)
                    if not ____try then
                        ____catch(____hasReturned)
                    end
                end
            end
        )
    end
)
return ____exports
