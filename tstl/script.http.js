import http from "k6/http";
import { check } from "k6";

export default function () {
    const body = JSON.stringify({
        id: crypto.randomUUID(),
        jsonrpc: "2.0",
        method: "ping",
        params: { message: "Hello, World!" },
    });

    let res = http.post("http://127.0.0.1:1234/rpc", body, {
        headers: { "Content-Type": "application/json" },
    });

    check(res, {
        "response code was 200": (res) => res.status === 200,
        "response body contains 'pong'": (res) => res.json().result === "Pong, Hello, World!!",
    });
}
