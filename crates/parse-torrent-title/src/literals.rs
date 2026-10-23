//! Which regex handlers can possibly match a title, found with a single scan of it.
//!
//! Checking each of the ~450 handler patterns against a title one by one costs more than all the actual
//! matching, since few of them match. Most patterns can only match where one of a few literals appears, like
//! `720p` or `x26`: `regex-syntax` extracts these from the translated patterns (see [`crate::translate`]), and
//! one Aho-Corasick automaton over all of them finds the candidate handlers in a single pass.
//!
//! Removing a match from the title can only create a new literal occurrence across the cut, so after a removal
//! only the text around it is scanned again. Earlier candidates stay candidates, which only costs a check.

use aho_corasick::{AhoCorasick, AhoCorasickKind};
use regex_syntax::hir::literal::{ExtractKind, Extractor, Seq};

use crate::Regex;

/// The candidate handlers of a title, by handler position.
pub(crate) type Candidates = Vec<bool>;

pub(crate) struct Literals {
    automaton: AhoCorasick,
    /// The handler position of each literal.
    owners: Vec<usize>,
    /// Handlers without literals, which are always candidates.
    unfiltered: Candidates,
    longest: usize,
}

impl Literals {
    /// Indexes the literals of the regex handlers, given by position (`None` for other handlers).
    pub(crate) fn new(regexes: &[Option<&Regex>]) -> Self {
        let mut literals: Vec<Vec<u8>> = Vec::new();
        let mut owners = Vec::new();
        let mut unfiltered = vec![false; regexes.len()];
        for (position, regex) in regexes.iter().enumerate() {
            match regex.and_then(required_literals) {
                Some(required) => {
                    let mut required: Vec<Vec<u8>> =
                        required.into_iter().map(|literal| literal.to_ascii_lowercase()).collect();
                    required.sort();
                    required.dedup();
                    owners.extend(std::iter::repeat_n(position, required.len()));
                    literals.extend(required);
                }
                None => unfiltered[position] = true,
            }
        }
        let longest = literals.iter().map(Vec::len).max().unwrap_or(0);
        // matching ascii letters in any case only finds more candidates, and folds case variants into one literal
        let automaton = AhoCorasick::builder()
            .ascii_case_insensitive(true)
            .kind(Some(AhoCorasickKind::DFA))
            .build(&literals)
            .expect("the literals are bounded in number and length");
        Self { automaton, owners, unfiltered, longest }
    }

    /// The handlers which can match `title`.
    pub(crate) fn candidates(&self, title: &str) -> Candidates {
        let mut candidates = self.unfiltered.clone();
        self.mark(title.as_bytes(), &mut candidates);
        candidates
    }

    /// Adds the handlers which can match across `cut`, where text was removed from `title`.
    pub(crate) fn add_around(&self, title: &str, cut: usize, candidates: &mut Candidates) {
        let reach = self.longest.saturating_sub(1);
        let window = &title.as_bytes()[cut.saturating_sub(reach)..title.len().min(cut + reach)];
        self.mark(window, candidates);
    }

    fn mark(&self, text: &[u8], candidates: &mut Candidates) {
        for found in self.automaton.find_overlapping_iter(text) {
            candidates[self.owners[found.pattern().as_usize()]] = true;
        }
    }
}

/// Literals one of which every match of `regex` contains, `None` when there are none worth searching for.
fn required_literals(regex: &Regex) -> Option<Vec<Vec<u8>>> {
    let translation = regex.translation()?;
    // the automaton matches ascii letters in any case, which is all js folds an ascii pattern into without the
    // `u` flag, so the costly unicode case folding of the extraction is only needed for other patterns
    let ascii_folding = translation.folds_ascii_letters_only();
    let pattern = translation.pattern.strip_prefix("(?i)").filter(|_| ascii_folding).unwrap_or(&translation.pattern);
    let hir = regex_syntax::ParserBuilder::new().build().parse(pattern).ok()?;
    // every match starts with one of the prefixes, and ends with one of the suffixes: keep the more selective
    let prefixes = literals(Extractor::new().kind(ExtractKind::Prefix).extract(&hir), ExtractKind::Prefix);
    let suffixes = literals(Extractor::new().kind(ExtractKind::Suffix).extract(&hir), ExtractKind::Suffix);
    let selectivity =
        |literals: &Vec<Vec<u8>>| (literals.iter().map(Vec::len).min().unwrap_or(0), std::cmp::Reverse(literals.len()));
    match (prefixes, suffixes) {
        (Some(prefixes), Some(suffixes)) => {
            Some(if selectivity(&suffixes) > selectivity(&prefixes) { suffixes } else { prefixes })
        }
        (prefixes, suffixes) => prefixes.or(suffixes),
    }
}

