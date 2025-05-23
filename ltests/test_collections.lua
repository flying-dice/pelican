package.path = package.path .. ";.\\ltests\\?.lua"
package.cpath = package.cpath .. ";.\\target\\debug\\?.dll"

---@type pelican
local P = require("pelican")
local lester = require('lester')

local describe, it, expect = lester.describe, lester.it, lester.expect

describe("collections.Vec", function()
    it("should construct an empty vec", function()
        local arr = P.collections.Vec()
        expect.equal(arr:len(), 0)
    end)

    it("should construct an vec with initial values", function()
        local arr = P.collections.Vec({ "a", "b", "c" })
        expect.equal(arr:len(), 3)
        expect.equal(arr:get(0), "a")
        expect.equal(arr:get(1), "b")
        expect.equal(arr:get(2), "c")
    end)

    it("should allow converting back to table", function()
        local arr = P.collections.Vec({ "a", "b", "c" })
        local tbl = arr:to_lua_table()
        expect.equal(#tbl, 3)
        expect.equal(tbl[1], "a")
        expect.equal(tbl[2], "b")
        expect.equal(tbl[3], "c")
    end)

    it("should allow setting values", function()
        local arr = P.collections.Vec({ "a", "b", "c" })
        arr:set(0, "x")
        expect.equal(arr:get(0), "x")
        expect.equal(arr:get(1), "b")
        expect.equal(arr:get(2), "c")
    end)

    it("should pop values from vec", function()
        local arr = P.collections.Vec({ "a", "b", "c" })
        local c = arr:pop()
        local b = arr:pop()
        local a = arr:pop()
        expect.equal(a, "a")
        expect.equal(b, "b")
        expect.equal(c, "c")
        expect.equal(arr:len(), 0)
        expect.equal(arr:pop(), nil)
    end)

    it("should clear the vec", function()
        local arr = P.collections.Vec({ "a", "b", "c" })
        expect.equal(arr:len(), 3)
        arr:clear()
        expect.equal(arr:len(), 0)
    end)

    it("should allow iterating over the vec", function()
        local arr = P.collections.Vec({ 1, 2, 3 })
        local i = 0
        arr:for_each(function(value)
            i = i + value
        end)
        expect.equal(i, 6)
    end)

    it("should allow mapping the vec", function()
        local arr = P.collections.Vec({ 1, 2, 3 })
        local mapped = arr:map(function(value)
            return value * 2
        end)
        expect.equal(mapped:len(), 3)
        expect.equal(mapped:get(0), 2)
        expect.equal(mapped:get(1), 4)
        expect.equal(mapped:get(2), 6)
    end)

    it("should allow filtering the vec", function()
        local arr = P.collections.Vec({ 1, 2, 3, 4, 5 })
        local filtered = arr:filter(function(value)
            return value % 2 == 0 -- even numbers
        end)
        expect.equal(filtered:len(), 2)
        expect.equal(filtered:get(0), 2)
        expect.equal(filtered:get(1), 4)
    end)

    it("should allow reducing the vec", function()
        local arr = P.collections.Vec({ 1, 2, 3 })
        local sum = arr:reduce(function(acc, value)
            return acc + value
        end, 0)
        expect.equal(sum, 6)
    end)

    it("should allow reducing an array of objects", function()
        local arr = P.collections.Vec({
            { id = 1, name = "John" },
            { id = 2, name = "Jane" },
            { id = 3, name = "Doe" }
        })
        local names = arr:reduce(function(acc, value)
            acc[value.id] = value.name
            return acc
        end, {})
        expect.equal(names, {
            [1] = "John",
            [2] = "Jane",
            [3] = "Doe"
        })
    end)

    it("should allow reversing the vec", function()
        local arr = P.collections.Vec({ 1, 2, 3 })
        arr:reverse()
        expect.equal(arr:get(0), 3)
        expect.equal(arr:get(1), 2)
        expect.equal(arr:get(2), 1)
    end)
end)

lester.report()