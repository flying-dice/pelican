local ____lualib = require("lualib_bundle")
local __TS__SourceMapTraceBack = ____lualib.__TS__SourceMapTraceBack
__TS__SourceMapTraceBack(debug.getinfo(1).short_src, {["5"] = 1,["6"] = 1,["7"] = 2,["8"] = 2,["9"] = 2,["10"] = 2,["11"] = 4,["12"] = 4,["13"] = 4,["14"] = 5,["15"] = 5,["16"] = 5,["17"] = 6,["18"] = 6,["19"] = 6,["20"] = 7,["21"] = 8,["22"] = 8,["23"] = 8,["24"] = 8,["25"] = 6,["26"] = 6,["27"] = 11,["28"] = 11,["29"] = 11,["30"] = 12,["31"] = 13,["32"] = 14,["33"] = 13,["34"] = 11,["35"] = 11,["36"] = 18,["37"] = 18,["38"] = 18,["39"] = 19,["40"] = 20,["41"] = 21,["42"] = 20,["43"] = 18,["44"] = 18,["45"] = 25,["46"] = 25,["47"] = 25,["48"] = 26,["49"] = 27,["50"] = 28,["51"] = 27,["52"] = 25,["53"] = 25,["54"] = 32,["55"] = 32,["56"] = 32,["57"] = 33,["58"] = 34,["59"] = 35,["60"] = 34,["61"] = 32,["62"] = 32,["63"] = 5,["64"] = 5,["65"] = 4,["66"] = 4});
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
