---@meta

---@class pelican.requests
local requests = {}

---@class pelican.requests.Response
local Response = {}

---Gets the HTTP status code.
---@return integer
function Response:get_status() end

---Gets all headers as a table.
---@return table
function Response:get_headers() end

---Gets a specific header value.
---@param key string
---@return string
function Response:get_header_value(key) end

---Gets the response body as text.
---@return string
function Response:get_text() end

---Gets the response body as JSON.
---@return any
function Response:get_json() end

---Performs a blocking GET request.
---@param url string
---@return pelican.requests.Response
function requests.get(url) end

return requests
