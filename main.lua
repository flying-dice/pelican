-- MAKE SURE YOUR WORKING DIRECTORY IS THE ROOT OF THE PROJECT NOT `src` OR ANY OTHER FOLDER
package.path = package.path .. ";.\\target\\debug\\?.lua"
package.cpath = package.cpath .. ";.\\target\\debug\\?.dll"

local json_rpc = require("json_rpc_server")

json_rpc.start_server(1234)

function on_rpc(payload)
    local request = json_rpc.decode(payload)

    if (request.id == nil) then
        return
    end

    print("Routing Request: " .. request.method)

    local response = {
        id = request.id,
        jsonrpc = "2.0",
    }

    if (string.match("subtract", request.method)) then
        response.result = request.params[1] - request.params[2]
    end

    io.flush()

    return json_rpc.encode(response)
end

while true do
    json_rpc.process_rpc(on_rpc)
end