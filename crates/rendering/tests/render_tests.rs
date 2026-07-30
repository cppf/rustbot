//! Unit tests for the rendering crate's pure text-transformation logic.
//! These run in CI via `cargo test --workspace` with no network or
//! Telegram dependency required.

use bot_core::Mode;
use rendering::{render, split_for_telegram};

#[test]
fn full_mode_wraps_whole_text() {
    assert_eq!(render(Mode::Full, "hello world"), "`hello world`");
}

#[test]
fn full_mode_passes_through_whitespace_only() {
    assert_eq!(render(Mode::Full, "   "), "   ");
}

#[test]
fn full_mode_escapes_backticks() {
    assert_eq!(render(Mode::Full, "code `here`"), "`code 'here'`");
}

#[test]
fn word_mode_wraps_each_word() {
    assert_eq!(render(Mode::Word, "hello world foo"), "`hello` `world` `foo`");
}

#[test]
fn sentence_mode_wraps_each_sentence() {
    assert_eq!(
        render(Mode::Sentence, "Hi there. How are you? Fine!"),
        "`Hi there.` `How are you?` `Fine!`"
    );
}

#[test]
fn paragraph_mode_wraps_each_paragraph() {
    assert_eq!(
        render(Mode::Paragraph, "para one\n\npara two"),
        "`para one`\n\n`para two`"
    );
}

#[test]
fn sentence_mode_keeps_closing_marks_attached() {
    let result = render(Mode::Sentence, "He said \"Hello!\" and left.");
    // The closing quote stays attached to the sentence-ending punctuation
    // it follows, rather than starting a new unit.
    assert!(result.contains("Hello!\""));
}

#[test]
fn split_for_telegram_respects_limit() {
    let long_text = "word ".repeat(1000);
    let rendered = render(Mode::Full, &long_text);
    let chunks = split_for_telegram(&rendered, 4096);
    assert!(chunks.len() > 1);
    for chunk in &chunks {
        assert!(chunk.chars().count() <= 4096);
    }
    // No content should be lost across the split.
    let total_chars: usize = chunks.iter().map(|c| c.chars().count()).sum();
    assert_eq!(total_chars, rendered.chars().count());
}

#[test]
fn split_for_telegram_short_text_is_single_chunk() {
    let chunks = split_for_telegram("short text", 4096);
    assert_eq!(chunks, vec!["short text".to_string()]);
}

#[test]
fn split_for_telegram_handles_multibyte_unicode() {
    // Emoji and other astral-plane characters are each a single `char`
    // (Unicode scalar value), matching the rune/codepoint semantics used
    // throughout this port. This guards against byte-vs-char slicing bugs.
    let text = "a".repeat(4090) + "🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉";
    let chunks = split_for_telegram(&text, 4096);
    let total_chars: usize = chunks.iter().map(|c| c.chars().count()).sum();
    assert_eq!(total_chars, text.chars().count());
    for chunk in &chunks {
        assert!(chunk.chars().count() <= 4096);
    }
}
