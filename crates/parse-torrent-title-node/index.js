// The api of the js library on top of the rust parser of `native.js`, built by `npm run build`.
// Values cross into rust as json; handlers and transformers written in js are wrapped to take and return json.
const { NativeParser, applyTransformer } = require("./native");

// built in transformers are described for rust to run them, and stay callable from js
function builtin(spec) {
    const transformer = (input, current) => {
        const json = current === undefined ? null : JSON.stringify(current);
        const result = applyTransformer(JSON.stringify(spec), String(input), json);
        return result === null ? null : JSON.parse(result);
    };
    transformer.spec = spec;
    return transformer;
}

// the moment.js tokens of a date format as the chrono specifiers the rust transformer reads
const MOMENT_TOKENS = { YYYY: "%Y", YY: "%y", MMMM: "%b", MMM: "%b", MM: "%m", M: "%m", Do: "%d", DD: "%d", D: "%d" };

function chronoFormat(format) {
    if (typeof format !== "string") {
        throw new Error("Transformers.date takes a single moment.js format");
    }
    const converted = format.replace(/YYYY|YY|MMMM|MMM|MM|Do|DD|M|D/g, token => MOMENT_TOKENS[token]);
    if (/[a-z]/i.test(converted.replace(/%[a-z]/gi, ""))) {
        throw new Error(`Unsupported token in date format ${JSON.stringify(format)}, use YYYY, YY, MMMM, MMM, MM, M, Do, DD or D`);
    }
    return converted;
}

const Transformers = {
    none: builtin({ kind: "none" }),
    value: value => builtin({ kind: "value", value }),
    integer: builtin({ kind: "integer" }),
    cjkInteger: builtin({ kind: "cjkInteger" }),
    boolean: builtin({ kind: "boolean" }),
    lowercase: builtin({ kind: "lowercase" }),
    uppercase: builtin({ kind: "uppercase" }),
    date: format => builtin({ kind: "date", format: chronoFormat(format) }),
    range: builtin({ kind: "range" }),
    list: builtin({ kind: "list" }),
    rangeUpTo: builtin({ kind: "rangeUpTo" }),
    yearRange: builtin({ kind: "yearRange" }),
    array: chain => (!chain || chain.spec ? builtin({ kind: "array", chain: chain && chain.spec }) : input => [chain(input)]),
    uniqConcat: chain => (chain.spec ? builtin({ kind: "uniqConcat", chain: chain.spec }) : (input, current) => {
        const values = current || [];
        const value = chain(input);
        return values.includes(value) ? values : values.concat(value);
    })
};

// a transformer written in js, called by rust with `[input, current]` and returning its truthy value as json
function jsonTransformer(transformer) {
    return json => {
        const [input, current] = JSON.parse(json);
        const value = transformer(input, current === null ? undefined : current);
        return value ? JSON.stringify(value) : null;
    };
}

// a handler written in js, called by rust with its context and returning the changed context with its result
function jsonHandler(handler) {
    return json => {
        const context = JSON.parse(json);
        const returned = handler(context);
        return JSON.stringify({ result: context.result, matched: context.matched, returned: returned || null });
    };
}

function toPosition({ before, after }) {
    return JSON.stringify({ before: before || null, after: after || null });
}

class Parser {

    constructor(native) {
        this.native = native || new NativeParser();
        this.parse = this.parse.bind(this);
    }

    addHandler(handlerName, handler, transformer, options) {
        if (typeof handlerName === "function") {

            // If no name is provided and a function handler is directly given
            [handlerName, handler, transformer] = ["unknown", handlerName, handler];
        }
        if (transformer && typeof transformer === "object") {

            // If options are given in place of the transformer
            [transformer, options] = [Transformers.none, transformer];
        }

        const { before, after, ...matchOptions } = options || {};
        const position = toPosition({ before, after });

        if (handler instanceof RegExp) {
            const spec = transformer ? transformer.spec : Transformers.none.spec;
            const custom = transformer && !transformer.spec ? jsonTransformer(transformer) : undefined;
            this.native.addRegexHandler(handlerName, handler.source, handler.flags, spec && JSON.stringify(spec), custom, JSON.stringify(matchOptions), position);
        } else if (typeof handler === "function") {
            this.native.addFunctionHandler(handlerName, jsonHandler(handler), position);
        } else {
            throw new Error(`Handler for ${handlerName} should be a RegExp or a function. Got: ${typeof handler}`);
        }
    }

    removeHandlers(target) {
        const shouldRemove = typeof target === "function" ? target : handler => [].concat(target).includes(handler.name);
        const positions = this.getHandlers().filter(handler => shouldRemove(handler)).map(handler => handler.position);

        return this.native.removeAt(positions);
    }

    getHandlers() {
        return JSON.parse(this.native.getHandlers()).map(({ name, index, position, source, flags, options }) => ({
            name,
            index,
            position,
            ...source !== undefined && { regExp: new RegExp(source, flags), options }
        }));
    }

    clone() {
        return new this.constructor(this.native.clone());
    }

    parse(title) {
        return JSON.parse(this.native.parse(title));
    }
}

function addDefaults(parser, { exclude } = {}) {
    parser.native.addDefaults([].concat(exclude || []));
}

const defaultParser = new Parser(NativeParser.withDefaults());

exports.addDefaults = addDefaults;
exports.addHandler = (...args) => defaultParser.addHandler(...args);
exports.removeHandlers = target => defaultParser.removeHandlers(target);
exports.createParser = () => defaultParser.clone();
exports.parse = title => defaultParser.parse(title);
exports.Parser = Parser;
exports.Transformers = Transformers;
