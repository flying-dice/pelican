import { jsonschema } from "pelican";
import { describe, expect, it } from "lester";

describe("jsonschema", () => {
    it("should successfully instantiate using the new keyword", () => {
        const validator = new jsonschema.Validator({ type: "string" });

        expect.equal(tostring(validator), 'Validator({"type":"string"})');
    });

    it("should successfully instantiate using the static new method", () => {
        const validator = jsonschema.Validator.new({ type: "string" });

        expect.equal(tostring(validator), 'Validator({"type":"string"})');
    });

    it("should validate a string against a string schema", () => {
        const validator = new jsonschema.Validator({ type: "string" });
        const [isValid, error] = validator.validate("Hello World");

        expect.equal(isValid, true);
        expect.equal(error, undefined);
    });

    it("should validate a number against a number schema", () => {
        const validator = new jsonschema.Validator({ type: "number" });
        const [isValid, error] = validator.validate(42);

        expect.equal(isValid, true);
        expect.equal(error, undefined);
    });

    it("should fail to validate a number against a string schema", () => {
        const validator = new jsonschema.Validator({ type: "string" });
        const [isValid, error] = validator.validate(42);

        expect.equal(isValid, false);
        expect.equal(error, '42 is not of type "string"');
    });

    it("should validate an object against an object schema", () => {
        const validator = new jsonschema.Validator({
            type: "object",
            properties: {
                name: { type: "string" },
                age: { type: "number" },
            },
            required: ["name", "age"],
        });
        const [isValid, error] = validator.validate({ name: "John", age: 30 });

        expect.equal(isValid, true);
        expect.equal(error, undefined);
    });

    it("should fail to compile an invalid schema", () => {
        try {
            new jsonschema.Validator({ type: "invalidType" });
            assert(false, "Expected an error to be thrown for invalid schema");
        } catch (e) {
            expect.equal(
                `${string.match(`${e}`, "([^\r\n]+)")[0]}`,
                `"invalidType" is not valid under any of the schemas listed in the 'anyOf' keyword`,
            );
        }
    });
});
