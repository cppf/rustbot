//! Converts text into Telegram monospace formatting according to `Mode`.

use bot_core::Mode;

use crate::render_units::render_units;
use crate::split_paragraphs::split_paragraphs;
use crate::split_sentences::split_sentences;
use crate::split_words::split_words;
use crate::wrap_code::wrap_code;

/// Converts `text` into Telegram monospace formatting according to `mode`.
///
/// Each unit (word, sentence, paragraph, or the whole text) is wrapped
/// individually in its own code span, separated by the same whitespace
/// that originally separated the units, so content and spacing are
/// preserved.
pub fn render(mode: Mode, text: &str) -> String {
    match mode {
    Mode::Word => render_units(text, split_words),
    Mode::Sentence => render_units(text, split_sentences),
    Mode::Paragraph => render_units(text, split_paragraphs),
    Mode::Full => {
        if text.trim().is_empty() {
            text.to_string()
        } else {
            wrap_code(text)
        }
    }
    }
