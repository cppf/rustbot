//! Wires the Statistics button on the settings keyboard and the Refresh
//! inline button on the resulting message.

use teloxide::prelude::*;
use teloxide::types::{CallbackQuery, Message};
use teloxide::types::ParseMode;
use teloxide::RequestError;

use crate::db::Db;
use crate::format_stats::format_stats;
use crate::is_not_modified_err::is_not_modified_err;
use crate::load_stats::load_stats;
use crate::stats_menu::stats_menu;

/// Handles the Statistics button: loads a snapshot and sends it as a new
/// message with the Refresh inline keyboard attached.
pub async fn on_stats_button(bot: Bot, msg: Message, db: Db) -> ResponseResult<()> {
    let s = match load_stats(&db) {
        Ok(s) => s,
        Err(e) => {
            log::error!("stats: load: {e}");
            bot.send_message(
                msg.chat.id,
                "Couldn't load statistics right now \u{2014} please try again.",
            )
            .await?;
            return Ok(());
        }
    };

    bot.send_message(msg.chat.id, format_stats(&s, chrono::Utc::now()))
        .parse_mode(ParseMode::Html)
        .reply_markup(stats_menu())
        .await?;
    Ok(())
}

/// Handles the Refresh inline button: reloads the Statistics message in
/// place. Telegram requires every callback query to be answered exactly
/// once (via `answer_callback_query`) or the client shows a loading
/// spinner indefinitely — every branch below does so.
pub async fn on_stats_refresh(bot: Bot, q: CallbackQuery, db: Db) -> ResponseResult<()> {
    let s = match load_stats(&db) {
        Ok(s) => s,
        Err(e) => {
            log::error!("stats: refresh: {e}");
            bot.answer_callback_query(q.id).text("Couldn't refresh \u{2014} try again.").await?;
            return Ok(());
        }
    };

    let Some(message) = q.regular_message() else {
        // The original message is inaccessible (too old) or this came
        // from inline mode; there is nothing we can edit in place.
        bot.answer_callback_query(q.id).text("Couldn't refresh \u{2014} try again.").await?;
        return Ok(());
    };

    let text = format_stats(&s, chrono::Utc::now());
    let edit_result = bot
        .edit_message_text(message.chat.id, message.id, text)
        .parse_mode(ParseMode::Html)
        .reply_markup(stats_menu())
        .await;

    match edit_result {
        Ok(_) => {
            bot.answer_callback_query(q.id).text("Refreshed \u{2705}").await?;
        }
        Err(e) => {
            if is_not_modified_err(&e) {
                // Telegram errors if the content is byte-for-byte
                // identical to the current message (e.g. two refreshes
                // within the same second). That's not a real failure.
                bot.answer_callback_query(q.id).text("Already up to date.").await?;
            } else {
                log::error!("stats: edit: {e}");
                bot.answer_callback_query(q.id).text("Couldn't refresh \u{2014} try again.").await?;
            }
        }
    }

    Ok(())
}

// Re-exported so telegram_ui's schema construction can reference the
// error type without depending on teloxide directly for it.
pub type StatsRequestError = RequestError;
