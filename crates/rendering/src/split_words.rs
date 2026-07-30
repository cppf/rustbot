//! Splits text into words, keeping whitespace attached for lossless reassembly.

fn is_space_char(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r')
}

/// Splits `text` into words, keeping the whitespace between them attached
/// to the following word so reassembly is lossless.
pub fn split_words(text: &str) -> Vec<String> {
    let mut units: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut in_space = false;
    let mut started = false;

    for c in text.chars() {
        let is_space = is_space_char(c);
        if !started {
            cur.push(c);
            in_space = is_space;
            started = true;
            continue;
        }
        if is_space == in_space {
            cur.push(c);
            continue;
        }
        if in_space {
            // transition from space to word: keep space attached ahead
            cur.push(c);
            in_space = false;
            continue;
        }
        // transition from word to space: flush word, start new unit with space
        units.push(std::mem::take(&mut cur));
        cur.push(c);
        in_space = true;
    }

    if !cur.is_empty() {
        units.push(cur);
    }
    units
}
