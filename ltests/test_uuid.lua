package.path = package.path .. ";.\\ltests\\?.lua"
package.cpath = package.cpath .. ";.\\target\\debug\\?.dll"

---@type pelican
local P = require("pelican")
local lester = require('lester')

local describe, it, expect = lester.describe, lester.it, lester.expect

describe("uuid.v4", function()
    it("should succeed with valid UUID", function()
        local res, err = P.uuid.v4()
        expect.equal(type(res), "string")
        expect.equal(#res, 36)
    end)
end)

lester.report()