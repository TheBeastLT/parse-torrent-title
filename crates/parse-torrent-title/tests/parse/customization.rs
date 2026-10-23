// Ported from test/customization.js of the js library.
//
// The js `addHandler(name, handler, transformer?, options?)` maps to `Parser::add_handler` / `insert_handler`
// with a `Handler`, so a few js tests about argument juggling are covered by the types instead:
// options given in place of the transformer, positioning keys leaking into the options, and both
// `before` and `after` being given (a `Position` is one or the other).

//! Parser customization

use parse_torrent_title::transformers::{lowercase, uppercase, value};
use parse_torrent_title::{
    Error, Handler, Options, Parser, Position, Regex, Target, Value, add_defaults, create_parser, parse,
};

fn regex(source: &str) -> Regex {
    Regex::new(source, "").unwrap()
}

fn names(parser: &Parser) -> Vec<String> {
    parser.handlers().into_iter().map(|handler| handler.name).collect()
}

fn simple_parser() -> Parser {
    let mut parser = Parser::new();
    parser.add_handler(Handler::regex("resolution", regex(r"\b(720)p\b")).transform(value("$1p")));
    parser.add_handler(Handler::regex("resolution", regex(r"\b(1080)p\b")).transform(value("$1p")));
    parser.add_handler(Handler::regex("codec", regex(r"\bx26[45]\b")));
    parser
}

fn str_field<'a>(parsed: &'a parse_torrent_title::ParsedTitle, field: &str) -> Option<&'a str> {
    parsed.get(field).and_then(Value::as_str)
}

/// getHandlers
mod handlers {
    use super::*;

    /// lists handlers with their category index and position
    #[test]
    fn lists_handlers_with_their_category_index_and_position() {
        let mut parser = simple_parser();
        parser.add_handler(Handler::function("custom", |_| None));

        let handlers = parser.handlers();

        let listed: Vec<_> = handlers.iter().map(|h| (h.name.as_str(), h.index, h.position)).collect();
        assert_eq!(listed, [("resolution", 0, 0), ("resolution", 1, 1), ("codec", 0, 2), ("custom", 0, 3)]);
        assert_eq!(handlers[1].regex.as_ref().map(Regex::source), Some(r"\b(1080)p\b"));
        let options = handlers[1].options.as_ref().unwrap();
        assert!(options.skip_if_already_found);
        assert!(!options.remove);
        assert!(handlers[3].regex.is_none());
    }

    /// names unnamed function handlers as unknown
    #[test]
    fn names_unnamed_function_handlers_as_unknown() {
        let mut parser = Parser::new();
        parser.add_handler(Handler::anonymous(|_| None));

        assert_eq!(names(&parser), ["unknown"]);
    }

    /// returns copies which do not modify the parser
    #[test]
    fn returns_copies_which_do_not_modify_the_parser() {
        let parser = simple_parser();
        let mut handlers = parser.handlers();
        handlers.pop();
        handlers[0].options.as_mut().unwrap().remove = true;

        assert_eq!(names(&parser), ["resolution", "resolution", "codec"]);
        assert!(!parser.handlers()[0].options.as_ref().unwrap().remove);
    }
}

/// addHandler positioning
mod add_handler_positioning {
    use super::*;

    /// appends to the end by default
    #[test]
    fn appends_to_the_end_by_default() {
        let mut parser = simple_parser();
        parser.add_handler(Handler::regex("network", regex(r"\bNF\b")));

        assert_eq!(names(&parser), ["resolution", "resolution", "codec", "network"]);
    }

    /// inserts after the last handler of a category
    #[test]
    fn inserts_after_the_last_handler_of_a_category() {
        let mut parser = simple_parser();
        parser.insert_handler(Position::after("resolution"), Handler::regex("network", regex(r"\bNF\b"))).unwrap();

        assert_eq!(names(&parser), ["resolution", "resolution", "network", "codec"]);
    }

    /// inserts before the first handler of a category
    #[test]
    fn inserts_before_the_first_handler_of_a_category() {
        let mut parser = simple_parser();
        let network = Handler::regex("network", regex(r"\bNF\b")).transform(uppercase());
        parser.insert_handler(Position::before("resolution"), network).unwrap();

        assert_eq!(names(&parser), ["network", "resolution", "resolution", "codec"]);
    }

    /// inserts relative to a specific handler index of a category
    #[test]
    fn inserts_relative_to_a_specific_handler_index_of_a_category() {
        let mut parser = simple_parser();
        parser
            .insert_handler(Position::after(Target::new("resolution", 0)), Handler::regex("after0", regex("a")))
            .unwrap();
        parser
            .insert_handler(Position::before(Target::new("resolution", 1)), Handler::regex("before1", regex("b")))
            .unwrap();

        assert_eq!(names(&parser), ["resolution", "after0", "before1", "resolution", "codec"]);
    }

