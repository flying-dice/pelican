/** @noSelfInFile **/

/** lester.d.ts - TypeScript declarations for Lester test framework (TSTL) */

/** @noSelf **/
declare module "lester" {
    /** @noSelf **/
    interface Lester {
        quiet: boolean;
        color: boolean;
        show_traceback: boolean;
        show_error: boolean;
        stop_on_fail: boolean;
        utf8term: boolean;
        filter: string;
        seconds: () => number;
        colors: {
            reset: string;
            bright: string;
            red: string;
            green: string;
            yellow: string;
            blue: string;
            magenta: string;
        };

        parse_args(args?: string[]): void;

        describe(name: string, func: () => void): void;

        it(name: string, func: () => void, enabled?: boolean): void;

        before(func: (testName: string) => void): void;

        after(func: (testName: string) => void): void;

        report(): boolean;

        exit(): never;

        expect: LesterExpect;
    }

    /** @noSelf **/
    interface LesterExpect {
        tohumanstring(value: any): string;

        fail(func: () => void, expected?: any): void;

        not_fail(func: () => void): void;

        exist(value: any): void;

        not_exist(value: any): void;

        truthy(value: any): void;

        falsy(value: any): void;

        strict_eq(v1: any, v2: any, name?: string): [true] | [false, string];

        equal(v1: any, v2: any): void;

        not_equal(v1: any, v2: any): void;
    }

    /** @noSelf **/
    const lester: Lester;
    export = lester;
}
