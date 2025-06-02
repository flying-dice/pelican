import { sqlite } from "pelican";
import { describe, expect, it } from "lester";

type User = {
    id: number;
    name: string;
    age: number;
};

describe("sqlite", () => {
    describe("SQLiteConnection", () => {
        it("should instantiate using the new keyword", () => {
            const connection = sqlite.SQLiteConnection.new(":memory:");
            expect.equal(tostring(connection), "SQLiteConnection(:memory:)");
        });
    });

    const connection = sqlite.SQLiteConnection.new(":memory:");
    connection.exec(
        "CREATE TABLE IF NOT EXISTS users(id   INTEGER PRIMARY KEY, name TEXT UNIQUE, age  INTEGER CHECK ( typeof(age) = 'integer'));",
    );
    connection.exec("INSERT INTO users (name, age) VALUES ('John Doe', 30);");
    connection.exec("INSERT INTO users (name, age) VALUES ('Jane Smith', 25);");

    describe("exec", () => {
        it("executes a valid INSERT OR REPLACE statement", () => {
            const [res] = connection.exec("INSERT OR REPLACE INTO users (name, age) VALUES ('Jane Smith', 25);");
            expect.equal(res, true);
        });

        it("returns an error for malformed SQL", () => {
            const [res, err] = connection.exec("INVALID SQL");
            expect.equal(res, undefined);
            expect.equal(err, 'SQLite error: near "INVALID": syntax error (code 1)');
        });

        it("returns an error when violating UNIQUE constraint", () => {
            const [, err] = connection.exec("INSERT INTO users (name, age) VALUES ('Jane Smith', 40);");
            expect.truthy(err?.includes("UNIQUE constraint failed"));
        });
    });

    describe("execute", () => {
        it("returns all users from SELECT query", () => {
            const [res] = connection.execute<User>("SELECT * FROM users;");
            expect.equal(res.length, 2);
            expect.equal(res[0].name, "John Doe");
            expect.equal(res[1].name, "Jane Smith");
        });

        it("returns filtered users using positional bind", () => {
            const [res] = connection.execute<User, [number]>("SELECT * FROM users WHERE age = ?;", [30]);
            expect.equal(res.length, 1);
            expect.equal(res[0].name, "John Doe");
        });

        it("inserts a new user using positional binds and validates insert", () => {
            const [res] = connection.execute<User, [string, number]>("INSERT INTO users (name, age) VALUES (?, ?);", [
                "Alice",
                66,
            ]);
            expect.equal(res.length, 0);

            const [selectRes] = connection.execute<User>("SELECT * FROM users WHERE age > 65;");
            expect.equal(selectRes.length, 1);
            expect.equal(selectRes[0].name, "Alice");
        });

        it("returns user using named parameter", () => {
            const [res] = connection.execute<User, Pick<User, "name">>("SELECT * FROM users WHERE name = :name;", {
                name: "John Doe",
            });
            expect.equal(res.length, 1);
            expect.equal(res[0].name, "John Doe");
        });

        it("returns empty result for non-matching WHERE clause", () => {
            const [res] = connection.execute<User, [number]>("SELECT * FROM users WHERE age = ?;", [99]);
            expect.equal(res.length, 0);
        });

        it("returns error for invalid SQL statement", () => {
            const [res, err] = connection.execute("INVALID SQL");
            expect.equal(res, undefined);
            expect.equal(err, 'SQLite error: near "INVALID": syntax error (code 1)');
        });

        it("should fail to insert a user with NaN age", () => {
            connection.execute("INSERT INTO users (name, age) VALUES (?, ?);", ["Bob", "NaN"]);
            const [qres] = connection.execute<User>("SELECT * FROM users WHERE name = 'Bob';");
            expect.equal(qres.length, 0);
            expect.equal(qres[0], undefined);
        });
    });
});
