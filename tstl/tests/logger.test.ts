import { logger } from "pelican";
import { describe, expect, it } from "lester";

describe("logger", () => {
    describe("debug", () => {
        logger.debug("This is a debug message");
    });

    describe("info", () => {
        logger.info("This is an info message");
    });

    describe("warn", () => {
        logger.warn("This is a warning message");
    });

    describe("error", () => {
        logger.error("This is an error message");
    });

    describe("Logger", () => {
        it("should instantiate logger using new keyword", () => {
            const my_logger = new logger.Logger("testNew");
            expect.equal(tostring(my_logger), "Logger(testNew)");
        });

        it("should instantiate logger using Logger function", () => {
            const my_logger = logger.Logger.new("testFunction");
            expect.equal(tostring(my_logger), "Logger(testFunction)");
        });

        it("should create a logger with a namespace", () => {
            const my_logger = new logger.Logger("testNamespace");
            expect.equal(type(my_logger), "userdata");
        });

        it("should log debug messages", () => {
            const my_logger = new logger.Logger("testDebug");
            expect.not_fail(() => {
                my_logger.debug("This is a debug message");
            });
        });

        it("should log info messages", () => {
            const my_logger = new logger.Logger("testInfo");
            expect.not_fail(() => {
                logger.info("This is an info message");
            });
        });

        it("should log warn messages", () => {
            const my_logger = new logger.Logger("testWarn");
            expect.not_fail(() => {
                my_logger.warn("This is a warning message");
            });
        });

        it("should log error messages", () => {
            const my_logger = new logger.Logger("testError");
            expect.not_fail(() => {
                my_logger.error("This is an error message");
            });
        });
    });
});