fn literals(mut seq: Seq, kind: ExtractKind) -> Option<Vec<Vec<u8>>> {
    match kind {
        ExtractKind::Prefix => seq.optimize_for_prefix_by_preference(),
        _ => seq.optimize_for_suffix_by_preference(),
    }
    let literals = seq.literals()?;
    // an empty literal is found everywhere, and an empty set would mean the pattern never matches
    if literals.is_empty() || literals.iter().any(|literal| literal.is_empty()) {
        return None;
    }
    Some(literals.iter().map(|literal| literal.as_bytes().to_vec()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn regex(source: &str, flags: &str) -> Regex {
        Regex::new(source, flags).unwrap()
    }

    #[test]
    fn finds_the_handlers_whose_literals_appear() {
        let (resolution, codec, anything) = (regex(r"\b(720|1080)p\b", "i"), regex(r"x26[45]", ""), regex(r".", ""));
        let literals = Literals::new(&[Some(&resolution), None, Some(&codec), Some(&anything)]);

        assert_eq!(literals.candidates("Movie 1080P"), [true, true, false, true]);
        assert_eq!(literals.candidates("Movie x264"), [false, true, true, true]);
    }

    #[test]
    fn keeps_handlers_whose_keywords_are_optional() {
        // real handlers whose matches may or may not contain each keyword
        let patterns = [
            (r"(?:\bthe\W)?(?:\bcomplete|collection|dvd)?\b[ .]?\bbox[ .-]?set\b", "i"),
            (r"\b(?:H[DQ][ .-]*)?CAM(?:H[DQ])?(?:[ .-]*Rip)?\b", "i"),
            (r"(?:\D|^)(\d{1,2})[. ]?(?:st|nd|rd|th)?[. ]*season", "i"),
            (r"\bmulti(?:ple)?[ .-]*(?:lang(?:uages?)?|audio|VF2)?\b", "i"),
            (r"[([]?(\d{3,4})p?[)\]]?", ""),
        ];
        let regexes: Vec<Regex> = patterns.iter().map(|(source, flags)| regex(source, flags)).collect();
        let literals = Literals::new(&regexes.iter().map(Some).collect::<Vec<_>>());
        let titles = [
            "Show Box Set",
            "The Complete Box-Set",
            "Collection boxset",
            "Movie CAM",
            "Movie HDCAMRip",
            "Movie HQ-CAM",
            "Show 2nd season",
            "Show 2 Season",
            "Show season",
            "Movie MULTI",
            "Movie Multiple Languages",
            "Movie 1080",
            "Movie [720p]",
            "nothing to see",
        ];
        for title in titles {
            let candidates = literals.candidates(title);
            for (regex, candidate) in regexes.iter().zip(candidates) {
                assert!(candidate || regex.find(title).is_none(), "{regex} matches {title:?} but was skipped");
            }
        }
        assert_eq!(literals.candidates("nothing to see")[..4], [false; 4]);
    }

    #[test]
    fn finds_literals_created_by_a_removal() {
        let codec = regex(r"x264", "");
        let literals = Literals::new(&[Some(&codec)]);
        let mut candidates = literals.candidates("x2 [cut] 64");
        assert_eq!(candidates, [false]);

        literals.add_around("x264", 2, &mut candidates);
        assert_eq!(candidates, [true]);
    }
}
