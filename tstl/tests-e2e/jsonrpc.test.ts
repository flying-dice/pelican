import { describe, it, expect } from "vitest";
import axios from "axios";
import * as crypto from "node:crypto";

describe("jsonrpc", () => {
    describe("health", () => {
        it("should return 200 OK", async () => {
            const response = await axios.get("http://localhost:1234/health");
            expect(response.status).toBe(200);
            expect(response.data).toEqual({
                name: "pelican",
                version: "0.3.0",
                status: "OK",
            });
        });
    });

    describe("http", () => {
        const httpClient = axios.create({
            baseURL: "http://localhost:1234",
            headers: {
                "Content-Type": "application/json",
            },
            validateStatus: null,
        });

        it("should perform rpc request", async () => {
            const rpcRequest = {
                jsonrpc: "2.0",
                method: "ping",
                params: { message: "World" },
                id: crypto.randomUUID(),
            };

            const response = await httpClient.post("/rpc", rpcRequest);
            expect(response.status).toBe(200);
            expect(response.data).toEqual({
                jsonrpc: "2.0",
                id: rpcRequest.id,
                result: "Pong, World!",
            });
        });

        it("should handle notification", () => {
            const rpcNotification = {
                jsonrpc: "2.0",
                method: "ping",
                params: { message: "World" },
            };

            return httpClient.post("/rpc", rpcNotification).then((response) => {
                expect(response.status).toBe(202);
                expect(response.data).toEqual("OK");
            });
        });

        it("should handle missing method", async () => {
            const rpcRequest = {
                jsonrpc: "2.0",
                id: crypto.randomUUID(),
                method: "nonExistentMethod",
            };
            const response = await httpClient.post("/rpc", rpcRequest);
            expect(response.status).toBe(200);
            expect(response.data).toEqual({
                jsonrpc: "2.0",
                id: rpcRequest.id,
                error: {
                    code: -32601,
                    message: "Method not found: nonExistentMethod",
                },
            });
        });

        it("should handle server errors", async () => {
            const rpcRequest = {
                jsonrpc: "2.0",
                method: "throws", // This method is designed to throw an error See server/jsonrpc.server.ts
                params: [],
                id: crypto.randomUUID(),
            };

            // Simulate a server error by modifying the server to throw an error
            const response = await httpClient.post("/rpc", rpcRequest);
            expect(response.status).toBe(200);
            expect(response.data).toEqual({
                jsonrpc: "2.0",
                id: rpcRequest.id,
                error: {
                    code: -32603,
                    data: "runtime error: Error: This is an error from the server.",
                    message: "LuaError",
                },
            });
        });

        it("should handle invalid JSON-RPC request", async () => {
            const response = await httpClient.post("/rpc", "Invalid JSON");
            expect(response.status).toBe(400);
            expect(response.data).toEqual(
                'Json deserialize error: invalid type: string "Invalid JSON", expected struct JsonRpcRequest at line 1 column 14',
            );
        });
    });

    describe("websocket", () => {
        it("performs an RPC request over websocket", async () => {
            const ws = new WebSocket("ws://localhost:1234/ws");

            const rpcRequest = {
                jsonrpc: "2.0",
                method: "ping",
                params: { message: "World" },
                id: crypto.randomUUID(),
            };

            const response = await new Promise((resolve, reject) => {
                ws.onopen = () => {
                    ws.send(JSON.stringify(rpcRequest));
                };

                ws.onmessage = (event) => {
                    try {
                        resolve(JSON.parse(event.data));
                    } catch (err) {
                        reject(err);
                    }
                };

                ws.onerror = (err) => reject(err);
            });

            expect(response).toMatchObject({
                jsonrpc: "2.0",
                id: rpcRequest.id,
                result: "Pong, World!",
            });
        });
    });
});
