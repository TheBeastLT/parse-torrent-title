const { none } = require("./transformers");

// chinese/japanese/russian chars https://stackoverflow.com/a/43419070
const ACCENTED_LATIN_CHARS = "\u00c0-\u00d6\u00d8-\u00f6\u00f8-\u024f";
const NON_ENGLISH_CHARS = "\u3040-\u30ff\u3400-\u4dbf\u4e00-\u9fff\uf900-\ufaff\uff66-\uff9f\u0400-\u04ff";
const RUSSIAN_CAST_REGEX = new RegExp("(?:\\([^)]*[\u0400-\u04ff][^)]*\\)|(?<=\\/.*)\\(.*\\))(?=\\s*(?:\\[[^\\]]*)?$)");
const ALT_TITLES_REGEX = new RegExp(`[^/|(]*[${NON_ENGLISH_CHARS}][^/|]*[/|]|[/|][^/|(]*[${NON_ENGLISH_CHARS}][^/|]*`, "g");
const ACCENTED_LATIN_REGEX = new RegExp(`[${ACCENTED_LATIN_CHARS}]`);
const NOT_ONLY_NON_ENGLISH_REGEX = new RegExp(`(?<=[a-zA-Z][^${NON_ENGLISH_CHARS}]+)[${NON_ENGLISH_CHARS}].*[${NON_ENGLISH_CHARS}]|[${NON_ENGLISH_CHARS}].*[${NON_ENGLISH_CHARS}](?=[^${NON_ENGLISH_CHARS}]+[a-zA-Z])`, "g");
const NOT_ALLOWED_SYMBOLS_AT_START_AND_END = new RegExp(`^[^\\w${ACCENTED_LATIN_CHARS}${NON_ENGLISH_CHARS}#[【★]+|[ \\-:,/\\\\[|{(#$&^]+$`, "g");
const REMAINING_NOT_ALLOWED_SYMBOLS_AT_START_AND_END = new RegExp(`^[^\\w${ACCENTED_LATIN_CHARS}${NON_ENGLISH_CHARS}#]+|[[\\]({} ]+$`, "g");
const BEFORE_TITLE_REGEX = new RegExp(`^\\[[^[\\]]+]|^(?:[^/[]*[${NON_ENGLISH_CHARS}][^/[]*/)+(?=\\s*[a-zA-Z])`);
const UNCLOSED_BRACKET_AT_END = /\s*[([{][^)\]}]*$/;
const UNOPENED_BRACKET = /^([^([{]*)[)\]}]/;
const DEFAULT_OPTIONS = {
    skipIfAlreadyFound: true, // whether to skip a matcher if another matcher from this group was already found
    skipFromTitle: false, // whether to exclude found match from the end result title
    skipIfFirst: false, // whether to skip this matcher if there are no other groups matched before it's matchIndex
    skipIfBefore: [], // whether to skip this matcher if it appears before specified matcher group in the name
    skipIfAfter: [], // whether to skip this matcher if it appears after specified matcher group in the name
    remove: false // whether to remove the found match from further matchers
};

function extendOptions(options) {
    return { ...DEFAULT_OPTIONS, ...options };
}

function createHandlerFromRegExp(name, regExp, transformer, options) {
    function handler({ title, result, matched }) {
        if (result[name] && options.skipIfAlreadyFound) {
            return null;
        }

        const match = title.match(regExp);
        const [rawMatch, cleanMatch] = match || [];

        if (rawMatch) {
            const transformed = transformer(cleanMatch || rawMatch, result[name]);
            const beforeTitleMatch = title.match(BEFORE_TITLE_REGEX);
            const isBeforeTitle = beforeTitleMatch && match.index < beforeTitleMatch[0].length;
            const otherMatches = options.skipIfFirst && Object.entries(matched).filter(e => e[0] !== name);
            const isSkipIfFirst = options.skipIfFirst && otherMatches.length &&
                otherMatches.every(e => match.index < e[1].matchIndex);
            const isSkipIfBefore = options.skipIfBefore.some(group => matched[group] && match.index < matched[group].matchIndex);
            const isSkipIfAfter = options.skipIfAfter.some(group => matched[group] && match.index >= matched[group].matchIndex);

            if (transformed && !isSkipIfFirst && !isSkipIfBefore && !isSkipIfAfter) {
                matched[name] = matched[name] || { rawMatch, matchIndex: match.index };
                result[name] = options.value || transformed;
                return {
                    rawMatch,
                    matchIndex: match.index,
                    remove: options.remove,
                    skipFromTitle: isBeforeTitle || options.skipFromTitle
                };
            }
        }

        return null;
    }

    return handler;
}

function removeAccentedAltTitles(title) {
    const titles = title.split(/\s+[/|]\s+/);
    const plainTitles = titles.filter(alias => !ACCENTED_LATIN_REGEX.test(alias));
    return plainTitles.length && plainTitles.length < titles.length ? plainTitles.join(" / ") : title;
}

