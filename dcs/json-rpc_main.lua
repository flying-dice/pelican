local usercallbacks = {}

function usercallbacks.onSimulationFrame()
    net.dostring_in("mission", [[a_do_script("__loop()")]])
end

function usercallbacks.onSimulationStop()
    log.info("[EXAMPLE] - Stopping JSON-RPC Server...")
    net.dostring_in("mission", [[a_do_script("__stop()")]])
end

DCS.setUserCallbacks(usercallbacks)