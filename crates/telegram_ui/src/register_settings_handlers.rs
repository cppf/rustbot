//! Builds the dptree branch handling the Word/Sentence/Paragraph/Full
//! buttons on the settings keyboard.

use bot_core::Mode;
use teloxide::dispatching::DpHandlerDescription;
use teloxide::dptree;
use teloxide::prelude::*;
use teloxide::types::KeyboardMarkup;

use crate::mode_handler::mode_handler;
use crate::settings_menu::SettingsMenuButtons;
use crate::store::Store;

/// Builds the dptree branch that wires up the Word/Sentence/Paragraph/Full
/// buttons on the settings keyboard. `settings_btns` is used to match each
/// button's exact text.
pub fn settings_handlers(
    settings_btns: &SettingsMenuButtons,
) -> Handler<'static, DependencyMap, ResponseResult<()>, DpHandlerDescription> {
    let word_text = settings_btns.word.clone();
    let sentence_text = settings_btns.sentence.clone();
    let paragraph_text = settings_btns.paragraph.clone();
    let full_text = settings_btns.full.clone();

    dptree::entry()
        .branch(
            Update::filter_message()
                .filter(move |msg: Message| msg.text() == Some(word_text.as_str()))
                .endpoint(move |bot: Bot, msg: Message, store: Store, main_menu: KeyboardMarkup| {
                    mode_handler(bot, msg, store, main_menu, Mode::Word)
                }),
        )
        .branch(
            Update::filter_message()
                .filter(move |msg: Message| msg.text() == Some(sentence_text.as_str()))
                .endpoint(move |bot: Bot, msg: Message, store: Store, main_menu: KeyboardMarkup| {
                    mode_handler(bot, msg, store, main_menu, Mode::Sentence)
                }),
        )
        .branch(
            Update::filter_message()
                .filter(move |msg: Message| msg.text() == Some(paragraph_text.as_str()))
                .endpoint(move |bot: Bot, msg: Message, store: Store, main_menu: KeyboardMarkup| {
                    mode_handler(bot, msg, store, main_menu, Mode::Paragraph)
                }),
        )
        .branch(
            Update::filter_message()
                .filter(move |msg: Message| msg.text() == Some(full_text.as_str()))
                .endpoint(move |bot: Bot, msg: Message, store: Store, main_menu: KeyboardMarkup| {
                    mode_handler(bot, msg, store, main_menu, Mode::Full)
                }),
        )
}
