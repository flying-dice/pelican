local ____lualib = require("lualib_bundle")
local __TS__StringIncludes = ____lualib.__TS__StringIncludes
local __TS__SourceMapTraceBack = ____lualib.__TS__SourceMapTraceBack
__TS__SourceMapTraceBack(debug.getinfo(1).short_src, {["6"] = 1,["7"] = 1,["8"] = 2,["9"] = 2,["10"] = 2,["11"] = 2,["12"] = 10,["13"] = 10,["14"] = 10,["15"] = 11,["16"] = 12,["17"] = 15,["18"] = 16,["19"] = 18,["20"] = 18,["21"] = 18,["22"] = 19,["23"] = 19,["24"] = 19,["25"] = 20,["26"] = 21,["27"] = 19,["28"] = 19,["29"] = 24,["30"] = 24,["31"] = 24,["32"] = 25,["33"] = 26,["34"] = 27,["35"] = 24,["36"] = 24,["37"] = 30,["38"] = 30,["39"] = 30,["40"] = 31,["41"] = 32,["42"] = 32,["43"] = 32,["44"] = 30,["45"] = 30,["46"] = 18,["47"] = 18,["48"] = 36,["49"] = 36,["50"] = 36,["51"] = 37,["52"] = 37,["53"] = 37,["54"] = 38,["55"] = 39,["56"] = 40,["57"] = 41,["58"] = 37,["59"] = 37,["60"] = 44,["61"] = 44,["62"] = 44,["63"] = 45,["64"] = 46,["65"] = 47,["66"] = 44,["67"] = 44,["68"] = 50,["69"] = 50,["70"] = 50,["71"] = 51,["72"] = 55,["73"] = 57,["74"] = 58,["75"] = 59,["76"] = 50,["77"] = 50,["78"] = 62,["79"] = 62,["80"] = 62,["81"] = 63,["82"] = 66,["83"] = 67,["84"] = 62,["85"] = 62,["86"] = 70,["87"] = 70,["88"] = 70,["89"] = 71,["90"] = 72,["91"] = 70,["92"] = 70,["93"] = 75,["94"] = 75,["95"] = 75,["96"] = 76,["97"] = 77,["98"] = 78,["99"] = 75,["100"] = 75,["101"] = 81,["102"] = 81,["103"] = 81,["104"] = 82,["105"] = 83,["106"] = 84,["107"] = 85,["108"] = 81,["109"] = 81,["110"] = 36,["111"] = 36,["112"] = 10,["113"] = 10});
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
