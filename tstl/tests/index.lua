--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]
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
        print("========== web ==========")
        dofile("tests/web.test.lua")
        print("\n")
    end)
    if not ____try then
        ____catch(____hasReturned)
    end
end
