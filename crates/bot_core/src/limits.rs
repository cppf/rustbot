//! Telegram's hard caps on text message and caption length.

/// `TELEGRAM_MESSAGE_LIMIT` is Telegram's hard cap on text message length,
/// in UTF-16 code units. We use it conservatively as a `char` (Unicode
/// scalar value) count, which is always <= the UTF-16 length and
/// therefore always safe.
pub const TELEGRAM_MESSAGE_LIMIT: usize = 4096;

/// `TELEGRAM_CAPTION_LIMIT` is Telegram's hard cap on media caption length.
pub const TELEGRAM_CAPTION_LIMIT: usize = 1024;
