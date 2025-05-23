package.path = package.path .. ";.\\ltests\\?.lua"
package.cpath = package.cpath .. ";.\\target\\debug\\?.dll"

---@type pelican
local P = require("pelican")
local lester = require('lester')

local describe, it, expect = lester.describe, lester.it, lester.expect

describe("requests.get", function()
    it("should succeed with get request", function()
        local res, err = P.requests.get("https://jsonplaceholder.typicode.com/posts/1")
        expect.equal(type(res), "userdata")
        expect.equal(res:get_status(), 200)
        expect.equal(res:get_header_value("Content-Type"), "application/json; charset=utf-8")
        expect.equal(res:get_text(), [[{
  "userId": 1,
  "id": 1,
  "title": "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
  "body": "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto"
}]])
        expect.equal(res:get_json(), {
            userId = 1,
            id = 1,
            title = "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
            body = "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto"
        })
    end)
end)

lester.report()