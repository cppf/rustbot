//! Finds the last whitespace break within a string.

fn is_break_char(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n')
}

/// Returns the char index immediately after the last whitespace run within
/// `s`, or `-1` if none is found.
pub fn last_word_break(s: &str) -> isize {
    let chars: Vec<char> = s.chars().collect();
    for i in (0..chars.len()).rev() {
        if is_break_char(chars[i]) {
            return (i + 1) as isize;
        }
    }
    -1
}
