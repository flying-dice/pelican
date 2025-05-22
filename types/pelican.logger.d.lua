---@meta

---@class pelican.logger
local logger = {}

---Initializes the logger.
---@param config {level: string, pattern: string, file?: string}
function logger.init_config(config) end

---Logs a debug message.
---@param msg string
function logger.debug(msg) end

---Logs an info message.
---@param msg string
function logger.info(msg) end

---Logs a warning message.
---@param msg string
function logger.warn(msg) end

---Logs an error message.
---@param msg string
function logger.error(msg) end

return logger
