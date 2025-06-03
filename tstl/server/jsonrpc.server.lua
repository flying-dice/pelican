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
router:add_method(
    "throws",
    function()
        error(
            __TS__New(Error, "This is an error from the server."),
            0
        )
    end
)
while true do
    server:process_rpc(router)
end
return ____exports
