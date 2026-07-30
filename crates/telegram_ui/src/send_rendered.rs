//! Renders text under a mode and sends it, splitting across messages if needed.

use bot_core::{Mode, TELEGRAM_MESSAGE_LIMIT};
use rendering::{render, split_for_telegram};
use teloxide::prelude::*;
use teloxide::types::{KeyboardMarkup, ParseMode};

/// Renders `text` under `mode` and sends it, splitting across multiple
/// messages if needed to respect Telegram's length limit.
pub async fn send_rendered(
    bot: &Bot,
    msg: &Message,
    mode: Mode,
    text: &str,
    menu: &KeyboardMarkup,
) -> ResponseResult<()> {
    let rendered = render(mode, text);
    let chunks = split_for_telegram(&rendered, TELEGRAM_MESSAGE_LIMIT);
    for (i, chunk) in chunks.iter().enumerate() {
        let mut req = bot.send_message(msg.chat.id, chunk).parse_mode(ParseMode::Markdown);
        if i == chunks.len() - 1 {
            req = req.reply_markup(menu.clone());
        }
        req.await?;
    }
    Ok(())
}
