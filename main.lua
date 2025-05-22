-- MAKE SURE YOUR WORKING DIRECTORY IS THE ROOT OF THE PROJECT NOT `src` OR ANY OTHER FOLDER
package.path = package.path .. ";.\\target\\debug\\?.lua"
package.cpath = package.cpath .. ";.\\target\\debug\\?.dll"

---@type pelican
local P = require("pelican")

P.logger.init_config({
    level = "DEBUG",
    pattern = "{d} - {l} - {m}{n}"
})

P.logger.info("Pelican Running...")
P.logger.info("Pelican Name: " .. P.name)
P.logger.info("Pelican Version: " .. P.version)

local server = P.web.serve({
    host = "127.0.0.1",
    port = 1234,
})

local connection = P.sqlite.open(":memory:")
connection:execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER);")
connection:execute("INSERT INTO users (name, age) VALUES ('John Doe', 30);")
connection:execute("INSERT INTO users (name, age) VALUES ('Jane Smith', 25);")

local router = P.web.router()

local sum_params_validator = P.jsonschema.validator_for({
    type = "array",
    minItems = 2,
    maxItems = 2,
    items = {
        type = "number"
    }
})

router:add_method("sum", function(params)
    local res, err = sum_params_validator:validate(params)

    if (err) then
        P.logger.error("Validation error: " .. err)
        error(err)
    end

    local a, b = params[1], params[2]

    return a + b
end)

router:add_method("get_weather", function()
    local weather_res, err = P.requests.get(
            "https://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&hourly=temperature_2m")

    if (err) then
        P.logger.error("Request error: " .. err)
        error(err)
    end

    P.logger.info("Request status: " .. weather_res:get_status())

    P.logger.info("Weather data: " .. weather_res:get_text())

    return weather_res:get_text()
end)

router:add_method("get_users_bind_place", function()
    local users_res, err = connection:query("SELECT * FROM users WHERE age > ?;", { 20 })

    if (err) then
        P.logger.error("Database error: " .. err)
        error(err)
    end

    return users_res
end)

router:add_method("get_users_bind_name", function()
    local users_res, err = connection:query("SELECT * FROM users WHERE name = :name;", { name = 'John Doe' })

    if (err) then
        P.logger.error("Database error: " .. err)
        error(err)
    end

    return users_res
end)

while true do
    local result, err = server:process_rpc(router)
    if err then
        --P.logger.error("RPC error: " .. tostring(err))
    end
end
