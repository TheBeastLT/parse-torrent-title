const Parser = require("./src/parser").Parser;
const handlers = require("./src/handlers");
const trasformers = require("./src/transformers");
const normalize = require("./src/normalize");
const filename = require("./src/filename");

const defaultParser = new Parser();

handlers.addDefaults(defaultParser);

exports.addDefaults = handlers.addDefaults;
exports.addHandler = (handlerName, handler, options) => defaultParser.addHandler(handlerName, handler, options);
exports.parse = title => defaultParser.parse(title);
exports.Parser = Parser;
exports.Transformers = trasformers;
exports.normalizeResolution = normalize.normalizeResolution;
exports.normalizeCodec = normalize.normalizeCodec;
exports.parseFilename = filename.parseFilename;
