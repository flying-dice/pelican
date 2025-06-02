# GUI Environment

For more details about the GUI environment, refer to the Eagle Dynamics documentation:

```
C:\Program Files\Eagle Dynamics\DCS World\API\DCS_ControlAPI.md
```

---

## Usage

To use Pelican in the GUI environment, create a Lua script in the following directory:

```
%USERPROFILE%\Saved Games\DCS\Scripts\Hooks
```

This script **must include the DLL path** in Lua’s `package.cpath` so it can load the Pelican module.

DCS World will automatically run all Lua scripts in this folder at startup.

---

### Example: Starting a JSON-RPC Server

Create a Lua file named `PelicanTestGameGUI.lua` with the following content:

{@includeCode ./PelicanTestGameGUI.lua}

---

## Testing the Server

To test the RPC server, run this command in the terminal:

```bash
curl --location 'http://localhost:1234/rpc' \
--header 'Content-Type: application/json' \
--data '{
  "jsonrpc": "2.0",
  "method": "ping",
  "params": [1],
  "id": "1"
}'
```

### Expected Response

```json
{
    "jsonrpc": "2.0",
    "id": "1",
    "result": {
        "message": "pong 1"
    }
}
```

You can also check the server's health endpoint at:

```
http://localhost:1234/health
```