function cleanTitle(rawTitle) {
    let cleanedTitle = rawTitle;

    if (cleanedTitle.indexOf(" ") === -1 && cleanedTitle.indexOf(".") !== -1) {
        cleanedTitle = cleanedTitle.replace(/\./g, " ");
    }

    cleanedTitle = cleanedTitle
        .replace(/_/g, " ")
        .replace(/[[(]movie[)\]]/i, "") // clear movie indication flag
        .replace(NOT_ALLOWED_SYMBOLS_AT_START_AND_END, "")
        .replace(RUSSIAN_CAST_REGEX, "") // clear russian cast information
        .replace(/^[[【★].*[\]】★][ .]?(.+)/, "$1") // remove release group markings sections from the start
        .replace(/(.+)[ .]?[[【★].*[\]】★]$/, "$1") // remove unneeded markings section at the end if present
        .replace(ALT_TITLES_REGEX, "") // remove alt language titles
        .replace(NOT_ONLY_NON_ENGLISH_REGEX, "") // remove non english chars if they are not the only ones left
        .replace(REMAINING_NOT_ALLOWED_SYMBOLS_AT_START_AND_END, "")
        .replace(/^[^\s.]*\.\S*$/, title => title.replace(/\./g, " ")) // replace dots with spaces if title is dot separated
        .replace(UNCLOSED_BRACKET_AT_END, "") // remove unclosed bracket section at the end
        .replace(UNOPENED_BRACKET, "$1") // remove closing bracket without an opening one
        .replace(/.*/, removeAccentedAltTitles) // prefer alt titles without accented chars
        .replace(/\s+/g, " ")
        .trim();

    return cleanedTitle;
}

function findPosition(handlers, before, after) {
    if (before && after) {
        throw new Error("Handler can not be positioned both before and after another handler");
    }
    if (!before && !after) {
        return handlers.length;
    }

    // a category name targets its first handler for `before` and its last handler for `after`
    const target = before || after;
    const { name, index = before ? 0 : -1 } = typeof target === "string" ? { name: target } : target;
    const positions = handlers.flatMap((entry, position) => (entry.name === name ? [position] : []));
    const position = positions[index < 0 ? positions.length + index : index];

    if (typeof position === "undefined") {
        throw new Error(`No handler found for ${JSON.stringify(target)}`);
    }

    return before ? position : position + 1;
}

class Parser {

    constructor() {
        this.handlers = [];
        this.parse = this.parse.bind(this);
    }

    addHandler(handlerName, handler, transformer, options) {
        if (typeof handlerName === "function") {

            // If no name is provided and a function handler is directly given
            [handlerName, handler, transformer] = ["unknown", handlerName, handler];
        }
        if (transformer && typeof transformer === "object") {

            // If options are given in place of the transformer
            [transformer, options] = [none, transformer];
        }

        const { before, after, ...matchOptions } = options || {};
        let entry;

        if (handler instanceof RegExp) {
            const regExpOptions = extendOptions(matchOptions);
            const regExpHandler = createHandlerFromRegExp(handlerName, handler, transformer || none, regExpOptions);
            entry = { name: handlerName, handler: regExpHandler, regExp: handler, options: regExpOptions };
        } else if (typeof handler === "function") {
            entry = { name: handlerName, handler };
        } else {
            throw new Error(`Handler for ${handlerName} should be a RegExp or a function. Got: ${typeof handler}`);
        }

        this.handlers.splice(findPosition(this.handlers, before, after), 0, entry);
    }

    removeHandlers(target) {
        const shouldRemove = typeof target === "function" ? target : handler => [].concat(target).includes(handler.name);
        const handlers = this.getHandlers();
        const count = this.handlers.length;

        this.handlers = this.handlers.filter((_, position) => !shouldRemove(handlers[position]));

        return count - this.handlers.length;
    }

    getHandlers() {
        const counts = {};

        return this.handlers.map(({ name, regExp, options }, position) => {
            counts[name] = (counts[name] || 0) + 1;
            return { name, index: counts[name] - 1, position, ...regExp && { regExp, options: { ...options } } };
        });
    }

    clone() {
        const parser = new this.constructor();
        parser.handlers = this.handlers.slice();
        return parser;
    }

    parse(title) {
        title = title.replace(/_+/g, " ");
        const result = {};
        const matched = {};
        const context = { title, result, matched };
        let endOfTitle = title.length;

        for (const { handler } of this.handlers) {
            context.title = title;
            const matchResult = handler(context);
            if (matchResult && matchResult.remove) {
                title = title.slice(0, matchResult.matchIndex) + title.slice(matchResult.matchIndex + matchResult.rawMatch.length);
            }
            if (matchResult && !matchResult.skipFromTitle && matchResult.matchIndex && matchResult.matchIndex < endOfTitle) {
                endOfTitle = matchResult.matchIndex;
            }
            if (matchResult && matchResult.remove && matchResult.skipFromTitle && matchResult.matchIndex < endOfTitle) {

                // adjust title index in case part of it should be removed and skipped
                endOfTitle -= matchResult.rawMatch.length;
            }
        }

        result.title = cleanTitle(title.slice(0, endOfTitle));

        return result;
    }
}

exports.Parser = Parser;
