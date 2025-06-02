--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]
local ____exports = {}
local ____emmy_core = require("emmy_core")
local tcpListen = ____emmy_core.tcpListen
local waitIDE = ____emmy_core.waitIDE
print("Starting Emmy Core Debugger...")
tcpListen("localhost", 9966)
print("Debugger is listening on localhost:9966. Waiting for IDE connection...")
waitIDE()
return ____exports
