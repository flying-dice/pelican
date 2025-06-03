import { jsonschema, sqlite, web } from "pelican";

const users_db = new sqlite.SQLiteConnection(":memory:");

type User = {
    id: number;
    name: string;
    age: number;
};

users_db.exec("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER);");

const user_validator = new jsonschema.Validator({
    type: "object",
    properties: {
        id: { type: "integer" },
        name: { type: "string" },
        age: { type: "integer", minimum: 18 },
    },
    required: ["name", "age"],
});

export function add_users(router: web.Router): void {
    router.add_method("get_users", () => {
        const [res] = users_db.execute<User>("SELECT * FROM users;");
        return res;
    });

    router.add_method("get_user", (props: { id: number }) => {
        const [res] = users_db.execute<User>("SELECT * FROM users WHERE id = ?", [props.id]);
        if (!res[0]) {
            throw new Error(`User with ID ${props.id} not found.`);
        }
        return res[0];
    });

    router.add_method("add_user", (props: { name: string; age: number }) => {
        const [valid, err] = user_validator.validate(props);
        if (!valid) {
            throw new Error(`Invalid user data: ${err}`);
        }

        users_db.execute("INSERT INTO users (name, age) VALUES (?, ?);", [props.name, props.age]);

        const [res] = users_db.execute<User>("SELECT * FROM users WHERE name = ?;", [props.name]);

        if (!res[0]) {
            throw new Error(`Failed to add user: ${props.name}`);
        }

        return res[0];
    });

    router.add_method("delete_all_users", () => {
        users_db.exec("DELETE FROM users;");
        return true;
    });
}
