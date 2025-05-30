import {logger} from "pelican";
import {describe, expect, it} from "lester";

describe("logger", () => {
    describe("Logger", () => {
        it("should create a logger with a namespace", () => {
            const my_logger = logger.Logger("testNamespace");
            expect.equal(type(my_logger), "userdata");
        });

        it("should log debug messages", () => {
            const my_logger = logger.Logger("testDebug");
            expect.not_fail(() => {
                my_logger.debug("This is a debug message");
            });
        });

        it("should log info messages", () => {
            const my_logger = logger.Logger("testInfo");
            expect.not_fail(() => {
                logger.info("This is an info message");
            });
        });

        it("should log warn messages", () => {
            const my_logger = logger.Logger("testWarn");
            expect.not_fail(() => {
                my_logger.warn("This is a warning message");
            });
        });

        it("should log error messages", () => {
            const my_logger = logger.Logger("testError");
            expect.not_fail(() => {
                my_logger.error("This is an error message");
            });
        });
    });
});
