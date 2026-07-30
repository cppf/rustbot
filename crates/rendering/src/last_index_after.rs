//! Finds the char index just after the last occurrence of a substring.

/// Returns the char index immediately after the last occurrence of `sep`
/// in `s`, or `-1` if not found.
///
/// Rust's `str::rfind` returns a *byte* offset; this converts it to a
/// *char* (Unicode scalar value) offset, matching the semantics used
/// throughout this crate (equivalent to Go's rune indices).
pub fn last_index_after(s: &str, sep: &str) -> isize {
    match s.rfind(sep) {
        None => -1,
        Some(byte_idx) => {
            let end_byte = byte_idx + sep.len();
            // Count chars up to end_byte.
            s[..end_byte].chars().count() as isize
        }
    }
}
