--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]
local ____exports = {}
local ____pelican = require("pelican")
local logger = ____pelican.logger
logger.debug("This is a debug message")
logger.info("This is an info message")
logger.warn("This is a warning message")
logger.error("This is an error message")
local myLogger = logger.Logger("PELICAN.TEST.INFO")
local _myLogger = logger.Logger("PELICAN.TEST")
myLogger:debug("This is a namespaced debug message")
_myLogger:info("This is a namespaced info message")
_myLogger:warn("This is a namespaced warning message")
_myLogger:error("This is a namespaced error message")
return ____exports
