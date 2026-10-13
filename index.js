const Parser = require("./src/parser").Parser;
const handlers = require("./src/handlers");
const trasformers = require("./src/transformers");

const defaultParser = new Parser();

handlers.addDefaults(defaultParser);

exports.addDefaults = handlers.addDefaults;
exports.addHandler = (...args) => defaultParser.addHandler(...args);
exports.removeHandlers = target => defaultParser.removeHandlers(target);
exports.createParser = () => defaultParser.clone();
exports.parse = title => defaultParser.parse(title);
exports.Parser = Parser;
exports.Transformers = trasformers;
