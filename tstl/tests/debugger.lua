local ____lualib = require("lualib_bundle")
local __TS__SourceMapTraceBack = ____lualib.__TS__SourceMapTraceBack
__TS__SourceMapTraceBack(debug.getinfo(1).short_src, {["5"] = 1,["6"] = 1,["7"] = 1,["8"] = 3,["9"] = 4,["10"] = 6,["11"] = 7});
local ____exports = {}
local ____emmy_core = require("emmy_core")
local tcpListen = ____emmy_core.tcpListen
local waitIDE = ____emmy_core.waitIDE
print("Starting Emmy Core Debugger...")
tcpListen("localhost", 9966)
print("Debugger is listening on localhost:9966. Waiting for IDE connection...")
waitIDE()
return ____exports
