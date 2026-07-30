//! Recognizes trailing quote/bracket marks that follow sentence punctuation.

/// Reports whether `c` is a closing quote or bracket that should stay
/// attached to the sentence-terminating punctuation before it.
pub fn is_closing_mark(c: char) -> bool {
    matches!(
        c,
        '"' | '\'' | ')' | ']' | '}' | '\u{201d}' | '\u{2019}' | '\u{00bb}'
    )
}
