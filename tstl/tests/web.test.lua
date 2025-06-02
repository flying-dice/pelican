--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]
local ____exports = {}
local ____pelican = require("pelican")
local json = ____pelican.json
local requests = ____pelican.requests
local ____lester = require("tests.lester")
local describe = ____lester.describe
local expect = ____lester.expect
local it = ____lester.it
describe(
    "web",
    function()
        local router_client = requests.BlockingHttpClient:new()
        it(
            "should respond to RPC calls",
            function()
                local encoded = json.encode({jsonrpc = "2.0", method = "ping", params = {message = "Tastic"}, id = "1"})
                local res, err = router_client:post("http://localhost:1234/rpc", encoded)
                expect.equal(err, nil)
                expect.equal(
                    res:get_text(),
                    "{\"jsonrpc\":\"2.0\",\"id\":\"1\",\"result\":\"Pong, Tastic!\"}"
                )
            end
        )
    end
)
return ____exports
