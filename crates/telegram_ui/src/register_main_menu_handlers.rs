//! Builds the dptree branch handling /start plus the Start, Settings, and
//! Back buttons.

use teloxide::dispatching::{DpHandlerDescription, UpdateFilterExt};
use teloxide::dptree;
use teloxide::prelude::*;
use teloxide::types::KeyboardMarkup;
use teloxide::utils::command::BotCommands;

use crate::main_menu::MainMenuButtons;
use crate::settings_menu::SettingsMenuButtons;
use crate::store::Store;
use crate::welcome_text::WELCOME_TEXT;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    Start,
}

async fn on_start(bot: Bot, msg: Message, main_menu: KeyboardMarkup) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, WELCOME_TEXT)
        .reply_markup(main_menu)
        .await?;
    Ok(())
}

async fn on_settings_button(
    bot: Bot,
    msg: Message,
    store: Store,
    settings_menu: KeyboardMarkup,
) -> ResponseResult<()> {
    let current = msg
        .from
        .as_ref()
        .map(|u| store.get(u.id.0 as i64))
        .unwrap_or(bot_core::DEFAULT_MODE);
    bot.send_message(
        msg.chat.id,
        format!(
            "Current mode: {}\n\nChoose a mode:",
            bot_core::mode_label(current)
        ),
    )
    .reply_markup(settings_menu)
    .await?;
    Ok(())
}

async fn on_back_button(bot: Bot, msg: Message, main_menu: KeyboardMarkup) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Back to main menu.")
        .reply_markup(main_menu)
        .await?;
    Ok(())
}

/// Builds the dptree branch that wires up /start plus the Start,
/// Settings, and Back buttons. `main_btns`/`settings_btns` are used to
/// match each button's exact text.
pub fn main_menu_handlers(
    main_btns: &MainMenuButtons,
    settings_btns: &SettingsMenuButtons,
) -> Handler<'static, DependencyMap, ResponseResult<()>, DpHandlerDescription> {
    let start_text = main_btns.start.clone();
    let settings_text = main_btns.settings.clone();
    let back_text = settings_btns.back.clone();

    dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(on_start),
        )
        .branch(
            Update::filter_message()
                .filter(move |msg: Message| msg.text() == Some(start_text.as_str()))
                .endpoint(on_start),
        )
        .branch(
            Update::filter_message()
                .filter(move |msg: Message| msg.text() == Some(settings_text.as_str()))
                .endpoint(on_settings_button),
        )
        .branch(
            Update::filter_message()
                .filter(move |msg: Message| msg.text() == Some(back_text.as_str()))
                .endpoint(on_back_button),
        )
}
