//! Builds the inline keyboard shown under the Statistics message.

use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

/// `STATS_REFRESH_CALLBACK_DATA` is the callback_data of the inline button
/// that reloads the Statistics message in place.
/// `register_stats_handlers` binds a callback query handler to this exact
/// value.
pub const STATS_REFRESH_CALLBACK_DATA: &str = "stats_refresh";

/// Builds the inline keyboard shown under the Statistics message.
pub fn stats_menu() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "\u{1F504} Refresh",
        STATS_REFRESH_CALLBACK_DATA,
    )]])
}
