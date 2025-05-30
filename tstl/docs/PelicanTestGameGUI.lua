-- %USERPROFILE%\Saved Games\DCS\Scripts\Hooks\PelicanTestGameGUI.lua

package.cpath = package.cpath .. ";" .. lfs.writedir() .. "\\Mods\\tech\\Pelican\\bin\\?.dll"

local status, ____pelican = pcall(require, "pelican")
if not status then
    log.write("PELICAN", log.ERROR, "Failed to load Pelican: " .. tostring(____pelican))
    return
end

local name = ____pelican.name
local version = ____pelican.version
local web = ____pelican.web
local logger = ____pelican.logger

logger.info("Pelican Running...")

log.write("PELICAN", log.INFO, "Pelican Running...")
log.write("PELICAN", log.INFO, "Pelican Name: " .. name)
log.write("PELICAN", log.INFO, "Pelican Version: " .. version)

local server = web.serve(
        {
            host = "127.0.0.1",
            port = 1234
        }
)

local router = web.router()

router:add_method(
        "ping",
        function(params)
            local param = params[1]
            return { message = "pong " .. param }
        end
)

local user_callbacks = {}

function user_callbacks.onSimulationFrame()
    local result, err = server:process_rpc(router)
    if not result then
        log.error("PELICAN", log.INFO, "RPC error: " .. tostring(err))
    end
end

DCS.setUserCallbacks(user_callbacks)
