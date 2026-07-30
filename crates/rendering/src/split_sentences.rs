//! Splits text into sentences.

use crate::closing_mark::is_closing_mark;

fn is_terminator(c: char) -> bool {
    matches!(c, '.' | '!' | '?')
}

/// Splits `text` into sentences, ending each unit after a
/// sentence-terminating punctuation mark (`.` `!` `?`) plus any trailing
/// quote or bracket, keeping following whitespace attached to the next
/// sentence.
pub fn split_sentences(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let mut units = Vec::new();
    let mut start = 0usize;
    let mut i = 0usize;

    while i < n {
        let c = chars[i];
        if is_terminator(c) {
            let mut j = i + 1;
            while j < n && is_closing_mark(chars[j]) {
                j += 1;
            }
            units.push(chars[start..j].iter().collect());
            start = j;
            i = j;
            continue;
        }
        i += 1;
    }
    if start < n {
        units.push(chars[start..].iter().collect());
    }
    units
}
