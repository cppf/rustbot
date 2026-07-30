//! Pure text-transformation logic: splitting text into words, sentences,
//! or paragraphs, wrapping units in Telegram monospace, and splitting long
//! output to fit Telegram's message length limits. No Telegram library
//! dependency — this crate only manipulates strings.

pub mod best_split_point;
pub mod closing_mark;
pub mod last_index_after;
pub mod last_sentence_break;
pub mod last_word_break;
pub mod render;
pub mod render_units;
pub mod split_for_telegram;
pub mod split_paragraphs;
pub mod split_sentences;
pub mod split_surrounding_space;
pub mod split_words;
pub mod wrap_code;

pub use render::render;
pub use split_for_telegram::split_for_telegram;
