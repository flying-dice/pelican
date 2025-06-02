import { json } from "pelican";
import { describe, expect, it } from "lester";

describe("json", () => {
    it("should successfully encode an object", () => {
        const luaTable = { isActive: true, name: "John", age: 30 };
        const [jsonString] = json.encode(luaTable);

        expect.equal(type(jsonString), "string");
        expect.equal(jsonString, '{"age":30,"isActive":true,"name":"John"}');
    });

    it("should should successfully encode an array", () => {
        const luaArray = ["a", "b", "c", "d", "e"];
        const [jsonString] = json.encode(luaArray);

        expect.equal(type(jsonString), "string");
        expect.equal(jsonString, '["a","b","c","d","e"]');
    });

    it("should successfully encode a string", () => {
        const luaString = "Hello World";
        const [jsonString] = json.encode(luaString);

        expect.equal(type(jsonString), "string");
        expect.equal(jsonString, '"Hello World"');
    });

    it("should successfully encode an integer", () => {
        const luaInteger = 42;
        const [jsonString] = json.encode(luaInteger);

        expect.equal(type(jsonString), "string");
        expect.equal(jsonString, "42");
    });

    it("should successfully encode a boolean", () => {
        const luaBoolean = true;
        const [jsonString] = json.encode(luaBoolean);

        expect.equal(type(jsonString), "string");
        expect.equal(jsonString, "true");
    });

    it("should successfully encode a nil value", () => {
        const luaNil = null; // Lua nil is represented as null in JSON
        const [jsonString] = json.encode(luaNil);

        expect.equal(type(jsonString), "string");
        expect.equal(jsonString, "null");
    });

    it("should successfully decode a JSON string to a Lua table", () => {
        const jsonString = '{"name":"John","age":30,"isActive":true}';
        const [luaTable] = json.decode(jsonString);

        expect.equal(type(luaTable), "table");
        expect.equal(luaTable.name, "John");
        expect.equal(luaTable.age, 30);
        expect.equal(luaTable.isActive, true);
    });

    it("should return err for malfored json", () => {
        const jsonString = '{"name": "John", "age": 30, "isActive": true'; // Missing closing brace
        const [luaTable, err] = json.decode(jsonString);
        expect.equal(luaTable, undefined);
        expect.equal(err, "EOF while parsing an object at line 1 column 44");
    });
});
