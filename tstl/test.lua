--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]
PELICAN = {logger_level = "debug"}
print("========== Running Lua Tests ==========")
package.cpath = package.cpath .. ";..\\target\\debug\\?.dll"
do
    local function ____catch(e)
        print("An error occurred while running tests: " .. tostring(e))
        print(debug.traceback())
    end
    local ____try, ____hasReturned = pcall(function()
        print("========== test_uuid.lua ==========")
        dofile("./uuid.test.lua")
        print("\n")
        print("========== test_json.lua ==========")
        dofile("./json.test.lua")
        print("\n")
    end)
    if not ____try then
        ____catch(____hasReturned)
    end
end
