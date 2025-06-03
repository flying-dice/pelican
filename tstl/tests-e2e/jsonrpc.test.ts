import { describe, it, expect } from "vitest";
import axios from "axios";

const rpcClient = axios.create({
    baseURL: "http://localhost:1234",
    headers: {
        "Content-Type": "application/json",
    },
    validateStatus: null,
});

describe("web", () => {
    describe("http", () => {
        it("should perform rpc request", async () => {
            const messageId = crypto.randomUUID();
            const body = {
                jsonrpc: "2.0",
                method: "ping",
                params: { message: "World" },
                id: messageId,
            };

            const response = await rpcClient.post("/rpc", body);
            expect(response.status).toBe(200);
            expect(response.data).toEqual({
                jsonrpc: "2.0",
                id: messageId,
                result: "Pong, World!",
            });
        });

        it("should handle notification", () => {
            const body = {
                jsonrpc: "2.0",
                method: "ping",
                params: { message: "World" },
            };

            return rpcClient.post("/rpc", body).then((response) => {
                expect(response.status).toBe(202);
                expect(response.data).toEqual("OK");
            });
        });

        it("should handle missing method", async () => {
            const messageId = crypto.randomUUID();
            const body = {
                jsonrpc: "2.0",
                id: messageId,
                method: "nonExistentMethod",
            };
            const response = await rpcClient.post("/rpc", body);
            expect(response.status).toBe(200);
            expect(response.data).toEqual({
                jsonrpc: "2.0",
                id: messageId,
                error: {
                    code: -32601,
                    message: "Method not found: nonExistentMethod",
                },
            });
        });

        it("should handle server errors", async () => {
            const messageId = crypto.randomUUID();
            const body = {
                jsonrpc: "2.0",
                method: "throws", // This method is designed to throw an error See server/jsonrpc.server.ts
                params: [],
                id: messageId,
            };

            // Simulate a server error by modifying the server to throw an error
            const response = await rpcClient.post("/rpc", body);
            expect(response.status).toBe(200);
            expect(response.data).toEqual({
                jsonrpc: "2.0",
                id: messageId,
                error: {
                    code: -32603,
                    data: "runtime error: Error: This is an error from the server.",
                    message: "LuaError",
                },
            });
        });

        it("should handle invalid JSON-RPC request", async () => {
            const response = await rpcClient.post("/rpc", "Invalid JSON");
            expect(response.status).toBe(400);
            expect(response.data).toEqual("Failed to parse request");
        });
    });
});
