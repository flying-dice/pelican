-- %USERPROFILE%\Saved Games\DCS\Scripts\PelicanTestMission.lua

package.cpath = package.cpath .. ";" .. lfs.writedir() .. "\\Mods\\tech\\Pelican\\bin\\?.dll"

local status, ____pelican = pcall(require, 'pelican')
if not status then
    env.error("Failed to load Pelican: " .. tostring(____pelican))
    return
end

local name = ____pelican.name
local version = ____pelican.version
local web = ____pelican.web
local logger = ____pelican.logger

logger.info("Pelican Running...")

env.info("Pelican Running...")
env.info("Pelican Name: " .. name)
env.info("Pelican Version: " .. version)

local server = web.serve({
    host = "127.0.0.1",
    port = 1235,
})

local router = web.router()

router:add_method("ping", function(params)
    local param = params[1]
    return { message = "pong " .. param }
end)

timer.scheduleFunction(function(arg, time)
    local result, err = server:process_rpc(router)
    if not result then
        env.error("RPC error: " .. tostring(err))
    end

    return timer.getTime() + .01
end, nil, timer.getTime() + .01)