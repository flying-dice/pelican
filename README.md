# JSON-RPC Server with Lua Integration

This project provides a fully functional JSON-RPC server implemented in Rust with integration to Lua. 
It supports both HTTP and WebSocket endpoints for receiving and responding to JSON-RPC requests. 

The RPC Router and Request Handler is extended through Lua scripts, making it highly customizable and embeddable.

## Features

- **JSON-RPC Support**: Implements JSON-RPC 2.0 protocol.
- **Dual Endpoint**: Supports both HTTP and WebSocket endpoints for RPC communication.
- **Lua Integration**: Allows Lua scripts to dynamically handle and process RPC requests.
- **Asynchronous Processing**: Uses Actix and Tokio for handling asynchronous requests.
- **Session Management**: Manages WebSocket sessions and broadcasts responses to all active sessions.

## How It Works

### 1. Lua Integration

The server exposes functions to Lua, allowing Lua scripts to start the server, process incoming RPC requests, and encode/decode JSON data. Here's an overview of the Lua functions provided:

- `start_server(port: number)`: Starts the server on the specified port.
- `process_rpc(callback: function)`: Processes incoming RPC requests by calling the specified Lua callback function.
- `encode(value: table)`: Encodes a Lua table into a JSON string.
- `decode(json: string)`: Decodes a JSON string into a Lua table.

### 2. Example Lua Usage

Here’s a sample Lua script to start the server and handle incoming RPC requests:

```lua
package.path = package.path .. ";.C:\\PATH\\TO\\DLL\\?.lua"
package.cpath = package.cpath .. ";C:\\PATH\\TO\\DLL\\?.dll"
local json_rpc = require("json_rpc_server")

-- Start the server on port 1234
json_rpc.start_server(1234)

-- Define the RPC handler
-- Payloads are JSON-RPC requests in string format
function on_rpc(payload)
    local request = json_rpc.decode(payload) -- {"jsonrpc": "2.0", "method": "subtract", "params": [42, 23], "id": "1"}

    print("Routing Request: " .. request.method)
    
    if (request.id == nil) then -- Used for handlers that don't require a response
        return
    end
    
    local response = {
        id = request.id,
        jsonrpc = "2.0",
    }

    if (string.match("subtract", request.method)) then
        response.result = request.params[1] - request.params[2]
    end

    io.flush()

    -- Return the response as a JSON string
    return json_rpc.encode(response) -- {"id":"1","jsonrpc":"2.0","result":19}
end

-- Continuously process incoming RPC requests
while true do
    json_rpc.process_rpc(on_rpc)
end
```

## Usage

Download Rust: https://www.rust-lang.org/

### Running the Server

1. **Build the Rust Library**:  
   Compile the Rust code to generate a shared library (`.dll`, `.so`, or `.dylib`) for Lua to load.

```shell 
cargo build --release
```

2. **Configure Lua Script**:  
   Update the `package.path` and `package.cpath` in your Lua script to point to the generated shared library.

3. **Run the Lua Script**:  
   Use a Lua interpreter to run your Lua script and start the server.

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
    "params": [42, 23],
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

## Building and Running

1. **Clone the Repository**:

   ```bash
   git clone <repository-url>
   cd json-rpc-server
   ```

2. **Build the Rust Project**:

   ```bash
   cargo build --release
   ```

3. **Run the Lua Script**:

   ```bash
   lua main.lua
   ```

## Contributing

Contributions are welcome! Please submit a pull request or open an issue to discuss potential changes or improvements.

## License

This project is licensed under the MIT License. See the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgements

Special thanks to the contributors of Actix, mlua, and the Rust and Lua communities for their excellent libraries and support.
