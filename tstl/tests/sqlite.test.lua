local ____lualib = require("lualib_bundle")
local __TS__StringIncludes = ____lualib.__TS__StringIncludes
local __TS__SourceMapTraceBack = ____lualib.__TS__SourceMapTraceBack
__TS__SourceMapTraceBack(debug.getinfo(1).short_src, {["6"] = 1,["7"] = 1,["8"] = 2,["9"] = 2,["10"] = 2,["11"] = 2,["12"] = 10,["13"] = 10,["14"] = 10,["15"] = 11,["16"] = 11,["17"] = 11,["18"] = 12,["19"] = 12,["20"] = 12,["21"] = 13,["22"] = 14,["23"] = 14,["24"] = 14,["25"] = 14,["26"] = 12,["27"] = 12,["28"] = 11,["29"] = 11,["30"] = 18,["31"] = 19,["32"] = 22,["33"] = 23,["34"] = 25,["35"] = 25,["36"] = 25,["37"] = 26,["38"] = 26,["39"] = 26,["40"] = 27,["41"] = 28,["42"] = 26,["43"] = 26,["44"] = 31,["45"] = 31,["46"] = 31,["47"] = 32,["48"] = 33,["49"] = 34,["50"] = 31,["51"] = 31,["52"] = 37,["53"] = 37,["54"] = 37,["55"] = 38,["56"] = 39,["57"] = 39,["58"] = 39,["59"] = 37,["60"] = 37,["61"] = 25,["62"] = 25,["63"] = 43,["64"] = 43,["65"] = 43,["66"] = 44,["67"] = 44,["68"] = 44,["69"] = 45,["70"] = 46,["71"] = 47,["72"] = 48,["73"] = 44,["74"] = 44,["75"] = 51,["76"] = 51,["77"] = 51,["78"] = 52,["79"] = 53,["80"] = 54,["81"] = 51,["82"] = 51,["83"] = 57,["84"] = 57,["85"] = 57,["86"] = 58,["87"] = 62,["88"] = 64,["89"] = 65,["90"] = 66,["91"] = 57,["92"] = 57,["93"] = 69,["94"] = 69,["95"] = 69,["96"] = 70,["97"] = 73,["98"] = 74,["99"] = 69,["100"] = 69,["101"] = 77,["102"] = 77,["103"] = 77,["104"] = 78,["105"] = 79,["106"] = 77,["107"] = 77,["108"] = 82,["109"] = 82,["110"] = 82,["111"] = 83,["112"] = 84,["113"] = 85,["114"] = 82,["115"] = 82,["116"] = 88,["117"] = 88,["118"] = 88,["119"] = 89,["120"] = 90,["121"] = 91,["122"] = 92,["123"] = 88,["124"] = 88,["125"] = 43,["126"] = 43,["127"] = 10,["128"] = 10});
local ____exports = {}
local ____pelican = require("pelican")
local sqlite = ____pelican.sqlite
local ____lester = require("tests.lester")
local describe = ____lester.describe
local expect = ____lester.expect
local it = ____lester.it
describe(
    "sqlite",
    function()
        describe(
            "SQLiteConnection",
            function()
                it(
                    "should instantiate using the new keyword",
                    function()
                        local connection = sqlite.SQLiteConnection.new(":memory:")
                        expect.equal(
                            tostring(connection),
                            "SQLiteConnection(:memory:)"
                        )
                    end
                )
            end
        )
        local connection = sqlite.SQLiteConnection.new(":memory:")
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
