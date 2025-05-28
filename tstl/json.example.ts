import {json} from "pelican";

const luaTable = {name: "John", age: 30, isActive: true};
const jsonString = json.encode(luaTable);

assert(jsonString === '{"isActive":true,"name":"John","age":30}');

const decodedTable = json.decode(jsonString);
assert(decodedTable.name === "John");