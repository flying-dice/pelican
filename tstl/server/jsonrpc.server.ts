import { jsonrpc } from "pelican";
import { add_users } from "./users";

PELICAN = {
    logger_level: "error",
};

const server = new jsonrpc.JsonRpcServer({
    port: 1234,
    host: "localhost",
});

const router = new jsonrpc.JsonRpcRouter();

add_users(router);
router.add_method("ping", (props: { message: string }) => {
    return `Pong, ${props.message}!`;
});

router.add_method("throws", () => {
    throw new Error("This is an error from the server.");
});

while (true) {
    server.process_rpc(router);
}
