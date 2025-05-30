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
        declare function encode(value: any): string;

        declare function decode(json: string): any;
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
        /** @customConstructor logger.Logger */
        declare class Logger {
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

        declare function Logger(namespace: string): Logger;
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
            static new(): BlockingHttpClient;

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
        declare class SqliteConnection {
            exec(sql: string): LuaMultiReturn<[boolean, string | undefined]>;

            execute<T = any, P = any>(sql: string, params?: P): LuaMultiReturn<[T[], string | undefined]>;
        }

        declare function open(path: string): SqliteConnection;
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
