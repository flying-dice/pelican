--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]
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
                local client = requests.BlockingHttpClient.new()
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
