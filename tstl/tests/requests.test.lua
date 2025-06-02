local ____lualib = require("lualib_bundle")
local __TS__SourceMapTraceBack = ____lualib.__TS__SourceMapTraceBack
__TS__SourceMapTraceBack(debug.getinfo(1).short_src, {["5"] = 1,["6"] = 1,["7"] = 1,["8"] = 2,["9"] = 2,["10"] = 2,["11"] = 2,["12"] = 4,["13"] = 4,["14"] = 4,["15"] = 5,["16"] = 5,["17"] = 5,["18"] = 6,["19"] = 6,["20"] = 6,["21"] = 7,["22"] = 8,["23"] = 9,["24"] = 9,["25"] = 9,["26"] = 9,["27"] = 10,["28"] = 10,["29"] = 10,["30"] = 10,["31"] = 6,["32"] = 6,["33"] = 5,["34"] = 5,["35"] = 19,["36"] = 19,["37"] = 19,["38"] = 20,["39"] = 22,["40"] = 22,["41"] = 22,["42"] = 23,["43"] = 24,["44"] = 25,["45"] = 25,["46"] = 25,["47"] = 25,["48"] = 26,["49"] = 26,["50"] = 26,["51"] = 26,["52"] = 22,["53"] = 22,["54"] = 34,["55"] = 34,["56"] = 34,["57"] = 35,["58"] = 38,["59"] = 39,["60"] = 39,["61"] = 39,["62"] = 39,["63"] = 40,["64"] = 40,["65"] = 40,["66"] = 40,["67"] = 34,["68"] = 34,["69"] = 48,["70"] = 48,["71"] = 48,["72"] = 49,["73"] = 50,["74"] = 53,["75"] = 54,["76"] = 54,["77"] = 54,["78"] = 54,["79"] = 48,["80"] = 48,["81"] = 57,["82"] = 57,["83"] = 57,["84"] = 58,["85"] = 59,["86"] = 62,["87"] = 63,["88"] = 63,["89"] = 63,["90"] = 63,["91"] = 57,["92"] = 57,["93"] = 66,["94"] = 66,["95"] = 66,["96"] = 67,["97"] = 68,["98"] = 69,["99"] = 69,["100"] = 69,["101"] = 69,["102"] = 66,["103"] = 66,["104"] = 72,["105"] = 72,["106"] = 72,["109"] = 78,["110"] = 79,["111"] = 79,["112"] = 78,["115"] = 75,["116"] = 76,["122"] = 72,["123"] = 72,["124"] = 85,["125"] = 85,["126"] = 85,["129"] = 90,["130"] = 91,["131"] = 91,["132"] = 90,["135"] = 87,["136"] = 88,["142"] = 85,["143"] = 85,["144"] = 19,["145"] = 19,["146"] = 98,["147"] = 98,["148"] = 98,["149"] = 99,["150"] = 99,["151"] = 99,["152"] = 100,["153"] = 101,["154"] = 101,["155"] = 101,["156"] = 101,["157"] = 102,["158"] = 103,["159"] = 104,["160"] = 104,["161"] = 104,["162"] = 104,["163"] = 105,["164"] = 105,["165"] = 105,["166"] = 105,["167"] = 99,["168"] = 99,["169"] = 98,["170"] = 98,["171"] = 4,["172"] = 4});
local ____exports = {}
local ____pelican = require("pelican")
local json = ____pelican.json
local requests = ____pelican.requests
local ____lester = require("tests.lester")
local describe = ____lester.describe
local expect = ____lester.expect
local it = ____lester.it
describe(
    "requests",
    function()
        describe(
            "get",
            function()
                it(
                    "should make a GET request and return the response",
                    function()
                        local response, err = requests.get("https://jsonplaceholder.typicode.com/posts/1")
                        expect.equal(err, nil)
                        expect.equal(
                            response:get_status(),
                            200
                        )
                        expect.equal(
                            response:get_json(),
                            {userId = 1, id = 1, title = "sunt aut facere repellat provident occaecati excepturi optio reprehenderit", body = "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto"}
                        )
                    end
                )
            end
        )
        describe(
            "BlockingHttpClient",
            function()
                local client = requests.BlockingHttpClient:new()
                it(
                    "should make a GET request using the client (with no headers) and return the response",
                    function()
                        local response, err = client:get("https://jsonplaceholder.typicode.com/posts/1")
                        expect.equal(err, nil)
                        expect.equal(
                            response:get_status(),
                            200
                        )
                        expect.equal(
                            response:get_json(),
                            {userId = 1, id = 1, title = "sunt aut facere repellat provident occaecati excepturi optio reprehenderit", body = "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto"}
                        )
                    end
                )
                it(
                    "should make a GET request using the client and return the response",
                    function()
                        local response, err = client:get("https://jsonplaceholder.typicode.com/posts/1", {headers = {["Content-Type"] = "application/json"}})
                        expect.equal(err, nil)
                        expect.equal(
                            response:get_status(),
                            200
                        )
                        expect.equal(
                            response:get_json(),
                            {userId = 1, id = 1, title = "sunt aut facere repellat provident occaecati excepturi optio reprehenderit", body = "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto"}
                        )
                    end
                )
                it(
                    "should make a POST request using the client and return the response",
                    function()
                        local body = json.encode({title = "foo", body = "bar", userId = 1})
                        local response, err = client:post("https://jsonplaceholder.typicode.com/posts", body, {headers = {["Content-Type"] = "application/json"}})
                        expect.equal(err, nil)
                        expect.equal(
                            response:get_status(),
                            201
                        )
                    end
                )
                it(
                    "should make a PUT request using the client and return the response",
                    function()
                        local body = json.encode({id = 1, title = "updated title", body = "updated body", userId = 1})
                        local response, err = client:put("https://jsonplaceholder.typicode.com/posts/1", body, {headers = {["Content-Type"] = "application/json"}})
                        expect.equal(err, nil)
                        expect.equal(
                            response:get_status(),
                            200
                        )
                    end
                )
                it(
                    "should make a DELETE request using the client and return the response",
                    function()
                        local response, err = client:delete("https://jsonplaceholder.typicode.com/posts/1")
                        expect.equal(err, nil)
                        expect.equal(
                            response:get_status(),
                            200
                        )
                    end
                )
                it(
                    "should fail to parse invalid headers",
                    function()
                        do
                            local function ____catch(e)
                                expect.equal((string.match(
                                    tostring(e),
                                    "([^\r\n]+)"
                                )), "bad argument #3 to `BlockingHttpClient.get`: invalid type: integer `1`, expected a map")
                            end
                            local ____try, ____hasReturned = pcall(function()
                                client:get("https://jsonplaceholder.typicode.com/posts/1", {headers = 1})
                                assert(false, "Expected an error to be thrown for invalid headers")
                            end)
                            if not ____try then
                                ____catch(____hasReturned)
                            end
                        end
                    end
                )
                it(
                    "should raise runtime error for invalid URL pattern",
                    function()
                        do
                            local function ____catch(e)
                                expect.equal((string.match(
                                    tostring(e),
                                    "([^\r\n]+)"
                                )), "bad argument #2 to `BlockingHttpClient.get`: relative URL without a base")
                            end
                            local ____try, ____hasReturned = pcall(function()
                                client:get("invalid-url")
                                assert(false, "Expected an error to be thrown for invalid URL")
                            end)
                            if not ____try then
                                ____catch(____hasReturned)
                            end
                        end
                    end
                )
            end
        )
        describe(
            "BlockingHttpClient (From Constructor)",
            function()
                it(
                    "should create a new BlockingHttpClient instance",
                    function()
                        local client = requests.BlockingHttpClient:new()
                        expect.equal(
                            type(client),
                            "userdata"
                        )
                        local response, err = client:get("https://jsonplaceholder.typicode.com/posts/1")
                        expect.equal(err, nil)
                        expect.equal(
                            response:get_status(),
                            200
                        )
                        expect.equal(
                            response:get_json(),
                            {userId = 1, id = 1, title = "sunt aut facere repellat provident occaecati excepturi optio reprehenderit", body = "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto"}
                        )
                    end
                )
            end
        )
    end
)
return ____exports
