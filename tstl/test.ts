PELICAN = {
    logger_level: "debug",
}

print("========== Running Lua Tests ==========")

// @ts-ignore
package.cpath = package.cpath + ";..\\target\\debug\\?.dll"

try {
    // Load individual test files
    print("========== test_uuid.lua ==========")
    dofile("./uuid.test.lua")
    print("\n")

    print("========== test_json.lua ==========")
    dofile("./json.test.lua")
    print("\n")
} catch (e) {
    print(`An error occurred while running tests: ${e}`)
    print(debug.traceback())
}
