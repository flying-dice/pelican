local ____lualib = require("lualib_bundle")
local __TS__SourceMapTraceBack = ____lualib.__TS__SourceMapTraceBack
__TS__SourceMapTraceBack(debug.getinfo(1).short_src, {["4"] = 1,["5"] = 5,["6"] = 8,["7"] = 11,["10"] = 40,["11"] = 41,["14"] = 16,["15"] = 17,["16"] = 18,["17"] = 20,["18"] = 21,["19"] = 22,["20"] = 24,["21"] = 25,["22"] = 26,["23"] = 28,["24"] = 29,["25"] = 30,["26"] = 32,["27"] = 33,["28"] = 34,["29"] = 36,["30"] = 37,["31"] = 38});
PELICAN = {logger_level = "debug"}
print("========== Running Lua Tests ==========")
package.cpath = package.cpath .. ";..\\target\\debug\\?.dll"
package.cpath = package.cpath .. ";C:/Users/jonat/AppData/Roaming/JetBrains/RustRover2025.1/plugins/EmmyLua/debugger/emmy/windows/x64/?.dll"
do
    local function ____catch(e)
        print("An error occurred while running tests: " .. tostring(e))
        print(debug.traceback())
    end
    local ____try, ____hasReturned = pcall(function()
        print("========== json ==========")
        dofile("tests/json.test.lua")
        print("\n")
        print("========== jsonschema ==========")
        dofile("tests/jsonschema.test.lua")
        print("\n")
        print("========== logger ==========")
        dofile("tests/logger.test.lua")
        print("\n")
        print("========== requests ==========")
        dofile("tests/requests.test.lua")
        print("\n")
        print("========== sqlite ==========")
        dofile("tests/sqlite.test.lua")
        print("\n")
        print("========== uuid ==========")
        dofile("tests/uuid.test.lua")
        print("\n")
    end)
    if not ____try then
        ____catch(____hasReturned)
    end
end
