---@meta

---@class pelican.uuid
local uuid = {}

---Generates a new UUID v4 string.
---This function creates a random UUID (Universally Unique Identifier) version 4.
---```lua
---local uuid = require("pelican.uuid")
---local new_uuid = uuid.v4()
---print(new_uuid)  -- 1816df21-f203-42e7-befa-be64d5454dab
---```
---@return string
function uuid.v4() end

return uuid
