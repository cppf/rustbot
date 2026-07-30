//! `Mode` identifies how incoming text is chunked before being wrapped in
//! Telegram monospace formatting.

/// How incoming text is chunked before being wrapped in monospace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Word,
    Sentence,
    Paragraph,
    Full,
}

/// `DEFAULT_MODE` is used for users who have not chosen one yet.
pub const DEFAULT_MODE: Mode = Mode::Full;
