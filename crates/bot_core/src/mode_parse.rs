//! Maps a settings button label back to a [`Mode`].

use crate::mode::Mode;

/// Maps a settings button label back to a `Mode`.
///
/// Returns `None` if `label` does not match a known mode.
pub fn parse_mode(label: &str) -> Option<Mode> {
    match label {
        "Word" => Some(Mode::Word),
        "Sentence" => Some(Mode::Sentence),
        "Paragraph" => Some(Mode::Paragraph),
        "Full" => Some(Mode::Full),
        _ => None,
    }
}
