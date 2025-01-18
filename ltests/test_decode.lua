package.path = package.path .. ";.\\ltests\\?.lua"
package.cpath = package.cpath .. ";.\\target\\debug\\?.dll"
local jsonrpc = require("lua_json_rpc")
local lester = require('lester')

local describe, it, expect = lester.describe, lester.it, lester.expect

describe("jsonrpc.decode", function()
    it("should succeed with valid JSON", function()
        local res, err = jsonrpc.decode('{"jsonrpc": "2.0", "method": "subtract", "params": [42, 23], "id": 1}')
        expect.equal(err, nil)
        expect.equal(res.jsonrpc, "2.0")
        expect.equal(res.method, "subtract")
        expect.equal(res.params[1], 42)
        expect.equal(res.params[2], 23)
        expect.equal(res.id, 1)
    end)

    it("should return error for invalid json", function()
        local res, err = jsonrpc.decode("abc")
        expect.equal(res, nil)
        expect.equal(err, "Invalid JSON: abc")
    end)
end)

lester.report()
