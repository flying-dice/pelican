# Mission Scripting Environment

For more details about the Mission Scripting Environment (MSE), refer to the Eagle Dynamics documentation:

[https://www.digitalcombatsimulator.com/en/support/faq/1249/](https://www.digitalcombatsimulator.com/en/support/faq/1249/)

---

## Preparing the Lua Environment

> ⚠️ The MSE **sanitizes the Lua environment**, disabling `package`, `require`, and other functions for security
> reasons.

Pelican usage in the MSE depends on when and how your Lua code runs relative to the DCS sanitization process. Choose one
of the following approaches:

---

### ✅ Option 1: Load Scripts Before Sanitization (Preferred)

Use `dofile` to load your script (using pelican) before DCS sanitizes the environment.

**Use Case:**
Ideal for scripts that should run globally across all missions—such as sidecars for data extraction, dynamic weather, or
general purpose AI control.

**Example Scenario:**

* Running an HTTP backend for a web client showing player locations.
* Implementing reusable logic across missions.

**Pros:**

* ✅ Keeps `require` and `package` disabled after setup (maintains DCS security).
* ✅ Global effect—only needs to be set up once.
* ✅ No modification to mission files needed.

**Cons:**

* ⚠️ Runs across **all missions**, which may be unnecessary for mission-specific logic.

See [option 1 documentation](./option_1.md) for details.

---

### ⚠️ Option 2: Disable Sanitization in `MissionScripting.lua`

Manually edit `MissionScripting.lua` to re-enable `require` and `package` inside mission scripts.

**Use Case:**
Required when Pelican is used **within mission-embedded scripts** that must call `require`.

**Example Scenario:**

* Using Pelican and SQLite for persistent mission data.

**Pros:**

* ✅ Enables full Lua module functionality within mission scripts.

**Cons:**

* ⚠️ Introduces potential security risks.
* ⚠️ Not recommended for multiplayer or untrusted missions.

See [option 2 documentation](./option_2.md) for details.

---

## Using Pelican in a Mission Script

Regardless of the approach, your Lua script must **set the DLL path** correctly before requiring Pelican.

Here’s a minimal example of a Pelican-powered Lua script that starts a JSON-RPC server:

{@includeCode ./PelicanTestMission.lua}

---

## Testing the Server

Use the following `curl` command to test your Pelican server:

```bash
curl --location 'http://localhost:1235/rpc' \
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

You can also verify the server is running by visiting:

```
http://localhost:1235/health
```