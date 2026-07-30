//! Handler factory for mode-select buttons.

use bot_core::{mode_label, Mode};
use teloxide::prelude::*;
use teloxide::types::KeyboardMarkup;

use crate::store::Store;

/// Sets the sender's mode to `mode` and confirms the change, returning to
/// the main menu.
pub async fn mode_handler(
    bot: Bot,
    msg: Message,
    store: Store,
    main_menu: KeyboardMarkup,
    mode: Mode,
) -> ResponseResult<()> {
    if let Some(user) = msg.from.as_ref() {
        store.set(user.id.0 as i64, mode);
    }
    bot.send_message(msg.chat.id, format!("Mode set to {}.", mode_label(mode)))
        .reply_markup(main_menu)
        .await?;
    Ok(())
}
