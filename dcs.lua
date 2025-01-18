package.path = package.path .. ";" .. lfs.writedir() .. "\\Scripts\\?.lua"
package.cpath = package.cpath .. ";" .. lfs.writedir() .. "\\Scripts\\?.dll"

local jsonrpc = require("lua_json_rpc")
jsonrpc.configure_logger("C:\\Users\\jonat\\RustroverProjects\\json-rpc-server\\log4rs.yaml")

local port = 11359

env.info("Starting JSON-RPC server on port " .. port .. ", ARCH: " .. _ARCHITECTURE .. " VERION: " .. _VERSION)

jsonrpc.start_server({
    port = port,
    workers = 2,
})

function on_rpc(payload)
    local request = jsonrpc.decode(payload)

    if (request.id == nil) then
        return
    end

    local response = {
        id = request.id,
        jsonrpc = "2.0",
    }

    if (string.match("subtract", request.method)) then
        env.info("Subtracting " .. request.params[1] .. " - " .. request.params[2])
        response.result = request.params[1] - request.params[2]
    end

    return jsonrpc.encode(response)
end

function loop()
    jsonrpc.process_rpc(on_rpc)
end

timer.scheduleFunction(
        function(arg, time)
            local success, err = pcall(loop)
            if not success then
                env.error("loop() error: " .. tostring(err), false) -- false to avoid a popup in DCS
            end
            return timer.getTime() + .1
        end, nil, timer.getTime() + .1
)

timer.scheduleFunction(
        function(arg, time)
            env.info("Shutting down JSON-RPC server")
            jsonrpc.stop_server()
            return nil
        end, nil, timer.getTime() + 10
)