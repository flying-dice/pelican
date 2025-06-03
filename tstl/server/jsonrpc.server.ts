import { web } from "pelican";
import { add_users } from "./users";

PELICAN = {
    logger_level: "error",
};

const server = web.serve({
    port: 1234,
    host: "localhost",
});

const router = web.router();

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
