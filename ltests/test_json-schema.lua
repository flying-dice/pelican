package.path = package.path .. ";.\\ltests\\?.lua"
package.cpath = package.cpath .. ";.\\target\\debug\\?.dll"

---@type pelican
local P = require("pelican")
local lester = require('lester')

local describe, it, expect = lester.describe, lester.it, lester.expect

describe("json-schema.validator:validate", function()
    local validator = P.jsonschema.validator_for({
        type = "array",
        minItems = 2,
        maxItems = 2,
        items = {
            type = "number"
        }
    })

    it("should validate a successful input", function()
        local res, err = validator:validate({ 1, 2 })
        expect.equal(err, nil)
        expect.equal(res, true)
    end)

    it("should fail with too few items", function()
        local res, err = validator:validate({ 1 })
        expect.equal(err, "[1] has less than 2 items")
        expect.equal(res, nil)
    end)
end)

lester.report()