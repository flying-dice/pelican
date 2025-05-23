package.path = package.path .. ";.\\ltests\\?.lua"
package.cpath = package.cpath .. ";.\\target\\debug\\?.dll"

---@type pelican
local P = require("pelican")
local lester = require('lester')

local describe, it, expect = lester.describe, lester.it, lester.expect

describe("json.decode", function()
    it("should succeed with valid JSON", function()
        local res, err = P.json.decode('{"jsonrpc": "2.0", "method": "subtract", "params": [42, 23], "id": 1}')
        expect.equal(err, nil)
        expect.equal(res.jsonrpc, "2.0")
        expect.equal(res.method, "subtract")
        expect.equal(res.params[1], 42)
        expect.equal(res.params[2], 23)
        expect.equal(res.id, 1)
    end)

    it("should return error for invalid json", function()
        local res, err = P.json.decode("abc")
        expect.equal(res, nil)
        expect.equal(err, "Invalid JSON: \"abc\"")
    end)
end)

describe("json.encode", function()
    it("should succeed with valid integer", function()
        local res, err = P.json.encode(1)
        expect.equal(err, nil)
        expect.equal(res, "1")
    end)

    it("should succeed with valid string", function()
        local res, err = P.json.encode("hello")
        expect.equal(err, nil)
        expect.equal(res, "\"hello\"")
    end)

    it("should succeed with valid table", function()
        local res, err = P.json.encode({ a = 1, b = 2 })
        expect.equal(err, nil)
        expect.equal(res, "{\"a\":1,\"b\":2}")
    end)

    it("should succeed with valid table with array of numbers", function()
        local res, err = P.json.encode({ 1, 2, 3 })
        expect.equal(err, nil)
        expect.equal(res, "[1,2,3]")
    end)

    it("should succeed with valid table with array of strings", function()
        local res, err = P.json.encode({ [1] = "one", [2] = "two", [3] = "three" })
        expect.equal(err, nil)
        expect.equal(res, "[\"one\",\"two\",\"three\"]")
    end)

    it("should succeed with valid table with array of mixed types", function()
        local res, err = P.json.encode({ [1] = 1, [2] = "two", [3] = 3 })
        expect.equal(err, nil)
        expect.equal(res, "[1,\"two\",3]")
    end)

    it("should map array up to missing index", function()
        local sparsetable = { [1] = 1, [3] = 3 }
        local res, err = P.json.encode(sparsetable)
        expect.equal(err, nil)
        expect.equal(res, "[1]")
    end)
end)

lester.report()