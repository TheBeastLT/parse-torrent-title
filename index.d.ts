declare namespace ParseTorrentTitle {

    /** A category name, or a specific handler of a category by its 0-based index (negative counts from the end) */
    type HandlerTarget = string | { name: string, index?: number };

    interface PositionOptions {
        /** Insert before this handler, or before the first handler of this category */
        before?: HandlerTarget;
        /** Insert after this handler, or after the last handler of this category */
        after?: HandlerTarget;
    }

    interface ParserOptions extends PositionOptions {
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

    type HandlerResult = Match & {
        /** Whether to exclude the match from the end result title */
        skipFromTitle?: boolean;
    } & (
        { remove?: false } |
        /** Removing the match from the name for further handlers needs its text */
        { remove: true, rawMatch: string }
    );

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
        (handlerName: string, handler: Handler<ParserResult>, options?: PositionOptions): void;
        (handler: Handler<ParserResult>, options?: PositionOptions): void;
    }

    interface HandlerInfo {
        name: string;
        /** Index of the handler within its category */
        index: number;
        /** Index of the handler within all handlers of the parser */
        position: number;
        /** Only present for regular expression handlers */
        regExp?: RegExp;
        /** Only present for regular expression handlers, with all defaults filled in */
        options?: Required<Omit<ParserOptions, "before" | "after" | "value">> & Pick<ParserOptions, "value">;
    }

    type RemoveHandlersTarget = string | Array<string> | ((handler: HandlerInfo) => boolean);

    interface AddDefaultsOptions {
        /** Categories to leave out */
        exclude?: Array<string>;
    }

    interface AddDefaultsFunction {
        (parser: Parser<any>, options?: AddDefaultsOptions): void;
    }

    class Parser<ParserResult = DefaultParserResult> {

        constructor();

        addHandler: AddHandlerFunction<ParserResult>;
        parse: ParseFunction<ParserResult>;
        /** Removes handlers and returns the number of removed handlers */
        removeHandlers(target: RemoveHandlersTarget): number;
        getHandlers(): Array<HandlerInfo>;
        clone(): Parser<ParserResult>;
    }
}

declare module "parse-torrent-title" {

    export interface DefaultParserResult extends ParseTorrentTitle.DefaultParserResult { }
    export type ParserOptions = ParseTorrentTitle.ParserOptions;
    export type PositionOptions = ParseTorrentTitle.PositionOptions;
    export type HandlerTarget = ParseTorrentTitle.HandlerTarget;
    export type HandlerInfo = ParseTorrentTitle.HandlerInfo;
    export type AddDefaultsOptions = ParseTorrentTitle.AddDefaultsOptions;
    export type Handler<ParserResult = DefaultParserResult> = ParseTorrentTitle.Handler<ParserResult>;
    export type HandlerContext<ParserResult = DefaultParserResult> = ParseTorrentTitle.HandlerContext<ParserResult>;
    export type HandlerResult = ParseTorrentTitle.HandlerResult;
    export type RemoveHandlersTarget = ParseTorrentTitle.RemoveHandlersTarget;
    export type Transformer = ParseTorrentTitle.Transformer;
    export class Parser<ParserResult = DefaultParserResult> extends ParseTorrentTitle.Parser<ParserResult> { }
    export const parse: ParseTorrentTitle.ParseFunction;
    export const addHandler: ParseTorrentTitle.AddHandlerFunction;
    export const removeHandlers: (target: ParseTorrentTitle.RemoveHandlersTarget) => number;
    export const createParser: () => Parser;
    export const addDefaults: ParseTorrentTitle.AddDefaultsFunction;
    export const Transformers: ParseTorrentTitle.Transformers;
}
