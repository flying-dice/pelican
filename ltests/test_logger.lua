package.path = package.path .. ";.\\ltests\\?.lua"
package.cpath = package.cpath .. ";.\\target\\debug\\?.dll"
local jsonrpc = require("lua_json_rpc")
local lester = require('lester')

local describe, it, expect = lester.describe, lester.it, lester.expect

describe("jsonrpc.logger", function()
    it("should error as logger is already initialized", function()
        local res, err = jsonrpc.configure_logger('log4rs.yaml')
        expect.equal(err, "Error configuring logger: attempted to set a logger after the logging system was already initialized")
        expect.equal(res, nil)
    end)
end)

lester.report()
