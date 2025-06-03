local ____lualib = require("lualib_bundle")
local Error = ____lualib.Error
local RangeError = ____lualib.RangeError
local ReferenceError = ____lualib.ReferenceError
local SyntaxError = ____lualib.SyntaxError
local TypeError = ____lualib.TypeError
local URIError = ____lualib.URIError
local __TS__New = ____lualib.__TS__New
local ____exports = {}
local ____pelican = require("pelican")
local jsonschema = ____pelican.jsonschema
local sqlite = ____pelican.sqlite
local users_db = sqlite.SQLiteConnection.new(":memory:")
users_db:exec("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER);")
local user_validator = jsonschema.Validator.new({type = "object", properties = {id = {type = "integer"}, name = {type = "string"}, age = {type = "integer", minimum = 18}}, required = {"name", "age"}})
function ____exports.add_users(self, router)
    router:add_method(
        "get_users",
        function()
            local res = users_db:execute("SELECT * FROM users;")
            return res
        end
    )
    router:add_method(
        "get_user",
        function(props)
            local res = users_db:execute("SELECT * FROM users WHERE id = ?", {props.id})
            if not res[1] then
                error(
                    __TS__New(
                        Error,
                        ("User with ID " .. tostring(props.id)) .. " not found."
                    ),
                    0
                )
            end
            return res[1]
        end
    )
    router:add_method(
        "add_user",
        function(props)
            local valid, err = user_validator:validate(props)
            if not valid then
                error(
                    __TS__New(
                        Error,
                        "Invalid user data: " .. tostring(err)
                    ),
                    0
                )
            end
            users_db:execute("INSERT INTO users (name, age) VALUES (?, ?);", {props.name, props.age})
            local res = users_db:execute("SELECT * FROM users WHERE name = ?;", {props.name})
            if not res[1] then
                error(
                    __TS__New(Error, "Failed to add user: " .. props.name),
                    0
                )
            end
            return res[1]
        end
    )
    router:add_method(
        "delete_all_users",
        function()
            users_db:exec("DELETE FROM users;")
            return true
        end
    )
end
return ____exports
