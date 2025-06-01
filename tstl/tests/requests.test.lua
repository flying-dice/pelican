local ____lualib = require("lualib_bundle")
local __TS__SourceMapTraceBack = ____lualib.__TS__SourceMapTraceBack
__TS__SourceMapTraceBack(debug.getinfo(1).short_src, {["5"] = 1,["6"] = 1,["7"] = 1,["8"] = 2,["9"] = 2,["10"] = 2,["11"] = 2,["12"] = 4,["13"] = 4,["14"] = 4,["15"] = 5,["16"] = 5,["17"] = 5,["18"] = 6,["19"] = 6,["20"] = 6,["21"] = 7,["22"] = 8,["23"] = 9,["24"] = 9,["25"] = 9,["26"] = 9,["27"] = 10,["28"] = 10,["29"] = 10,["30"] = 10,["31"] = 6,["32"] = 6,["33"] = 5,["34"] = 5,["35"] = 19,["36"] = 19,["37"] = 19,["38"] = 20,["39"] = 22,["40"] = 22,["41"] = 22,["42"] = 23,["43"] = 24,["44"] = 25,["45"] = 25,["46"] = 25,["47"] = 25,["48"] = 26,["49"] = 26,["50"] = 26,["51"] = 26,["52"] = 22,["53"] = 22,["54"] = 34,["55"] = 34,["56"] = 34,["57"] = 35,["58"] = 38,["59"] = 39,["60"] = 39,["61"] = 39,["62"] = 39,["63"] = 40,["64"] = 40,["65"] = 40,["66"] = 40,["67"] = 34,["68"] = 34,["69"] = 48,["70"] = 48,["71"] = 48,["72"] = 49,["73"] = 49,["74"] = 49,["75"] = 49,["76"] = 49,["77"] = 54,["78"] = 55,["79"] = 55,["80"] = 55,["81"] = 55,["82"] = 48,["83"] = 48,["84"] = 58,["85"] = 58,["86"] = 58,["87"] = 59,["88"] = 59,["89"] = 59,["90"] = 59,["91"] = 59,["92"] = 64,["93"] = 65,["94"] = 65,["95"] = 65,["96"] = 65,["97"] = 58,["98"] = 58,["99"] = 68,["100"] = 68,["101"] = 68,["102"] = 69,["103"] = 70,["104"] = 71,["105"] = 71,["106"] = 71,["107"] = 71,["108"] = 68,["109"] = 68,["110"] = 74,["111"] = 74,["112"] = 74,["115"] = 80,["116"] = 81,["117"] = 81,["118"] = 80,["121"] = 77,["122"] = 78,["128"] = 74,["129"] = 74,["130"] = 19,["131"] = 19,["132"] = 88,["133"] = 88,["134"] = 88,["135"] = 89,["136"] = 89,["137"] = 89,["138"] = 90,["139"] = 91,["140"] = 91,["141"] = 91,["142"] = 91,["143"] = 92,["144"] = 93,["145"] = 94,["146"] = 94,["147"] = 94,["148"] = 94,["149"] = 95,["150"] = 95,["151"] = 95,["152"] = 95,["153"] = 89,["154"] = 89,["155"] = 88,["156"] = 88,["157"] = 4,["158"] = 4});
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
                        local response, err = client:post(
                            "https://jsonplaceholder.typicode.com/posts",
                            json.encode({title = "foo", body = "bar", userId = 1}),
                            {headers = {["Content-Type"] = "application/json"}}
                        )
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
                        local response, err = client:put(
                            "https://jsonplaceholder.typicode.com/posts/1",
                            json.encode({id = 1, title = "updated title", body = "updated body", userId = 1}),
                            {headers = {["Content-Type"] = "application/json"}}
                        )
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
                                expect.equal(true, false)
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
