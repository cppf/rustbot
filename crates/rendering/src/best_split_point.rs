//! Picks the best boundary at which to split long text.

use crate::last_index_after::last_index_after;
use crate::last_sentence_break::last_sentence_break;
use crate::last_word_break::last_word_break;

/// Returns a char index <= `limit` at which to cut `remaining`, preferring
/// (in order) a paragraph break, a sentence break, a word break, falling
/// back to a hard character cut at `limit`.
///
/// `limit` and the returned index are char (Unicode scalar value) counts,
/// matching the rune-based indexing used throughout this crate.
pub fn best_split_point(remaining: &str, limit: usize) -> usize {
    let window: String = remaining.chars().take(limit).collect();

    let i = last_index_after(&window, "\n\n");
    if i > 0 {
        return i as usize;
    }
    let i = last_sentence_break(&window);
    if i > 0 {
        return i as usize;
    }
    let i = last_word_break(&window);
    if i > 0 {
        return i as usize;
    }
    limit
}
