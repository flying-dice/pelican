package.path = package.path .. ";" .. lfs.writedir() .. "\\Scripts\\?.lua"
package.cpath = package.cpath .. ";" .. lfs.writedir() .. "\\Scripts\\?.dll"

local jsonrpc = require("lua_json_rpc")

local port = 1359

env.info("[EXAMPLE] - Starting JSON-RPC server on port " .. port .. ", ARCH: " .. _ARCHITECTURE .. " VERION: " .. _VERSION)

__stop = jsonrpc.start_server({
    host = "0.0.0.0",
    port = port,
    workers = 2
})

function loop()
    jsonrpc.process_rpc(on_rpc)
end

function __loop()
    local success, err = pcall(loop)
    if not success then
        env.error("loop() error: " .. tostring(err), false) -- false to avoid a popup in DCS
    end
end

function on_rpc(payload)
    if (payload.id == nil) then
        return
    end

    local response = {
        id = payload.id,
        jsonrpc = "2.0",
    }

    if (string.match("subtract", payload.method)) then
        env.info("Subtracting " .. payload.params[1] .. " - " .. payload.params[2])
        response.result = payload.params[1] - payload.params[2]
    end

    return response
end