    /// targets the first and last handler of a non contiguous category
    #[test]
    fn targets_the_first_and_last_handler_of_a_non_contiguous_category() {
        let mut parser = Parser::new();
        parser.add_handler(Handler::regex("a", regex("a1")));
        parser.add_handler(Handler::regex("b", regex("b")));
        parser.add_handler(Handler::regex("a", regex("a2")));
        parser.insert_handler(Position::before("a"), Handler::regex("first", regex("f"))).unwrap();
        parser.insert_handler(Position::after("a"), Handler::regex("last", regex("l"))).unwrap();

        assert_eq!(names(&parser), ["first", "a", "b", "a", "last"]);
    }

    /// targets handlers counted from the end with a negative index
    #[test]
    fn targets_handlers_counted_from_the_end_with_a_negative_index() {
        let mut parser = simple_parser();
        parser
            .insert_handler(
                Position::before(Target::new("resolution", -2)),
                Handler::regex("network", regex(r"\bNF\b")),
            )
            .unwrap();

        assert_eq!(names(&parser), ["network", "resolution", "resolution", "codec"]);
    }

    /// targets the first or last handler of a category when the index is omitted
    #[test]
    fn targets_the_first_or_last_handler_of_a_category_when_the_index_is_omitted() {
        let mut parser = simple_parser();
        parser.insert_handler(Position::before("resolution"), Handler::regex("first", regex("f"))).unwrap();
        parser.insert_handler(Position::after("resolution"), Handler::regex("last", regex("l"))).unwrap();

        assert_eq!(names(&parser), ["first", "resolution", "resolution", "last", "codec"]);
    }

    /// positions function handlers
    #[test]
    fn positions_function_handlers() {
        let mut parser = simple_parser();
        parser.insert_handler(Position::before("codec"), Handler::function("named", |_| None)).unwrap();
        parser.insert_handler(Position::before("resolution"), Handler::anonymous(|_| None)).unwrap();

        assert_eq!(names(&parser), ["unknown", "resolution", "resolution", "named", "codec"]);
    }

    /// does not pass positioning keys to the handler options
    #[test]
    fn does_not_pass_positioning_keys_to_the_handler_options() {
        let mut parser = simple_parser();
        let network = Handler::regex("network", regex(r"\bNF\b")).options(Options::default().remove());
        parser.insert_handler(Position::after("codec"), network).unwrap();

        assert_eq!(parser.handlers()[3].options, Some(Options::default().remove()));
    }

    /// runs inserted handlers in their position
    #[test]
    fn runs_inserted_handlers_in_their_position() {
        let mut parser = simple_parser();
        let four_k = Handler::regex("resolution", Regex::new(r"\b(4k)\b", "i").unwrap()).transform(value("4k"));
        parser.insert_handler(Position::before("resolution"), four_k).unwrap();

        assert_eq!(str_field(&parser.parse("Movie 1080p 4k x264"), "resolution"), Some("4k"));
    }

    /// throws for an unknown category
    #[test]
    fn throws_for_an_unknown_category() {
        let mut parser = simple_parser();

        let error =
            parser.insert_handler(Position::after("missing"), Handler::regex("network", regex("NF"))).unwrap_err();
        assert_eq!(error, Error::UnknownTarget { name: "missing".to_owned(), index: None });
        assert!(error.to_string().contains("missing"));
    }

    /// throws for an out of range category index
    #[test]
    fn throws_for_an_out_of_range_category_index() {
        let mut parser = simple_parser();

        let error = parser
            .insert_handler(Position::before(Target::new("codec", 1)), Handler::regex("network", regex("NF")))
            .unwrap_err();
        assert!(error.to_string().contains("codec"));
    }

    /// throws for an invalid regex
    #[test]
    fn throws_for_an_invalid_regex() {
        assert!(matches!(Regex::new("(unclosed", ""), Err(Error::InvalidRegex { .. })));
    }
}

/// removeHandlers
mod remove_handlers {
    use super::*;

    /// removes all handlers of a category
    #[test]
    fn removes_all_handlers_of_a_category() {
        let mut parser = simple_parser();

        assert_eq!(parser.remove_handlers(&["resolution"]), 2);
        assert_eq!(names(&parser), ["codec"]);
    }

    /// removes handlers of several categories
    #[test]
    fn removes_handlers_of_several_categories() {
        let mut parser = simple_parser();

        assert_eq!(parser.remove_handlers(&["resolution", "codec"]), 3);
        assert!(names(&parser).is_empty());
    }

    /// removes handlers matching a filter
    #[test]
    fn removes_handlers_matching_a_filter() {
        let mut parser = simple_parser();

        assert_eq!(parser.remove_handlers_where(|handler| handler.name == "resolution" && handler.index == 1), 1);
        let sources: Vec<String> =
            parser.handlers().iter().filter_map(|h| h.regex.as_ref().map(|r| r.source().to_owned())).collect();
        assert_eq!(sources, [r"\b(720)p\b", r"\bx26[45]\b"]);
    }

