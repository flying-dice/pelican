import { web } from "pelican";

PELICAN = {
    logger_level: "debug",
};

const server = web.serve({
    port: 1234,
    host: "localhost",
});

const router = web.router();

router.add_method("ping", (props: { message: string }) => {
    return `Pong, ${props.message}!`;
});

while (true) {
    server.process_rpc(router);
}
