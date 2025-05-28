package.path = package.path .. ";.\\ltests\\?.lua"
package.cpath = package.cpath .. ";.\\target\\debug\\?.dll"

---@type pelican
local P = require("pelican")
local lester = require('lester')

local describe, it, expect = lester.describe, lester.it, lester.expect

describe("logger.Logger", function()
    it("should construct a namespaced logger", function()
        local logger = P.logger.Logger("TEST.NAMESPACE")
        logger:info("This is a test message")
    end)

    it("should log messages with different levels", function()
        local logger = P.logger.Logger("TEST.LOGGER")
        logger:debug("Debug message")
        logger:info("Info message")
        logger:warn("Warning message")
        logger:error("Error message")
    end)
end)

lester.report()