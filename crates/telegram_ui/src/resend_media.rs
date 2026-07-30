//! Re-sends the same media file back to the chat, preserving its type.

use teloxide::prelude::*;
use teloxide::types::{InputFile, KeyboardMarkup, ParseMode};

/// Sends the same media file back to `msg`'s chat with the given
/// `caption`, preserving the original media type. Uses Telegram's
/// `file.id` so the file is re-sent without re-uploading it.
pub async fn resend_media(
    bot: &Bot,
    msg: &Message,
    caption: &str,
    reply_markup: Option<KeyboardMarkup>,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    if let Some(sizes) = msg.photo() {
        // The last entry is the highest-resolution size, mirroring what
        // Telegram itself considers "the" photo for re-send purposes.
        if let Some(largest) = sizes.last() {
            let mut req = bot
                .send_photo(chat_id, InputFile::file_id(largest.file.id.clone()))
                .caption(caption)
                .parse_mode(ParseMode::Markdown);
            if let Some(menu) = reply_markup {
                req = req.reply_markup(menu);
            }
            req.await?;
        }
    } else if let Some(video) = msg.video() {
        let mut req = bot
            .send_video(chat_id, InputFile::file_id(video.file.id.clone()))
            .caption(caption)
            .parse_mode(ParseMode::Markdown);
        if let Some(menu) = reply_markup {
            req = req.reply_markup(menu);
        }
        req.await?;
    } else if let Some(animation) = msg.animation() {
        let mut req = bot
            .send_animation(chat_id, InputFile::file_id(animation.file.id.clone()))
            .caption(caption)
            .parse_mode(ParseMode::Markdown);
        if let Some(menu) = reply_markup {
            req = req.reply_markup(menu);
        }
        req.await?;
    } else if let Some(audio) = msg.audio() {
        let mut req = bot
            .send_audio(chat_id, InputFile::file_id(audio.file.id.clone()))
            .caption(caption)
            .parse_mode(ParseMode::Markdown);
        if let Some(menu) = reply_markup {
            req = req.reply_markup(menu);
        }
        req.await?;
    } else if let Some(voice) = msg.voice() {
        let mut req = bot
            .send_voice(chat_id, InputFile::file_id(voice.file.id.clone()))
            .caption(caption)
            .parse_mode(ParseMode::Markdown);
        if let Some(menu) = reply_markup {
            req = req.reply_markup(menu);
        }
        req.await?;
    } else if let Some(document) = msg.document() {
        let mut req = bot
            .send_document(chat_id, InputFile::file_id(document.file.id.clone()))
            .caption(caption)
            .parse_mode(ParseMode::Markdown);
        if let Some(menu) = reply_markup {
            req = req.reply_markup(menu);
        }
        req.await?;
    } else if let Some(sticker) = msg.sticker() {
        // Stickers have no caption in the Bot API.
        let mut req = bot.send_sticker(chat_id, InputFile::file_id(sticker.file.id.clone()));
        if let Some(menu) = reply_markup {
            req = req.reply_markup(menu);
        }
        req.await?;
    } else if let Some(video_note) = msg.video_note() {
        // Video notes have no caption in the Bot API.
        let mut req =
            bot.send_video_note(chat_id, InputFile::file_id(video_note.file.id.clone()));
        if let Some(menu) = reply_markup {
            req = req.reply_markup(menu);
        }
        req.await?;
    } else {
        let mut req = bot.send_message(chat_id, "Unsupported media type.");
        if let Some(menu) = reply_markup {
            req = req.reply_markup(menu);
        }
        req.await?;
    }

    Ok(())
}
