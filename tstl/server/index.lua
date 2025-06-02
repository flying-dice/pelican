--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]
package.cpath = package.cpath .. ";..\\target\\debug\\?.dll"
do
    local function ____catch(e)
        print("An error occurred while running tests: " .. tostring(e))
        print(debug.traceback())
    end
    local ____try, ____hasReturned = pcall(function()
        print("========== server ==========")
        dofile("tests/jsonrpc.server.lua")
        print("\n")
    end)
    if not ____try then
        ____catch(____hasReturned)
    end
end
