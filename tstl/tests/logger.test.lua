local ____lualib = require("lualib_bundle")
local __TS__SourceMapTraceBack = ____lualib.__TS__SourceMapTraceBack
__TS__SourceMapTraceBack(debug.getinfo(1).short_src, {["5"] = 1,["6"] = 1,["7"] = 2,["8"] = 2,["9"] = 2,["10"] = 2,["11"] = 4,["12"] = 4,["13"] = 4,["14"] = 5,["15"] = 5,["16"] = 5,["17"] = 6,["18"] = 5,["19"] = 5,["20"] = 9,["21"] = 9,["22"] = 9,["23"] = 10,["24"] = 9,["25"] = 9,["26"] = 13,["27"] = 13,["28"] = 13,["29"] = 14,["30"] = 13,["31"] = 13,["32"] = 17,["33"] = 17,["34"] = 17,["35"] = 18,["36"] = 17,["37"] = 17,["38"] = 21,["39"] = 21,["40"] = 21,["41"] = 22,["42"] = 22,["43"] = 22,["44"] = 23,["45"] = 24,["46"] = 24,["47"] = 24,["48"] = 24,["49"] = 22,["50"] = 22,["51"] = 27,["52"] = 27,["53"] = 27,["54"] = 28,["55"] = 29,["56"] = 29,["57"] = 29,["58"] = 29,["59"] = 27,["60"] = 27,["61"] = 32,["62"] = 32,["63"] = 32,["64"] = 33,["65"] = 34,["66"] = 34,["67"] = 34,["68"] = 34,["69"] = 32,["70"] = 32,["71"] = 37,["72"] = 37,["73"] = 37,["74"] = 38,["75"] = 39,["76"] = 40,["77"] = 39,["78"] = 37,["79"] = 37,["80"] = 44,["81"] = 44,["82"] = 44,["83"] = 45,["84"] = 46,["85"] = 47,["86"] = 46,["87"] = 44,["88"] = 44,["89"] = 51,["90"] = 51,["91"] = 51,["92"] = 52,["93"] = 53,["94"] = 54,["95"] = 53,["96"] = 51,["97"] = 51,["98"] = 58,["99"] = 58,["100"] = 58,["101"] = 59,["102"] = 60,["103"] = 61,["104"] = 60,["105"] = 58,["106"] = 58,["107"] = 21,["108"] = 21,["109"] = 4,["110"] = 4});
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
            "debug",
            function()
                logger.debug("This is a debug message")
            end
        )
        describe(
            "info",
            function()
                logger.info("This is an info message")
            end
        )
        describe(
            "warn",
            function()
                logger.warn("This is a warning message")
            end
        )
        describe(
            "error",
            function()
                logger.error("This is an error message")
            end
        )
        describe(
            "Logger",
            function()
                it(
                    "should instantiate logger using new keyword",
                    function()
                        local my_logger = logger.Logger.new("testNew")
                        expect.equal(
                            tostring(my_logger),
                            "Logger(testNew)"
                        )
                    end
                )
                it(
                    "should instantiate logger using Logger function",
                    function()
                        local my_logger = logger.Logger.new("testFunction")
                        expect.equal(
                            tostring(my_logger),
                            "Logger(testFunction)"
                        )
                    end
                )
                it(
                    "should create a logger with a namespace",
                    function()
                        local my_logger = logger.Logger.new("testNamespace")
                        expect.equal(
                            type(my_logger),
                            "userdata"
                        )
                    end
                )
                it(
                    "should log debug messages",
                    function()
                        local my_logger = logger.Logger.new("testDebug")
                        expect.not_fail(function()
                            my_logger:debug("This is a debug message")
                        end)
                    end
                )
                it(
                    "should log info messages",
                    function()
                        local my_logger = logger.Logger.new("testInfo")
                        expect.not_fail(function()
                            logger.info("This is an info message")
                        end)
                    end
                )
                it(
                    "should log warn messages",
                    function()
                        local my_logger = logger.Logger.new("testWarn")
                        expect.not_fail(function()
                            my_logger:warn("This is a warning message")
                        end)
                    end
                )
                it(
                    "should log error messages",
                    function()
                        local my_logger = logger.Logger.new("testError")
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
