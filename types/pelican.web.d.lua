---@meta

---@class pelican.web
local web = {}

---@class pelican.web.Server
local Server = {}

---Processes queued RPC requests.
---@param router pelican.web.Router
---@return boolean
function Server:process_rpc(router) end

---Stops the server.
---@param graceful? boolean
function Server:stop(graceful) end

---Stops the server asynchronously.
---@param graceful boolean
function Server:async_stop(graceful) end

---@class pelican.web.Router
local Router = {}

---Adds a method to the router.
---@param name string
---@param callback fun(params: any): any
function Router:add_method(name, callback) end

---Creates a new server.
---@param config {host: string, port: integer}
---@return pelican.web.Server
function web.serve(config) end

---Creates a new router.
---@return pelican.web.Router
function web.router() end

return web
