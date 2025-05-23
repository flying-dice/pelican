---@meta

---@class pelican.sqlite
local sqlite = {}

---@class pelican.sqlite.Connection
local Connection = {}

---Executes a SQL query and returns true on success. This does not support Bind Parameters.
---@param query string
---@return boolean|nil, nil|string # Returns true on success, or (false, error_message) on failure.
function Connection:exec(query) end

---Executes a SQL query and returns the results. This supports Bind Parameters.
---@param query string
---@return table|nil, nil|string # Returns a table of results or (nil, error_message) on failure.
function Connection:execute(query) end


---Opens a SQLite database file and returns a connection.
---@param path string
---@return pelican.sqlite.Connection
function sqlite.open(path) end

return sqlite