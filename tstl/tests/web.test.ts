import { json, requests } from "pelican";
import { describe, expect, it } from "lester";

describe("web", () => {
    const router_client = new requests.BlockingHttpClient();

    it("should respond to RPC calls", () => {
        const [encoded] = json.encode({
            jsonrpc: "2.0",
            method: "ping",
            params: { message: "Tastic" },
            id: "1",
        });
        const [res, err] = router_client.post("http://localhost:1234/rpc", encoded);
        expect.equal(err, null);
        expect.equal(res.get_text(), `{"jsonrpc":"2.0","id":"1","result":"Pong, Tastic!"}`);
    });
});
