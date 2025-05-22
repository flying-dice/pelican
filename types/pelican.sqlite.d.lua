---@meta

---@class pelican.sqlite
local sqlite = {}

---@class pelican.sqlite.Connection
local Connection = {}

---Executes a SQL query.
---@param query string
---@return boolean|nil, nil|string # Returns true on success, or (false, error_message) on failure.
function Connection:execute(query) end

---Executes a SQL query and returns the results.
---@param query string
---@return table|nil, nil|string # Returns a table of results or (nil, error_message) on failure.
function Connection:query(query) end


---Opens a SQLite database file and returns a connection.
---@param path string
---@return pelican.sqlite.Connection
function sqlite.open(path) end

return sqlite