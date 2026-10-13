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
        /** Whether to skip this matcher if it appears after specified matcher group in the name */
        skipIfAfter?: Array<string>;
        /** Whether to remove the found match from further matchers */
        remove?: boolean;
        /** Value to set instead of the transformed match */
        value?: any;
    }

    interface DefaultParserResult {
        title: string;
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
        extras?: boolean;
        region?: string;
        container?: string;
        extension?: string;
        source?: string;
        codec?: string;
        bitDepth?: string;
        hdr?: Array<string>;
        threeD?: string;
        audio?: string;
        audioChannels?: string;
        group?: string;
        site?: string;
        volumes?: Array<number>;
        seasons?: Array<number>;
        season?: number;
        episodes?: Array<number>;
        episode?: number;
        halfEpisode?: boolean;
        episodeCode?: string;
        languages?: Array<string>;
        dubbed?: boolean;
    }

    interface Match {
        rawMatch?: string;
        matchIndex: number;
    }

    interface HandlerContext<ParserResult = DefaultParserResult> {
        /** The name being parsed, without the matches removed by previous handlers */
        title: string;
        result: ParserResult;
        /** Matches found so far, keyed by the field name */
        matched: { [field: string]: Match };
    }

    interface HandlerResult extends Match {
        /** Whether to remove the match from the name for further handlers */
        remove?: boolean;
        /** Whether to exclude the match from the end result title */
        skipFromTitle?: boolean;
    }

    interface Handler<ParserResult = DefaultParserResult> {
        (context: HandlerContext<ParserResult>): HandlerResult | null | void;
    }

    interface ParseFunction<ParserResult = DefaultParserResult> {
        (title: string): ParserResult;
    }

    interface Transformer {
        (input: string, existingValue?: any): any;
    }

    interface Transformers {
        none: Transformer;
        value: (value: any) => Transformer;
        integer: Transformer;
        boolean: Transformer;
        lowercase: Transformer;
        uppercase: Transformer;
        date: (dateFormat: string | Array<string>) => Transformer;
        range: Transformer;
        list: Transformer;
        rangeUpTo: Transformer;
        yearRange: Transformer;
        array: (chain?: Transformer) => Transformer;
        uniqConcat: (chain: Transformer) => Transformer;
    }

    interface AddHandlerFunction<ParserResult = DefaultParserResult> {
        (handlerName: string, handler: RegExp, transformer?: Transformer, options?: ParserOptions): void;
        (handlerName: string, handler: RegExp, options?: ParserOptions): void;
        (handlerName: string, handler: Handler<ParserResult>): void;
        (handler: Handler<ParserResult>): void;
    }

    interface AddDefaultsFunction {
        (parser: Parser<any>): void;
    }

    class Parser<ParserResult = DefaultParserResult> {

        constructor();

        addHandler: AddHandlerFunction<ParserResult>;
        parse: ParseFunction<ParserResult>;
    }
}

declare module "parse-torrent-title" {

    export interface DefaultParserResult extends ParseTorrentTitle.DefaultParserResult { }
    export type ParserOptions = ParseTorrentTitle.ParserOptions;
    export type Handler<ParserResult = DefaultParserResult> = ParseTorrentTitle.Handler<ParserResult>;
    export type HandlerContext<ParserResult = DefaultParserResult> = ParseTorrentTitle.HandlerContext<ParserResult>;
    export type HandlerResult = ParseTorrentTitle.HandlerResult;
    export type Transformer = ParseTorrentTitle.Transformer;
    export class Parser<ParserResult = DefaultParserResult> extends ParseTorrentTitle.Parser<ParserResult> { }
    export const parse: ParseTorrentTitle.ParseFunction;
    export const addHandler: ParseTorrentTitle.AddHandlerFunction;
    export const addDefaults: ParseTorrentTitle.AddDefaultsFunction;
    export const Transformers: ParseTorrentTitle.Transformers;
}
