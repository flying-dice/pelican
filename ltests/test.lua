package.path = package.path .. ";.\\ltests\\?.lua"
package.cpath = package.cpath .. ";.\\target\\debug\\?.dll"

-- Load individual test files
print("========== test_json.lua ==========")
dofile("./ltests/test_json.lua")
print("\n")

print("========== test_uuid.lua ==========")
dofile("./ltests/test_uuid.lua")
print("\n")

-- Run all tests
--lester.exit()