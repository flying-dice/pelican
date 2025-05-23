package.path = package.path .. ";.\\ltests\\?.lua"
package.cpath = package.cpath .. ";.\\target\\debug\\?.dll"

-- Load individual test files
print("========== test_json.lua ==========")
dofile("./ltests/test_json.lua")
print("\n")

print("========== test_uuid.lua ==========")
dofile("./ltests/test_uuid.lua")
print("\n")

print("========== test_requests.lua ==========")
dofile("./ltests/test_requests.lua")
print("\n")

print("========== test_json-schema.lua ==========")
dofile("./ltests/test_json-schema.lua")
print("\n")

print("========== test_sqlite.lua ==========")
dofile("./ltests/test_sqlite.lua")
print("\n")

print("========== test_collections.lua ==========")
dofile("./ltests/test_collections.lua")
print("\n")

-- Run all tests
--lester.exit()