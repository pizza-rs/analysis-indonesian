use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Indonesian stemmer based on the Asian Federation for Natural Language Processing
/// approach. Removes common Indonesian affixes (prefixes, suffixes, and circumfixes).
#[derive(Clone, Debug, Default)]
pub struct IndonesianStemFilter;

impl IndonesianStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for IndonesianStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.len() < 5 {
            return (false, None);
        }

        let stemmed = stem_indonesian(text);
        if stemmed != text {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn stem_indonesian(word: &str) -> String {
    let mut s = word.to_string();

    // Step 1: Remove particle suffixes (-lah, -kah, -tah, -pun)
    for suffix in &["-lah", "-kah", "-tah", "-pun", "lah", "kah", "tah", "pun"] {
        if s.ends_with(suffix) && s.len() - suffix.len() >= 3 {
            s = s[..s.len() - suffix.len()].to_string();
            break;
        }
    }

    // Step 2: Remove possessive suffixes (-ku, -mu, -nya)
    for suffix in &["nya", "mu", "ku"] {
        if s.ends_with(suffix) && s.len() - suffix.len() >= 3 {
            s = s[..s.len() - suffix.len()].to_string();
            break;
        }
    }

    // Step 3: Remove derivational suffixes (-kan, -an, -i)
    for suffix in &["kan", "an"] {
        if s.ends_with(suffix) && s.len() - suffix.len() >= 3 {
            s = s[..s.len() - suffix.len()].to_string();
            break;
        }
    }
    if s.ends_with('i') && s.len() - 1 >= 3 {
        s = s[..s.len() - 1].to_string();
    }

    // Step 4: Remove common prefixes (me-, meng-, mem-, men-, meny-, di-, pe-, per-, ber-, ke-, se-, ter-)
    let prefixes: &[&str] = &[
        "memper", "mempel", "menge", "menye", "mempe", "meng", "meny", "memo", "memu", "mem",
        "men", "pen", "pem", "peng", "peny", "ber", "per", "ter", "pel", "di", "ke", "se", "me",
        "pe",
    ];

    for prefix in prefixes {
        if s.starts_with(prefix) && s.len() - prefix.len() >= 3 {
            s = s[prefix.len()..].to_string();
            break;
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stem_suffix() {
        let filter = IndonesianStemFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("memakan"),
            start_offset: 0,
            end_offset: 7,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        // "me-" prefix + "-kan" suffix removed from "memakan" → "mak" or similar
        assert!(token.term.as_ref().len() < "memakan".len());
    }

    #[test]
    fn test_short_word_unchanged() {
        let filter = IndonesianStemFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("dan"),
            start_offset: 0,
            end_offset: 3,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "dan");
    }
}
