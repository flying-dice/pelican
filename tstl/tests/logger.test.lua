--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]
local ____exports = {}
local ____pelican = require("pelican")
local logger = ____pelican.logger
local ____lester = require("tests.lester")
local describe = ____lester.describe
local expect = ____lester.expect
local it = ____lester.it
describe(
    "logger",
    function()
        describe(
            "Logger",
            function()
                it(
                    "should create a logger with a namespace",
                    function()
                        local my_logger = logger.Logger("testNamespace")
                        expect.equal(
                            type(my_logger),
                            "userdata"
                        )
                    end
                )
                it(
                    "should log debug messages",
                    function()
                        local my_logger = logger.Logger("testDebug")
                        expect.not_fail(function()
                            my_logger:debug("This is a debug message")
                        end)
                    end
                )
                it(
                    "should log info messages",
                    function()
                        local my_logger = logger.Logger("testInfo")
                        expect.not_fail(function()
                            logger.info("This is an info message")
                        end)
                    end
                )
                it(
                    "should log warn messages",
                    function()
                        local my_logger = logger.Logger("testWarn")
                        expect.not_fail(function()
                            my_logger:warn("This is a warning message")
                        end)
                    end
                )
                it(
                    "should log error messages",
                    function()
                        local my_logger = logger.Logger("testError")
                        expect.not_fail(function()
                            my_logger:error("This is an error message")
                        end)
                    end
                )
            end
        )
    end
)
return ____exports
