package.path = package.path .. ";" .. lfs.writedir() .. "\\Mods\\tech\\Pelican\\scripts\\?.lua"
package.cpath = package.cpath .. ";" .. lfs.writedir() .. "\\Mods\\tech\\Pelican\\bin\\?.dll"

local P = require("pelican")

P.logger.init_config({
    level = "DEBUG",
    pattern = "{d} - {l} - {m}{n}",
    file = lfs.writedir() .. "\\Logs\\pelican.log",
})

P.logger.info("Pelican Running...")
P.logger.info("Pelican Name: " .. P.name)
P.logger.info("Pelican Version: " .. P.version)
P.logger.info(P.json.encode({ a = 1, b = 2 }))
P.logger.info(P.uuid.v4())

local server = P.web.serve({
    host = "127.0.0.1",
    port = 1234,
})

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

local connection = P.sqlite.open("C:\\Users\\jonat\\RustroverProjects\\json-rpc-server\\pelican.db")
connection:execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER);")
connection:execute("INSERT INTO users (name, age) VALUES ('John Doe', 30);")
connection:execute("INSERT INTO users (name, age) VALUES ('Jane Smith', 25);")

router:add_method("get_users", function()
    local users, err = connection:query("SELECT * FROM users WHERE age > ?;", { 20 })

    if (err) then
        P.logger.error("Database error: " .. err)
        error(err)
    end

    return users
end)

connection:execute("CREATE TABLE IF NOT EXISTS airbases (id INTEGER PRIMARY KEY, name TEXT, x REAL, y REAL, z REAL, lat REAL, lng REAL, alt REAL);")

for _, airbase in ipairs(world.getAirbases()) do
    local position = airbase:getPosition().p
    local lat, lon, alt = coord.LOtoLL(position)

    connection:query("INSERT INTO airbases (id, name, x, y, z, lat, lng, alt) VALUES (?, ?, ?, ?, ?, ?, ?, ?);",
            { airbase:getID(), airbase:getName(), position.x, position.y, position.z, lat, lon, alt })

    P.logger.info("Airbase inserted: " .. airbase:getName())
end

router:add_method("get_airbases", function()
    local abs = {}

    for _, airbase in ipairs(world.getAirbases()) do
        local ab = {}

        local position = airbase:getPosition().p
        local lat, lon, alt = coord.LOtoLL(position)

        ab.id = airbase:getID()
        ab.name = airbase:getName()
        ab.x = position.x
        ab.y = position.y
        ab.z = position.z
        ab.lat = lat
        ab.lng = lon
        ab.alt = alt

        table.insert(abs, ab)

    end

    return abs
end)

timer.scheduleFunction(function(arg, time)
    local result, err = server:process_rpc(router)
    if not result then
        P.logger.error("RPC error: " .. tostring(err))
    end

    return timer.getTime() + .1
end, nil, timer.getTime() + .1)