print("========== Running Lua Examples ==========")

// @ts-ignore
package.cpath = package.cpath + ";..\\target\\debug\\?.dll"

try {
    dofile("./json.example.lua")
    dofile("./uuid.example.lua")
    dofile("./logger.example.lua")
} catch (e) {
    print(`An error occurred while running examples: ${e}`)
    print(debug.traceback())
}