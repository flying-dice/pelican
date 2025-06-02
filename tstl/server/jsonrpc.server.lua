--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]
local ____exports = {}
local ____pelican = require("pelican")
local web = ____pelican.web
PELICAN = {logger_level = "debug"}
local server = web.serve({port = 1234, host = "localhost"})
local router = web.router()
router:add_method(
    "ping",
    function(props)
        return ("Pong, " .. props.message) .. "!"
    end
)
while true do
    server:process_rpc(router)
end
return ____exports
