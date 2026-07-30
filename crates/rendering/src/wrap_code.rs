//! Wraps a string in a Telegram monospace code span.

/// Wraps `s` in a Telegram monospace code span.
///
/// Backticks inside `s` are escaped so the span cannot be broken out of.
pub fn wrap_code(s: &str) -> String {
    let escaped = s.replace('`', "'");
    format!("`{escaped}`")
}
