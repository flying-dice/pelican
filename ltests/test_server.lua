package.path = package.path .. ";.\\ltests\\?.lua"
package.cpath = package.cpath .. ";.\\target\\debug\\?.dll"
local jsonrpc = require("lua_json_rpc")
local lester = require('lester')

local describe, it, expect = lester.describe, lester.it, lester.expect

describe("jsonrpc.server", function()
    it("should create server and provide close callback", function()
        local res, err = jsonrpc.start_server({ host = "0.0.0.0", port = 3000, workers = 2 })
        expect.equal(type(res), "function")

        res()
    end)
end)

lester.report()
