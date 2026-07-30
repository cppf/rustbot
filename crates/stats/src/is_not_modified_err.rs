//! Detects Telegram's harmless "message is not modified" edit error.

use std::fmt::Display;

/// Reports whether `err` is Telegram's "message is not modified" error,
/// returned when editing a message with identical content.
pub fn is_not_modified_err<E: Display>(err: &E) -> bool {
    err.to_string()
        .to_lowercase()
        .contains("message is not modified")
}
