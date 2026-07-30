//! Splits text into units, wraps each non-blank unit in its own code span,
//! and reassembles them preserving original separators.

use crate::split_surrounding_space::split_surrounding_space;
use crate::wrap_code::wrap_code;

/// Splits `text` into units with the given `splitter`, wraps each non-blank
/// unit in its own code span, and reassembles them using the original
/// separators so surrounding whitespace/newlines are preserved.
pub fn render_units<F>(text: &str, splitter: F) -> String
where
    F: Fn(&str) -> Vec<String>,
{
    if text.trim().is_empty() {
        return text.to_string();
    }

    let units = splitter(text);
    let mut out = String::with_capacity(text.len() * 2);
    for u in units {
        if u.trim().is_empty() {
            out.push_str(&u);
            continue;
        }
        let (lead, core, trail) = split_surrounding_space(&u);
        out.push_str(&lead);
        out.push_str(&wrap_code(&core));
        out.push_str(&trail);
    }
    out
}
