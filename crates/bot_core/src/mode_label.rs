//! Human-readable labels for each [`Mode`], shown on settings buttons.

use crate::mode::Mode;

/// Returns the human-readable label shown on settings buttons.
pub fn mode_label(mode: Mode) -> &'static str {
    match mode {
        Mode::Word => "Word",
        Mode::Sentence => "Sentence",
        Mode::Paragraph => "Paragraph",
        Mode::Full => "Full",
    }
}
