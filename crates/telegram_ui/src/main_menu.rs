//! Builds the main persistent keyboard (Start, Settings).

use teloxide::types::{KeyboardButton, KeyboardMarkup};

/// Groups the button labels on the main persistent keyboard so handlers
/// can be registered against the same values used to build the menu.
#[derive(Debug, Clone)]
pub struct MainMenuButtons {
    pub start: String,
    pub settings: String,
}

/// Builds the main persistent keyboard (Start, Settings) and returns it
/// along with its button labels for handler registration.
pub fn new_main_menu() -> (KeyboardMarkup, MainMenuButtons) {
    let btns = MainMenuButtons {
        start: "\u{25B6}\u{FE0F} Start".to_string(),
        settings: "\u{2699}\u{FE0F} Settings".to_string(),
    };
    let menu = KeyboardMarkup::new([[
        KeyboardButton::new(btns.start.clone()),
        KeyboardButton::new(btns.settings.clone()),
    ]])
    .resize_keyboard();
    (menu, btns)
}
