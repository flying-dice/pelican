// @ts-ignore
package.cpath = package.cpath + ";..\\target\\debug\\?.dll";

try {
    print("========== server ==========");
    dofile("server/jsonrpc.server.lua");
    print("\n");
} catch (e) {
    print(`An error occurred while running tests: ${e}`);
    print(debug.traceback());
}
