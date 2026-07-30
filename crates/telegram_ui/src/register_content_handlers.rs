/// Builds the dptree branch that wires up handling of incoming text
/// messages and media (photos, videos, voice notes, etc.). Text messages
/// that are also commands (e.g. `/start`) are excluded, since those are
/// matched earlier in the schema by `register_main_menu_handlers`.
use teloxide::dispatching::DependencyMap;

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