    /// returns zero when nothing matches
    #[test]
    fn returns_zero_when_nothing_matches() {
        let mut parser = simple_parser();

        assert_eq!(parser.remove_handlers(&["missing"]), 0);
    }

    /// stops parsing removed categories
    #[test]
    fn stops_parsing_removed_categories() {
        let mut parser = create_parser();
        parser.remove_handlers(&["group"]);

        assert!(!parser.parse("Game.of.Thrones.S01E01.720p.HDTV.x264-CTU").contains("group"));
    }
}

/// clone
mod clone {
    use super::*;

    /// copies handlers without sharing changes
    #[test]
    fn copies_handlers_without_sharing_changes() {
        let mut parser = simple_parser();
        let mut copy = parser.clone();
        copy.remove_handlers(&["codec"]);
        copy.add_handler(Handler::regex("network", regex("NF")));
        parser.add_handler(Handler::regex("other", regex("O")));

        assert_eq!(names(&parser), ["resolution", "resolution", "codec", "other"]);
        assert_eq!(names(&copy), ["resolution", "resolution", "network"]);
    }
}

/// createParser
mod create_parser_ {
    use super::*;

    /// creates a parser with the default handlers
    #[test]
    fn creates_a_parser_with_the_default_handlers() {
        let parser = create_parser();

        assert_eq!(
            parser.parse("Game.of.Thrones.S01E01.720p.HDTV.x264-CTU"),
            parse("Game.of.Thrones.S01E01.720p.HDTV.x264-CTU")
        );
    }

    /// does not modify the default parser
    #[test]
    fn does_not_modify_the_default_parser() {
        let mut parser = create_parser();
        parser.remove_handlers(&["resolution"]);

        assert_eq!(parse("Game.of.Thrones.S01E01.720p.HDTV.x264-CTU").resolution(), Some("720p"));
    }
}

/// addDefaults
mod add_defaults_ {
    use super::*;

    /// excludes categories
    #[test]
    fn excludes_categories() {
        let mut parser = Parser::new();
        add_defaults(&mut parser, &["site", "languages"]);

        let names = names(&parser);
        assert!(!names.iter().any(|name| name == "site" || name == "languages"));
        assert!(names.iter().any(|name| name == "resolution"));
    }
}

/// bug fixes
mod bug_fixes {
    use super::*;

    /// forwards transformer and options of module level addHandler
    ///
    /// The default parser is immutable in rust, so this runs on a copy of it.
    #[test]
    fn forwards_transformer_and_options_of_add_handler() {
        let mut parser = create_parser();
        let bar = Handler::regex("customBarTag", regex(r"\bBAR\b"))
            .transform(lowercase())
            .options(Options::default().remove());
        parser.insert_handler(Position::before("site"), bar).unwrap();

        let parsed = parser.parse("BAR Movie 2010");
        assert_eq!(str_field(&parsed, "customBarTag"), Some("bar"));
        assert_eq!(parsed.title(), "Movie");
    }

    /// applies options given in place of the transformer
    #[test]
    fn applies_options_without_a_transformer() {
        let mut parser = create_parser();
        let tag = Handler::regex("tag", regex(r"\bFOO\b")).options(Options::default().remove());
        parser.insert_handler(Position::before("site"), tag).unwrap();

        let parsed = parser.parse("FOO Movie 2010");
        assert_eq!(str_field(&parsed, "tag"), Some("FOO"));
        assert_eq!(parsed.title(), "Movie");
    }

    /// parses with an unbound parse function
    #[test]
    fn parses_with_an_unbound_parse_function() {
        let parser = create_parser();
        let parse = |title| parser.parse(title);

        assert_eq!(parse("Game.of.Thrones.S01E01.720p.HDTV.x264-CTU").title(), "Game of Thrones");
    }
}

/// function handlers
mod function_handlers {
    use super::*;
    use parse_torrent_title::HandlerMatch;

    /// can read earlier matches and remove text from the title
    #[test]
    fn can_read_earlier_matches_and_remove_text_from_the_title() {
        let mut parser = create_parser();
        let network = Handler::function("network", |context| {
            let index = parse_torrent_title::utf16_index(context.title, context.title.find("NF")?);
            context.result.set("network", "Netflix");
            Some(HandlerMatch {
                raw_match: Some("NF".to_owned()),
                match_index: index,
                remove: true,
                skip_from_title: false,
            })
        });
        parser.insert_handler(Position::before("site"), network).unwrap();

        let parsed = parser.parse("Stranger Things NF S04E01");
        assert_eq!(str_field(&parsed, "network"), Some("Netflix"));
        assert_eq!(parsed.title(), "Stranger Things");
        assert_eq!(parsed.episode(), Some(1));
    }
}
