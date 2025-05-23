---@meta

---@class pelican.jsonschema
local jsonschema = {}

---@class pelican.jsonschema.Validator
local Validator = {}

---Validates a value against the schema.
---@param value any
---@return boolean|nil, nil, string
function Validator:validate(value) end

---Creates a validator for a schema.
---@param schema table
---@return pelican.jsonschema.Validator
function jsonschema.validator_for(schema) end

return jsonschema
