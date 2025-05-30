PELICAN = {
    logger_level: "debug",
};

print("========== Running Lua Tests ==========");

// @ts-ignore
package.cpath = package.cpath + ";..\\target\\debug\\?.dll";

try {
    print("========== json ==========");
    dofile("tests/json.test.lua");
    print("\n");

    print("========== logger ==========");
    dofile("tests/logger.test.lua");
    print("\n");

    print("========== requests ==========");
    dofile("tests/requests.test.lua");
    print("\n");

    print("========== sqlite ==========");
    dofile("tests/sqlite.test.lua");
    print("\n");

    print("========== uuid ==========");
    dofile("tests/uuid.test.lua");
    print("\n");
} catch (e) {
    print(`An error occurred while running tests: ${e}`);
    print(debug.traceback());
}
