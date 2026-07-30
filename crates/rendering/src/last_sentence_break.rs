//! Finds the last sentence break within a string.

use crate::closing_mark::is_closing_mark;

fn is_terminator(c: char) -> bool {
    matches!(c, '.' | '!' | '?')
}

fn is_trailing_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\n' | '\t')
}

/// Returns the char index immediately after the last sentence-terminating
/// punctuation (plus trailing closing marks and whitespace) within `s`, or
/// `-1` if none is found.
pub fn last_sentence_break(s: &str) -> isize {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut best: isize = -1;
    let mut i = 0usize;

    while i < n {
        let c = chars[i];
        if is_terminator(c) {
            let mut j = i + 1;
            while j < n && is_closing_mark(chars[j]) {
                j += 1;
            }
            while j < n && is_trailing_whitespace(chars[j]) {
                j += 1;
            }
            best = j as isize;
            i = j;
            continue;
        }
        i += 1;
    }
    best
}
