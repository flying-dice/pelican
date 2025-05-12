package.path = package.path .. ";" .. lfs.writedir() .. "\\Mods\\tech\\Pelican\\scripts\\?.lua"
package.cpath = package.cpath .. ";" .. lfs.writedir() .. "\\Mods\\tech\\Pelican\\bin\\?.dll"

local P = require("pelican")

P.logger.init_config({
    level = "DEBUG",
    pattern = "{d} - {l} - {m}{n}"
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

local sum_schema = {
    type = "array",
    items = {
        type = "number"
    }
}
router:add_method("sum", function(params)
    P.valico.validate(sum_schema, params)
    local a, b = params[1], params[2]

    return a + b
end)

timer.scheduleFunction(function(arg, time)
    server:process_rpc(router)
    return timer.getTime() + .1
end, nil, timer.getTime() + .1)