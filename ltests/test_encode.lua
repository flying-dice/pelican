package.path = package.path .. ";.\\ltests\\?.lua"
package.cpath = package.cpath .. ";.\\target\\debug\\?.dll"
local jsonrpc = require("lua_json_rpc")
local lester = require('lester')

local describe, it, expect = lester.describe, lester.it, lester.expect

describe("jsonrpc.encode", function()
    it("should succeed with valid integer", function()
        local res, err = jsonrpc.encode(1)
        expect.equal(err, nil)
        expect.equal(res, "1")
    end)

    it("should succeed with valid string", function()
        local res, err = jsonrpc.encode("hello")
        expect.equal(err, nil)
        expect.equal(res, "\"hello\"")
    end)

    it("should succeed with valid table", function()
        local res, err = jsonrpc.encode({ a = 1, b = 2 })
        expect.equal(err, nil)
        expect.equal(res, "{\"a\":1,\"b\":2}")
    end)

    it("should succeed with valid table with array of numbers", function()
        local res, err = jsonrpc.encode({ 1, 2, 3 })
        expect.equal(err, nil)
        expect.equal(res, "[1,2,3]")
    end)

    it("should succeed with valid table with array of strings", function()
        local res, err = jsonrpc.encode({ [1] = "one", [2] = "two", [3] = "three" })
        expect.equal(err, nil)
        expect.equal(res, "[\"one\",\"two\",\"three\"]")
    end)

    it("should succeed with valid table with array of mixed types", function()
        local res, err = jsonrpc.encode({ [1] = 1, [2] = "two", [3] = 3 })
        expect.equal(err, nil)
        expect.equal(res, "[1,\"two\",3]")
    end)

    it("should map array up to missing index", function()
        local sparsetable = { [1] = 1, [3] = 3 }
        local res, err = jsonrpc.encode(sparsetable)
        expect.equal(err, nil)
        expect.equal(res, "[1]")
    end)
end)

lester.report()