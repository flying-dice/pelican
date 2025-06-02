import { uuid } from "pelican";
import { describe, expect, it } from "lester";

describe("uuid", () => {
    it("should produce a valid uuid V4", () => {
        const uuidv4 = uuid.v4();
        print(`Generated uuid v4: ${uuidv4}`);

        expect.equal(type(uuidv4), "string");
        expect.equal(string.len(uuidv4), 36);
    });

    it("should produce a valid uuid V7", () => {
        const uuidv7 = uuid.v7();
        print(`Generated uuid v7: ${uuidv7}`);

        expect.equal(type(uuidv7), "string");
        expect.equal(string.len(uuidv7), 36);
    });
});
