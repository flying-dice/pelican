--- Hello World!!
--- @module pelican

-- ==============================
-- JSON Module
-- ==============================

---@class pelican.json
json = {}

--- Encodes a Lua value into a JSON string.
---@param value any
---@return string
function json.encode(value)
end

--- Decodes a JSON string into a Lua value.
---@param json string
---@return any
function json.decode(json)
end

-- ==============================
-- Logger Module
-- ==============================

---@class pelican.logger.Logger
---@field debug fun(self: pelican.logger.Logger, message: string): void
---@field info fun(self: pelican.logger.Logger, message: string): void
---@field warn fun(self: pelican.logger.Logger, message: string): void
---@field error fun(self: pelican.logger.Logger, message: string): void
local Logger = {}

--- Constructs a new Logger instance.
---@param namespace string
---@return pelican.logger.Logger
function Logger:new(namespace)
end

---@class pelican.logger
logger = {}

---@param namespace string
---@return pelican.logger.Logger
function logger.Logger(namespace)
end

---@param message string
function logger.debug(message)
end

---@param message string
function logger.info(message)
end

---@param message string
function logger.warn(message)
end

---@param message string
function logger.error(message)
end

-- ==============================
-- Requests Module
-- ==============================

---@class pelican.requests.HttpRequestOptions
---@field headers table<string, string>|nil
---@field timeout number|nil

---@class pelican.requests.HttpResponse
---@field get_status fun(self: pelican.requests.HttpResponse): number
---@field get_headers fun(self: pelican.requests.HttpResponse): table<string, string>
---@field get_header_value fun(self: pelican.requests.HttpResponse, name: string): string|nil
---@field get_text fun(self: pelican.requests.HttpResponse): string
---@field get_json fun(self: pelican.requests.HttpResponse): any

---@class pelican.requests.BlockingHttpClient
local BlockingHttpClient = {}

---@return pelican.requests.BlockingHttpClient
function BlockingHttpClient.new()
end

---@param url string
---@param options pelican.requests.HttpRequestOptions|nil
---@return pelican.requests.HttpResponse, string|nil
function BlockingHttpClient:get(url, options)
end

---@param url string
---@param body string|nil
---@param options pelican.requests.HttpRequestOptions|nil
---@return pelican.requests.HttpResponse, string|nil
function BlockingHttpClient:post(url, body, options)
end

---@param url string
---@param body string|nil
---@param options pelican.requests.HttpRequestOptions|nil
---@return pelican.requests.HttpResponse, string|nil
function BlockingHttpClient:put(url, body, options)
end

---@param url string
---@param options pelican.requests.HttpRequestOptions|nil
---@return pelican.requests.HttpResponse, string|nil
function BlockingHttpClient:delete(url, options)
end

---@class pelican.requests
requests = {}

---@param url string
---@return pelican.requests.HttpResponse, string|nil
function requests.get(url)
end

-- ==============================
-- SQLite Module
-- ==============================

---@class pelican.sqlite.SqliteConnection
---@field exec fun(self: pelican.sqlite.SqliteConnection, sql: string): boolean, string|nil
---@field execute fun(self: pelican.sqlite.SqliteConnection, sql: string, params: any|nil): table<any>, string|nil
local SqliteConnection = {}

---@class pelican.sqlite
sqlite = {}

---@param path string
---@return pelican.sqlite.SqliteConnection
function sqlite.open(path)
end

-- ==============================
-- UUID Module
-- ==============================

---@class pelican.uuid
uuid = {}

---@return string
function uuid.v4()
end

---@return string
function uuid.v7()
end
