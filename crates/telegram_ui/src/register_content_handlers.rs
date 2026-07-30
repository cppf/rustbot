//! Builds the dptree branch handling incoming text messages and media.

use teloxide::dispatching::{DpHandlerDescription, DependencyMap};
use teloxide::dptree;
use teloxide::prelude::*;
use teloxide::types::KeyboardMarkup;

use crate::handle_media::handle_media;
use crate::send_rendered::send_rendered;
use crate::store::Store;

async fn on_text(
    bot: Bot,
    msg: Message,
    store: Store,
    main_menu: KeyboardMarkup,
) -> ResponseResult<()> {
    let mode = msg
        .from
        .as_ref()
        .map(|u| store.get(u.id.0 as i64))
        .unwrap_or(bot_core::DEFAULT_MODE);

    if let Some(text) = msg.text() {
        send_rendered(&bot, &msg, mode, text, &main_menu).await?;
    }

    Ok(())
}

async fn on_media(
    bot: Bot,
    msg: Message,
    store: Store,
    main_menu: KeyboardMarkup,
) -> ResponseResult<()> {
    let mode = msg
        .from
        .as_ref()
        .map(|u| store.get(u.id.0 as i64))
        .unwrap_or(bot_core::DEFAULT_MODE);

    handle_media(&bot, &msg, mode, &main_menu).await?;
    Ok(())
}

fn is_media(msg: &Message) -> bool {
    msg.photo().is_some()
        || msg.video().is_some()
        || msg.animation().is_some()
        || msg.audio().is_some()
        || msg.voice().is_some()
        || msg.document().is_some()
        || msg.sticker().is_some()
        || msg.video_note().is_some()
}

fn is_non_command_text(msg: &Message) -> bool {
    match msg.text() {
        Some(text) => !text.starts_with('/'),
        None => false,
    }
}

/// Builds the dptree branch that wires up handling of incoming text
/// messages and media (photos, videos, voice notes, etc.). Text messages
/// that are also commands (e.g. `/start`) are excluded, since those are
/// matched earlier in the schema by `register_main_menu_handlers`.
pub fn content_handlers() -> Handler<'static, DependencyMap, ResponseResult<()>, DpHandlerDescription> {
    dptree::entry()
        .branch(
            Update::filter_message()
                .filter(|msg: Message| is_non_command_text(&msg))
                .endpoint(on_text),
        )
        .branch(
            Update::filter_message()
                .filter(|msg: Message| is_media(&msg))
                .endpoint(on_media),
        )
}
