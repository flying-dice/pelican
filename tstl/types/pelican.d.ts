/**
 * Hello World!!
 *
 * @summary This is the pelican module.
 *
 * @module
 */
declare module "pelican" {
    /**
     * The `json` module provides functions to encode Lua tables into JSON strings and decode JSON strings back into Lua tables.
     *
     * ## TypeScript Example
     * {@includeCode ../tests/json.test.ts}
     *
     * ## Lua Example
     * {@includeCode ../tests/json.test.lua}
     *
     * @summary Provides functions for encoding and decoding JSON in Lua.
     *
     * @noSelf
     */
    declare namespace json {
        declare function encode<T = any>(value: T): LuaMultiReturn<[string, string | undefined]>;

        declare function decode<T = any>(json: string): LuaMultiReturn<[T, string | undefined]>;
    }

    /**
     * The `json_schema` module provides functions to validate Lua tables against JSON schemas.
     *
     * ## TypeScript Example
     * {@includeCode ../tests/jsonschema.test.ts}
     *
     * ## Lua Example
     * {@includeCode ../tests/jsonschema.test.lua}
     *
     * @summary Provides functions for validating Lua tables against JSON schemas.
     *
     * @noSelf
     */
    declare namespace jsonschema {
        /**
         * @customConstructor jsonschema.Validator.new
         */
        declare class Validator<T> {
            static new<T>(this: void, schema: T): Validator<T>;

            constructor(schema: T): Validator<T>;

            validate(data: any): LuaMultiReturn<[boolean, string | undefined]>;
        }
    }

    /**
     * The `logger` module provides a simple logging utility for Lua, allowing you to log messages at different levels (debug, info, warn, error).
     *
     * ## TypeScript Example
     * {@includeCode ../tests/logger.test.ts}
     *
     * ## Lua Example
     * {@includeCode ../tests/logger.test.lua}
     *
     * @summary Provides a simple logging utility for Lua applications.
     *
     * @noSelf
     */
    declare namespace logger {
        /** @customConstructor logger.Logger.new */
        declare class Logger {
            static new(this: void, namespace: string): Logger;

            constructor(namespace: string);

            debug(message: string): void;

            info(message: string): void;

            warn(message: string): void;

            error(message: string): void;
        }

        declare function debug(message: string): void;

        declare function info(message: string): void;

        declare function warn(message: string): void;

        declare function error(message: string): void;
    }

    /**
     * The `requests` module provides functions to make HTTP requests.
     *
     * ## TypeScript Example
     * {@includeCode ../tests/requests.test.ts}
     *
     * ## Lua Example
     * {@includeCode ../tests/requests.test.lua}
     *
     * @summary Provides functions for making HTTP requests.
     * @noSelf
     */
    declare namespace requests {
        declare type HttpRequestOptions = {
            headers?: Record<string, string>;
            timeout?: number;
        };

        declare interface HttpResponse {
            get_status(): number;

            get_headers(): Record<string, string>;

            get_header_value(name: string): string | undefined;

            get_text(): string;

            get_json<T = any>(): T;
        }

        /** @customConstructor requests.BlockingHttpClient:new */
        declare class BlockingHttpClient {
            static new(this: void): BlockingHttpClient;

            get(url: string, options?: HttpRequestOptions): LuaMultiReturn<[HttpResponse, string | undefined]>;

            post(
                url: string,
                body?: string,
                options?: HttpRequestOptions,
            ): LuaMultiReturn<[HttpResponse, string | undefined]>;

            put(
                url: string,
                body?: string,
                options?: HttpRequestOptions,
            ): LuaMultiReturn<[HttpResponse, string | undefined]>;

            delete(url: string, options?: HttpRequestOptions): LuaMultiReturn<[HttpResponse, string | undefined]>;
        }

        declare function get(url: string): LuaMultiReturn<[HttpResponse, string | undefined]>;
    }

    /**
     * The `sqlite` module provides functions to interact with SQLite databases.
     *
     * ## TypeScript Example
     * {@includeCode ../tests/sqlite.test.ts}
     *
     * ## Lua Example
     * {@includeCode ../tests/sqlite.test.lua}
     *
     * @summary Provides functions for interacting with SQLite databases.
     * @noSelf
     */
    declare namespace sqlite {
        /**
         * @customConstructor sqlite.SQLiteConnection.new
         */
        declare class SQLiteConnection {
            static new(this: void, path: string): SQLiteConnection;

            constructor(path: string);

            exec(sql: string): LuaMultiReturn<[boolean, string | undefined]>;

            execute<T = any, P = any>(sql: string, params?: P): LuaMultiReturn<[T[], string | undefined]>;
        }
    }

    /**
     * The `uuid` module provides functions to generate UUIDs (Universally Unique Identifiers).
     *
     * ## TypeScript Example
     * {@includeCode ../tests/uuid.test.ts}
     *
     * ## Lua Example
     * {@includeCode ../tests/uuid.test.lua}
     *
     * @summary Provides functions for generating UUIDs (Universally Unique Identifiers).
     *
     * @noSelf
     */
    declare namespace uuid {
        declare function v4(): string;

        declare function v7(): string;
    }
}
