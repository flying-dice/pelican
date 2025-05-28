import {uuid} from "pelican";

// Use uuid.v4() to generate a UUID version 4
const uuidv4 = uuid.v4()
assert(string.len(uuidv4) === 36)
assert(type(uuidv4) === "string");

// Use uuid.v7() to generate a UUID version 7
const uuidv7 = uuid.v7()
assert(string.len(uuidv7) === 36)
assert(type(uuidv7) === "string");
