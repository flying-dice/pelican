# Pelican

**Pelican** is a modular library for **DCS World**, focused on enabling **external integrations** via an embedded **HTTP
and WebSocket server**. It allows developers to build rich interfaces and tools that interact with DCS in real-time,
making it ideal for dashboards, companion apps, telemetry capture, automation systems, and more.

Pelican is designed for use in both the **Mission Scripting Environment (MSE)** and the **GUI Scripting Environment**
within DCS world.

## Key Features

- ✅ Embedded **HTTP** and **WebSocket** JSON-RPC server for real-time, bidirectional communication with DCS
- ✅ Built-in **SQLite** module providing direct access to SQLite APIs from within the LUA environment
- ✅ Compatible with both **GUI** and **Mission** scripting environments
- ✅ Modular architecture for flexibility and extensibility

## Installation

Pelican is distributed as a `DLL` file. To install:

1. Place the DLL into the following directory:

    ```
    %USERPROFILE%\Saved Games\DCS\Mods\tech\Pelican\bin
    ```

2. Replace `Pelican` with the actual name of your mod if you want to isolate it from other usages, i.e. for Version
   Pinning
   or to avoid conflicts with other mods.

> ⚠️ Ensure that the directory structure matches your mod usage.

## Setup

Pelican's setup process differs depending on the environment you're using.

### GUI Scripting Environment

Refer to the [GUI Environment setup guide](./gui.md) for detailed instructions on configuring Pelican for GUI scripting.

### Mission Scripting Environment

Refer to the [Mission Scripting Environment setup guide](./mission.md) for details on configuring Pelican in the MSE
context.

---

## Setup

If you're comfortable with DCS scripting, here's a simple setup example for both environments.

### GUI Environment Example

Place the following script in your DCS World Saved Games Scripts/Hooks folder:

{@includeCode ./PelicanTestGameGUI.lua}

### Mission Scripting Environment Example

1. Place this script in your DCS World Saved Games Scripts folder:

{@includeCode ./PelicanTestMission.lua}

2. Modify your `MissionScripting.lua` to load the script before sandbox restrictions are applied:

{@includeCode ./MissionScripting.lua}

> Alternatively, re-add the `require`, `package` and `lfs` functions bearing in mind the security implications of doing
> so.
