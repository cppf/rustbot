//! Unit tests for bot_core.

use bot_core::{mode_label, parse_mode, Mode};

#[test]
fn mode_label_and_parse_mode_round_trip() {
    for mode in [Mode::Word, Mode::Sentence, Mode::Paragraph, Mode::Full] {
        let label = mode_label(mode);
        assert_eq!(parse_mode(label), Some(mode));
    }
}

#[test]
fn parse_mode_rejects_unknown_label() {
    assert_eq!(parse_mode("Not a real mode"), None);
}
