//! Builds the settings keyboard (Word/Sentence/Paragraph/Full/Stats/Back).

use bot_core::{mode_label, Mode};
use teloxide::types::{KeyboardButton, KeyboardMarkup};

/// Groups the button labels on the settings keyboard so handlers can be
/// registered against the same values used to build the menu.
#[derive(Debug, Clone)]
pub struct SettingsMenuButtons {
    pub word: String,
    pub sentence: String,
    pub paragraph: String,
    pub full: String,
    pub stats: String,
    pub back: String,
}

/// Builds the settings keyboard (one button per `Mode`, plus Statistics
/// and Back) and returns it along with its button labels for handler
/// registration.
pub fn new_settings_menu() -> (KeyboardMarkup, SettingsMenuButtons) {
    let btns = SettingsMenuButtons {
        word: mode_label(Mode::Word).to_string(),
        sentence: mode_label(Mode::Sentence).to_string(),
        paragraph: mode_label(Mode::Paragraph).to_string(),
        full: mode_label(Mode::Full).to_string(),
        stats: "\u{1F4CA} Statistics".to_string(),
        back: "\u{2B05}\u{FE0F} Back".to_string(),
    };

    let menu = KeyboardMarkup::new([
        vec![
            KeyboardButton::new(btns.word.clone()),
            KeyboardButton::new(btns.sentence.clone()),
        ],
        vec![
            KeyboardButton::new(btns.paragraph.clone()),
            KeyboardButton::new(btns.full.clone()),
        ],
        vec![KeyboardButton::new(btns.stats.clone())],
        vec![KeyboardButton::new(btns.back.clone())],
    ])
    .resize_keyboard();

    (menu, btns)
}
