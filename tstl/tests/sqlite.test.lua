local ____lualib = require("lualib_bundle")
local __TS__StringIncludes = ____lualib.__TS__StringIncludes
local ____exports = {}
local ____pelican = require("pelican")
local sqlite = ____pelican.sqlite
local ____lester = require("tests.lester")
local describe = ____lester.describe
local expect = ____lester.expect
local it = ____lester.it
describe(
    "sqlite module",
    function()
        local connection = sqlite.open(":memory:")
        connection:exec("CREATE TABLE IF NOT EXISTS users(id   INTEGER PRIMARY KEY, name TEXT UNIQUE, age  INTEGER CHECK ( typeof(age) = 'integer'));")
        connection:exec("INSERT INTO users (name, age) VALUES ('John Doe', 30);")
        connection:exec("INSERT INTO users (name, age) VALUES ('Jane Smith', 25);")
        describe(
            "exec",
            function()
                it(
                    "executes a valid INSERT OR REPLACE statement",
                    function()
                        local res = connection:exec("INSERT OR REPLACE INTO users (name, age) VALUES ('Jane Smith', 25);")
                        expect.equal(res, true)
                    end
                )
                it(
                    "returns an error for malformed SQL",
                    function()
                        local res, err = connection:exec("INVALID SQL")
                        expect.equal(res, nil)
                        expect.equal(err, "SQLite error: near \"INVALID\": syntax error (code 1)")
                    end
                )
                it(
                    "returns an error when violating UNIQUE constraint",
                    function()
                        local ____, err = connection:exec("INSERT INTO users (name, age) VALUES ('Jane Smith', 40);")
                        local ____expect_truthy_2 = expect.truthy
                        local ____opt_0 = err
                        ____expect_truthy_2(____opt_0 and __TS__StringIncludes(err, "UNIQUE constraint failed"))
                    end
                )
            end
        )
        describe(
            "execute",
            function()
                it(
                    "returns all users from SELECT query",
                    function()
                        local res = connection:execute("SELECT * FROM users;")
                        expect.equal(#res, 2)
                        expect.equal(res[1].name, "John Doe")
                        expect.equal(res[2].name, "Jane Smith")
                    end
                )
                it(
                    "returns filtered users using positional bind",
                    function()
                        local res = connection:execute("SELECT * FROM users WHERE age = ?;", {30})
                        expect.equal(#res, 1)
                        expect.equal(res[1].name, "John Doe")
                    end
                )
                it(
                    "inserts a new user using positional binds and validates insert",
                    function()
                        local res = connection:execute("INSERT INTO users (name, age) VALUES (?, ?);", {"Alice", 66})
                        expect.equal(#res, 0)
                        local selectRes = connection:execute("SELECT * FROM users WHERE age > 65;")
                        expect.equal(#selectRes, 1)
                        expect.equal(selectRes[1].name, "Alice")
                    end
                )
                it(
                    "returns user using named parameter",
                    function()
                        local res = connection:execute("SELECT * FROM users WHERE name = :name;", {name = "John Doe"})
                        expect.equal(#res, 1)
                        expect.equal(res[1].name, "John Doe")
                    end
                )
                it(
                    "returns empty result for non-matching WHERE clause",
                    function()
                        local res = connection:execute("SELECT * FROM users WHERE age = ?;", {99})
                        expect.equal(#res, 0)
                    end
                )
                it(
                    "returns error for invalid SQL statement",
                    function()
                        local res, err = connection:execute("INVALID SQL")
                        expect.equal(res, nil)
                        expect.equal(err, "SQLite error: near \"INVALID\": syntax error (code 1)")
                    end
                )
                it(
                    "should fail to insert a user with NaN age",
                    function()
                        connection:execute("INSERT INTO users (name, age) VALUES (?, ?);", {"Bob", "NaN"})
                        local qres = connection:execute("SELECT * FROM users WHERE name = 'Bob';")
                        expect.equal(#qres, 0)
                        expect.equal(qres[1], nil)
                    end
                )
            end
        )
    end
)
return ____exports
