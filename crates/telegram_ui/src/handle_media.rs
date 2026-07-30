//! Re-sends received media with its caption rendered in the selected mode.

use bot_core::{Mode, TELEGRAM_CAPTION_LIMIT};
use rendering::{render, split_for_telegram};
use teloxide::prelude::*;
use teloxide::types::{KeyboardMarkup, ParseMode};

use crate::resend_media::resend_media;

/// Re-sends the received media with its caption rendered in the selected
/// mode. If the rendered caption exceeds Telegram's caption limit, the
/// media is sent with the first chunk as caption and any remaining
/// chunks are sent as follow-up text messages so no content is lost.
pub async fn handle_media(
    bot: &Bot,
    msg: &Message,
    mode: Mode,
    menu: &KeyboardMarkup,
) -> ResponseResult<()> {
    let chunks: Vec<String> = match msg.caption() {
        Some(caption) => {
            let rendered = render(mode, caption);
            split_for_telegram(&rendered, TELEGRAM_CAPTION_LIMIT)
        }
        None => Vec::new(),
    };

    let first_caption = chunks.first().cloned().unwrap_or_default();
    let media_reply_markup = if chunks.len() <= 1 {
        Some(menu.clone())
    } else {
        None
    };

    resend_media(bot, msg, &first_caption, media_reply_markup).await?;

    if chunks.len() <= 1 {
        return Ok(());
    }

    let rest = &chunks[1..];
    for (i, chunk) in rest.iter().enumerate() {
        let mut req = bot
            .send_message(msg.chat.id, chunk)
            .parse_mode(ParseMode::MarkdownV2);
        if i == rest.len() - 1 {
            req = req.reply_markup(menu.clone());
        }
        req.await?;
    }

    Ok(())
}
