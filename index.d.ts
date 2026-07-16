declare namespace ParseTorrentTitle {

    interface ParserOptions {
        /** Whether to skip a matcher if another matcher from this group was already found */
        skipIfAlreadyFound?: boolean;
        /** Whether to exclude found match from the end result title */
        skipFromTitle?: boolean;
        /** Whether to skip this matcher if there are no other groups matched before it's matchIndex */
        skipIfFirst?: boolean;
        /** Whether to skip this matcher if it appears before specified matcher group in the name */
        skipIfBefore?: Array<string>;
        /** Whether to remove the found match from further matchers */
        remove?: boolean;
    }

    interface DefaultParserResult {
        title: string;
        arc?: string;
        date?: string;
        year?: number | string;
        resolution?: string;
        extended?: boolean;
        unrated?: boolean;
        proper?: boolean;
        repack?: boolean;
        convert?: boolean;
        hardcoded?: boolean;
        retail?: boolean;
        remastered?: boolean;
        complete?: boolean;
        region?: string;
        container?: string;
        extension?: string;
        source?: string;
        codec?: string;
        bitDepth?: string;
        hdr?: Array<string>;
        threeD?: string;
        audio?: string;
        group?: string;
        batch?: boolean;
        isBatch?: boolean;
        isMovie?: boolean;
        episodeRangeStart?: number;
        episodeRangeEnd?: number;
        absoluteRangeHint?: { start: number; end: number };
        volumes?: Array<number>;
        seasons?: Array<number>;
        season?: number;
        episodes?: Array<number>;
        episode?: number;
        languages?: string;
        dubbed?: boolean;
    }

    interface Handler<ParserResult = DefaultParserResult> {
        (input: { title: string, result: ParserResult, matched: Array<string> }): void;
        (input: { title: string }): void;
        (input: { result: ParserResult }): void;
    }

    interface ParseFunction<ParserResult = DefaultParserResult> {
        (title: string): ParserResult;
    }

    interface Transformer {
        (input: string): any;
    }

    interface AddHandlerFunction<ParserResult = DefaultParserResult> {
        (handlerName: string, handler: RegExp, transformer?: Transformer, options?: ParserOptions): void;
        (handlerName: string, handler: RegExp, transformer?: Transformer): void;
        (handlerName: string, handler: RegExp, options?: ParserOptions): void;
        (handlerName: string, handler: Handler<ParserResult>): void;
        (handler: Handler<ParserResult>): void;
    }

    interface AddDefaultsFunction {
        (parser: Parser): void;
    }

    class Parser<ParserResult = DefaultParserResult> {

        constructor();

        addHandler: AddHandlerFunction<ParserResult>;
        parse: ParseFunction<ParserResult>;
    }
}

declare module "parse-torrent-title" {

    export interface DefaultParserResult extends ParseTorrentTitle.DefaultParserResult { }
    export class Parser<ParserResult = DefaultParserResult> extends ParseTorrentTitle.Parser<ParserResult> { }
    export const parse: ParseTorrentTitle.ParseFunction;
    export const addHandler: ParseTorrentTitle.AddHandlerFunction;
    export const addDefaults: ParseTorrentTitle.AddDefaultsFunction;

    /** Normalize a parsed resolution value (e.g. "4k") to its display label ("4K"). */
    export function normalizeResolution(resolution?: string | null): string | null;
    /** Normalize a parsed codec value (e.g. "hevc") to its display label ("HEVC"). */
    export function normalizeCodec(codec?: string | null): string | null;
    /** Extract an episode number from a batch-torrent filename; null if none found. */
    export function parseFilename(filename?: string | null): number | null;
}
