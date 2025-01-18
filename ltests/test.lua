package.path = package.path .. ";.\\ltests\\?.lua"
package.cpath = package.cpath .. ";.\\target\\debug\\?.dll"

-- Load individual test files
print("========== test_decode.lua ==========")
dofile("./ltests/test_decode.lua")

print("\n")

print("========== test_encode.lua ==========")
dofile("./ltests/test_encode.lua")

print("\n")

print("========== test_server.lua ==========")
dofile("./ltests/test_server.lua")

-- Run all tests
--lester.exit()