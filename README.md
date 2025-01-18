# JSON-RPC Server with Lua Integration

This project provides a fully functional JSON-RPC server implemented in Rust with integration to Lua.
It supports both HTTP and WebSocket endpoints for receiving and responding to JSON-RPC requests.

The RPC Router and Request Handler is extended through Lua scripts, making it highly customizable and embeddable.

## Features

- **JSON-RPC Support**: Implements JSON-RPC 2.0 protocol.
- **Dual Endpoint**: Supports both HTTP and WebSocket endpoints for RPC communication.
- **Lua Integration**: Allows Lua scripts to dynamically handle and process RPC requests.
- **Asynchronous Processing**: Uses Actix accepting requests and conversion to and from JSON

## How It Works

### 1. Lua Integration

The server exposes functions to Lua, allowing Lua scripts to start the server, process incoming RPC requests, and
encode/decode JSON data. Here's an overview of the Lua functions provided:

- `start_server(config: AppConfig)`: Starts the server on the specified port
- `process_rpc(callback: function)`: Processes incoming RPC requests by calling the specified Lua callback function.
- `encode(value: table)`: Encodes a Lua table into a JSON string.
- `decode(json: string)`: Decodes a JSON string into a Lua table.

#### App Config

The `AppConfig` struct is used to configure the server. It includes the following fields:

- `host`: The host address to bind the server to.
- `port`: The port number to listen on.
- `workers`: The number of worker threads to use for processing requests.
- `api_key`: The API key required to access the server. (Optional)

The API key needs to be included in the request headers as `x-api-key` to authenticate the request.

### 2. Example Lua Usage

Here’s a sample Lua script to start the server and handle incoming RPC requests:

```lua
-- MAKE SURE YOUR WORKING DIRECTORY IS THE ROOT OF THE PROJECT NOT `src` OR ANY OTHER FOLDER
package.path = package.path .. ";.\\target\\debug\\?.lua"
package.cpath = package.cpath .. ";.\\target\\debug\\?.dll"

local jsonrpc = require("lua_json_rpc")

local stop = jsonrpc.start_server({
    host = "0.0.0.0",
    port = 1359,
    workers = 2,
    api_key = "super-secret-k3y"
})

io.write("JSON-RPC server started on port 1359\n")
io.flush()

function on_rpc(request)
    io.write("Routing Request: " .. request.method .. "\n")

    local response = {
        id = request.id,
        jsonrpc = "2.0",
    }

    if (string.match("subtract", request.method)) then
        response.result = request.params[1] - request.params[2]
    end

    io.flush()

    return response
end

local started = os.clock()

---- Run for 10 seconds
while os.clock() - started < 30 do
    jsonrpc.process_rpc(on_rpc)
end

print("Shutting down JSON-RPC server")
stop()

os.execute("echo Press any key to continue... && pause > nul")
```

## Usage

### Sending Requests

- **HTTP Endpoint**:  
  Send JSON-RPC 2.0 requests to `http://<server-address>:<port>/rpc` using POST method with a JSON body.

- **WebSocket Endpoint**:  
  Connect to `ws://<server-address>:<port>/ws` and send JSON-RPC 2.0 requests as text messages.

## Example RPC Request

### HTTP Request

```bash
curl -X POST http://localhost:1234/rpc \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc": "2.0", "method": "subtract", "params": [42, 23], "id": "1"}'
```

### WebSocket Request

```json
{
  "jsonrpc": "2.0",
  "method": "subtract",
  "params": [
    42,
    23
  ],
  "id": "1"
}
```

### Expected Response

```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "result": 19
}
```

## Error Handling

- The server returns a `400 Bad Request` response if the incoming JSON-RPC request is malformed.
- If the request is valid but no response is generated, a `202 Accepted` with OK is returned.
- If the request is a websocket request but no response is required (e.g., notifications), no response is sent.

## Building

The project can be built using Cargo. Run the following command in the project root directory:

```bash
cargo build
```

This will build the project in debug mode. To build in release mode, run:

```bash
cargo build --release
```

By default the build will produce a DLL compatible with the lua runtime installed on the host.

The build is set up for DCS world currently as such the lua5.1 directory is provided to ensure compatibility with DCS
world.

Run `build-dcs.bat` to build the project for DCS world creating a compatible DLL.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue to discuss potential changes or improvements.

## License

This project is licensed under the MIT License. See the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgements

Special thanks to the contributors of Actix, mlua, and the Rust and Lua communities for their excellent libraries and
support.
