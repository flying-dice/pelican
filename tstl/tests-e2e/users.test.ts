import { describe, it, expect, beforeEach } from "vitest";
import axios from "axios";
import * as crypto from "node:crypto";

describe("users", () => {
    const httpClient = axios.create({
        baseURL: "http://localhost:1234",
        headers: {
            "Content-Type": "application/json",
        },
        validateStatus: null,
    });

    beforeEach(async () => {
        await httpClient.post("/rpc", {
            jsonrpc: "2.0",
            method: "delete_all_users",
            id: crypto.randomUUID(),
        });
    });

    it("should get all users", async () => {
        const rpcRequest = {
            jsonrpc: "2.0",
            method: "get_users",
            id: crypto.randomUUID(),
        };

        const response = await httpClient.post("/rpc", rpcRequest);
        expect(response.status).toBe(200);
        expect(response.data).toEqual({
            jsonrpc: "2.0",
            id: rpcRequest.id,
            result: {},
        });

        await httpClient.post("/rpc", {
            jsonrpc: "2.0",
            method: "add_user",
            params: { name: "Alice", age: 30 },
            id: crypto.randomUUID(),
        });

        await httpClient.post("/rpc", {
            jsonrpc: "2.0",
            method: "add_user",
            params: { name: "Brian", age: 35 },
            id: crypto.randomUUID(),
        });

        const responseAfterAddingUsers = await httpClient.post("/rpc", rpcRequest);
        expect(responseAfterAddingUsers.status).toBe(200);
        expect(responseAfterAddingUsers.data).toEqual({
            jsonrpc: "2.0",
            id: rpcRequest.id,
            result: [
                { id: 1, name: "Alice", age: 30 },
                { id: 2, name: "Brian", age: 35 },
            ],
        });
    });

    it("should error on invalid user", () => {
        const rpcRequest = {
            jsonrpc: "2.0",
            method: "add_user",
            params: { name: "Charlie", age: 17 }, // Invalid age
            id: crypto.randomUUID(),
        };

        return httpClient.post("/rpc", rpcRequest).then((response) => {
            expect(response.status).toBe(200);
            expect(response.data).toEqual({
                jsonrpc: "2.0",
                id: rpcRequest.id,
                error: {
                    code: -32603,
                    data: "runtime error: Error: Invalid user data: 17 is less than the minimum of 18",
                    message: "LuaError",
                },
            });
        });
    });

    it("should add a user", async () => {
        const rpcRequest = {
            jsonrpc: "2.0",
            method: "add_user",
            params: { name: "Alice", age: 30 },
            id: crypto.randomUUID(),
        };

        const response = await httpClient.post("/rpc", rpcRequest);
        expect(response.status).toBe(200);
        expect(response.data).toEqual({
            jsonrpc: "2.0",
            id: rpcRequest.id,
            result: { id: 1, name: "Alice", age: 30 },
        });
    });

    it("should get a user by ID", async () => {
        await httpClient.post("/rpc", {
            jsonrpc: "2.0",
            method: "add_user",
            params: { name: "Alice", age: 30 },
            id: crypto.randomUUID(),
        });

        const rpcRequest = {
            jsonrpc: "2.0",
            method: "get_user",
            params: { id: 1 },
            id: crypto.randomUUID(),
        };

        const response = await httpClient.post("/rpc", rpcRequest);
        expect(response.status).toBe(200);
        expect(response.data).toEqual({
            jsonrpc: "2.0",
            id: rpcRequest.id,
            result: { id: 1, name: "Alice", age: 30 },
        });
    });

    it("should handle user not found", async () => {
        const rpcRequest = {
            jsonrpc: "2.0",
            method: "get_user",
            params: { id: 999 },
            id: crypto.randomUUID(),
        };

        const response = await httpClient.post("/rpc", rpcRequest);
        expect(response.status).toBe(200);
        expect(response.data).toEqual({
            jsonrpc: "2.0",
            id: rpcRequest.id,
            error: {
                code: -32603,
                data: "runtime error: Error: User with ID 999 not found.",
                message: "LuaError",
            },
        });
    });
});
