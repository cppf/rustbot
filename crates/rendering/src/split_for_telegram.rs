//! Splits rendered text into chunks that fit Telegram's message length limit.

use crate::best_split_point::best_split_point;

/// Splits rendered `text` into chunks that each fit within `limit`
/// chars (Unicode scalar values), preferring to break at paragraph, then
/// sentence, then word, then character boundaries so content is never
/// truncated or lost.
pub fn split_for_telegram(text: &str, limit: usize) -> Vec<String> {
    let total_chars = text.chars().count();
    if total_chars <= limit {
        return vec![text.to_string()];
    }

    let mut chunks = Vec::new();
    let mut remaining: Vec<char> = text.chars().collect();

    while remaining.len() > limit {
        let remaining_str: String = remaining.iter().collect();
        let mut cut = best_split_point(&remaining_str, limit);
        if cut == 0 {
            cut = limit;
        }
        let chunk: String = remaining[..cut].iter().collect();
        chunks.push(chunk);
        remaining = remaining[cut..].to_vec();
    }
    if !remaining.is_empty() {
        chunks.push(remaining.iter().collect());
    }
    chunks
}
