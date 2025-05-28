/**
 * Hello World!!
 *
 * @summary This is the pelican module.
 *
 * @module
 */
declare module "pelican" {

    /**
     * A UUID is a unique 128-bit value, stored as 16 octets, and regularly formatted as a hex string in five groups. UUIDs are used to assign unique identifiers to entities without requiring a central allocating authority.
     *
     * They are particularly useful in distributed systems, though can be used in disparate areas, such as databases and network protocols. Typically, a UUID is displayed in a readable string form as a sequence of hexadecimal digits, separated into groups by hyphens.
     *
     * The uniqueness property is not strictly guaranteed; however, for all practical purposes, it can be assumed that an unintentional collision would be extremely unlikely.
     *
     * ### Which UUID version should I use?
     * If you just want to generate unique identifiers, then consider version 4 (v4) UUIDs. If you want to use UUIDs as database keys or need to sort them then consider version 7 (v7) UUIDs. Other versions are not supported and should generally be avoided unless there’s an existing need for them.
     *
     * ## TypeScript Example
     * {@includeCode ../uuid.example.ts}
     *
     * ## Lua Example
     * {@includeCode ../uuid.example.lua}
     *
     * @summary Provides functions for generating UUIDs (Universally Unique Identifiers).
     */
    declare namespace uuid {

        declare function v4(): string;

        declare function v7(): string;
    }


    /**
     * The `json` module provides functions to encode Lua tables into JSON strings and decode JSON strings back into Lua tables.
     *
     * This module is useful for serializing data structures for storage or transmission in a format that is widely used and understood.
     *
     * ## TypeScript Example
     * {@includeCode ../json.example.ts}
     *
     * ## Lua Example
     * {@includeCode ../json.example.lua}
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
     * This module is useful for debugging and monitoring your Lua applications by providing a consistent way to log messages with timestamps and severity levels.
     *
     * It supports namespaced logging, allowing you to create loggers with specific namespaces for better organization of log messages.
     *
     * ## TypeScript Example
     * {@includeCode ../logger.example.ts}
     *
     * ## Lua Example
     * {@includeCode ../logger.example.lua}
     *
     * @summary Provides a simple logging utility for Lua applications.
     *
     * @noSelf
     */
    declare namespace logger {
        declare function debug(message: string): void;

        declare function info(message: string): void;

        declare function warn(message: string): void;

        declare function error(message: string): void;
        
        declare function Logger(namespace: string): Logger;

        /** @customConstructor logger.Logger */
        declare class Logger {
            constructor(namespace: string);

            debug(message: string): void;

            info(message: string): void;

            warn(message: string): void;

            error(message: string): void;
        }
    }
}

