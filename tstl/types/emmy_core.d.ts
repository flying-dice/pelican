/**
 * @hidden
 * @module
 */
declare module "emmy_core" {
    /**
     * The `tcpListen` function starts a TCP server that listens for incoming connections on the specified host and port.
     * @noSelf
     */
    declare function tcpListen(this: void, host: string, port: number): void;

    /**
     * The `waitIDE` function blocks the execution until an IDE debugger connects to the Lua script.
     * This is typically used in development environments to allow debugging of Lua scripts.
     * @noSelf
     */
    declare function waitIDE(this: void): void;
}
