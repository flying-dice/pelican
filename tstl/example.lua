--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]
print("========== Running Lua Examples ==========")
package.cpath = package.cpath .. ";..\\target\\debug\\?.dll"
do
    local function ____catch(e)
        print("An error occurred while running examples: " .. tostring(e))
        print(debug.traceback())
    end
    local ____try, ____hasReturned = pcall(function()
        dofile("./json.example.lua")
        dofile("./uuid.example.lua")
        dofile("./logger.example.lua")
    end)
    if not ____try then
        ____catch(____hasReturned)
    end
end
