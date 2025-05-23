package.path = package.path .. ";.\\ltests\\?.lua"
package.cpath = package.cpath .. ";.\\target\\debug\\?.dll"

---@type pelican
local P = require("pelican")
local lester = require('lester')

local describe, it, expect = lester.describe, lester.it, lester.expect

describe("sqlite", function()
    local connection = P.sqlite.open(":memory:")
    connection:exec("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT UNIQUE, age INTEGER);")
    connection:exec("INSERT INTO users (name, age) VALUES ('John Doe', 30);")
    connection:exec("INSERT INTO users (name, age) VALUES ('Jane Smith', 25);")

    describe("exec", function()

        it("should execute a valid SQL statement", function()
            local res, err = connection:exec("INSERT OR REPLACE INTO users (name, age) VALUES ('Jane Smith', 25);")
            expect.equal(err, nil)
            expect.equal(res, true)
        end)

        it("should return an error for invalid SQL statement", function()
            local res, err = connection:exec("INVALID SQL")
            expect.equal(res, nil)
            expect.equal(err, "SQLite error: near \"INVALID\": syntax error (code 1)")
        end)

    end)

    describe("execute", function()

        it("should return results for a valid SELECT statement", function()
            local res, err = connection:execute("SELECT * FROM users;")
            expect.equal(err, nil)
            expect.equal(#res, 2)
            expect.equal(res[1].name, "John Doe")
            expect.equal(res[2].name, "Jane Smith")
        end)

        it("should support binding parameters", function()
            local res, err = connection:execute("SELECT * FROM users WHERE age = ?;", { 30 })
            expect.equal(err, nil)
            expect.equal(#res, 1)
            expect.equal(res[1].name, "John Doe")
        end)

        it("should support array binds for inserts", function()
            local res, err = connection:execute("INSERT INTO users (name, age) VALUES (?, ?);", { "Alice", 66 })
            expect.equal(err, nil)
            expect.equal(res, {})

            local select_res, select_err = connection:execute("SELECT * FROM users WHERE age > 65;")
            expect.equal(select_err, nil)
            expect.equal(#select_res, 1)
            expect.equal(select_res[1].name, "Alice")
        end)

        it("should support named binds", function()
            local res, err = connection:execute("SELECT * FROM users WHERE name = :name;", { name = 'John Doe' })
            expect.equal(err, nil)
            expect.equal(#res, 1)
            expect.equal(res[1].name, "John Doe")
        end)

        it("should return an error for invalid SELECT statement", function()
            local res, err = connection:execute("INVALID SQL")
            expect.equal(res, nil)
            expect.equal(err, "SQLite error: near \"INVALID\": syntax error (code 1)")
        end)

        it("should return an error for non-existent table", function()
            local res, err = connection:execute("SELECT * FROM non_existent_table;")
            expect.equal(res, nil)
            expect.equal(err, "SQLite error: no such table: non_existent_table (code 1)")
        end)

    end)

end)

lester.report()