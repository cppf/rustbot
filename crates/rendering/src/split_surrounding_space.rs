//! Separates leading/trailing whitespace from the core content of a unit.

const WHITESPACE: [char; 4] = [' ', '\t', '\r', '\n'];

/// Separates leading/trailing whitespace from the core content of a unit.
///
/// Returns `(lead, core, trail)` so wrapping only touches the visible text.
pub fn split_surrounding_space(s: &str) -> (String, String, String) {
    let trimmed_left = s.trim_start_matches(WHITESPACE);
    let lead_len = s.len() - trimmed_left.len();
    let lead = &s[..lead_len];

    let trimmed_both = trimmed_left.trim_end_matches(WHITESPACE);
    let trail = &trimmed_left[trimmed_both.len()..];

    (lead.to_string(), trimmed_both.to_string(), trail.to_string())
}
