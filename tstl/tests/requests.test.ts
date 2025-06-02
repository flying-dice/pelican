import { json, requests } from "pelican";
import { describe, expect, it } from "lester";

describe("requests", () => {
    describe("get", () => {
        it("should make a GET request and return the response", () => {
            const [response, err] = requests.get("https://jsonplaceholder.typicode.com/posts/1");
            expect.equal(err, null);
            expect.equal(response.get_status(), 200);
            expect.equal(response.get_json(), {
                userId: 1,
                id: 1,
                title: "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
                body: "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto",
            });
        });
    });

    describe("BlockingHttpClient", () => {
        const client = requests.BlockingHttpClient.new();

        it("should make a GET request using the client (with no headers) and return the response", () => {
            const [response, err] = client.get("https://jsonplaceholder.typicode.com/posts/1");
            expect.equal(err, null);
            expect.equal(response.get_status(), 200);
            expect.equal(response.get_json(), {
                userId: 1,
                id: 1,
                title: "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
                body: "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto",
            });
        });

        it("should make a GET request using the client and return the response", () => {
            const [response, err] = client.get("https://jsonplaceholder.typicode.com/posts/1", {
                headers: { "Content-Type": "application/json" },
            });
            expect.equal(err, null);
            expect.equal(response.get_status(), 200);
            expect.equal(response.get_json(), {
                userId: 1,
                id: 1,
                title: "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
                body: "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto",
            });
        });

        it("should make a POST request using the client and return the response", () => {
            const [body] = json.encode({ title: "foo", body: "bar", userId: 1 });
            const [response, err] = client.post("https://jsonplaceholder.typicode.com/posts", body, {
                headers: { "Content-Type": "application/json" },
            });
            expect.equal(err, null);
            expect.equal(response.get_status(), 201);
        });

        it("should make a PUT request using the client and return the response", () => {
            const [body] = json.encode({ id: 1, title: "updated title", body: "updated body", userId: 1 });
            const [response, err] = client.put("https://jsonplaceholder.typicode.com/posts/1", body, {
                headers: { "Content-Type": "application/json" },
            });
            expect.equal(err, null);
            expect.equal(response.get_status(), 200);
        });

        it("should make a DELETE request using the client and return the response", () => {
            const [response, err] = client.delete("https://jsonplaceholder.typicode.com/posts/1");
            expect.equal(err, null);
            expect.equal(response.get_status(), 200);
        });

        it("should fail to parse invalid headers", () => {
            try {
                // @ts-ignore
                client.get("https://jsonplaceholder.typicode.com/posts/1", { headers: 1 });
                assert(false, "Expected an error to be thrown for invalid headers");
            } catch (e) {
                expect.equal(
                    `${string.match(`${e}`, "([^\r\n]+)")[0]}`,
                    "bad argument #3 to `BlockingHttpClient.get`: invalid type: integer `1`, expected a map",
                );
            }
        });

        it("should raise runtime error for invalid URL pattern", () => {
            try {
                client.get("invalid-url");
                assert(false, "Expected an error to be thrown for invalid URL");
            } catch (e) {
                expect.equal(
                    `${string.match(`${e}`, "([^\r\n]+)")[0]}`,
                    "bad argument #2 to `BlockingHttpClient.get`: relative URL without a base",
                );
            }
        });
    });

    describe("BlockingHttpClient (From Constructor)", () => {
        it("should create a new BlockingHttpClient instance", () => {
            const client = new requests.BlockingHttpClient();
            expect.equal(type(client), "userdata");
            const [response, err] = client.get("https://jsonplaceholder.typicode.com/posts/1");
            expect.equal(err, null);
            expect.equal(response.get_status(), 200);
            expect.equal(response.get_json(), {
                userId: 1,
                id: 1,
                title: "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
                body: "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto",
            });
        });
    });
});
