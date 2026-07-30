//! Entry point — wires everything together.

use std::net::SocketAddr;

use teloxide::dispatching::DpHandlerDescription;
use teloxide::prelude::*;
use teloxide::update_listeners::webhooks;

use bot_core::load_config_with_fallback;
use stats::{init_db, on_stats_button, on_stats_refresh, track_update, Db};
use telegram_ui::{
    content_handlers, main_menu_handlers, new_bot, new_main_menu, new_settings_menu,
    settings_handlers, webhook_url, MainMenuButtons, SettingsMenuButtons, Store,
};

// deploy_config.rs lives at the workspace root — see that file for the
// one thing to edit before deploying (your BOT_TOKEN from @BotFather).
#[path = "../deploy_config.rs"]
mod deploy_config;

/// Builds the dptree branch that wires the Statistics button and the
/// Refresh inline button, using the shared `Db` handle injected as a
/// dependency.
fn stats_handlers(
    settings_btns: &SettingsMenuButtons,
) -> Handler<'static, DependencyMap, ResponseResult<()>, DpHandlerDescription> {
    let stats_text = settings_btns.stats.clone();

    dptree::entry()
        .branch(
            Update::filter_message()
                .filter(move |msg: Message| msg.text() == Some(stats_text.as_str()))
                .endpoint(on_stats_button),
        )
        .branch(Update::filter_callback_query().endpoint(on_stats_refresh))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("starting bot");

    let cfg = load_config_with_fallback(
        Some(deploy_config::BOT_TOKEN),
        Some(deploy_config::PORT),
        Some(deploy_config::RAILWAY_PUBLIC_DOMAIN),
    )
    .expect("failed to load config");

    let db: Db = init_db().expect("failed to initialize stats database");

    let bot = new_bot(&cfg.token);

    let (main_menu, main_btns): (_, MainMenuButtons) = new_main_menu();
    let (settings_menu, settings_btns): (_, SettingsMenuButtons) = new_settings_menu();
    let store: Store = telegram_ui::new_store();

    // track_update runs for every update before any branch below gets a
    // chance to handle (or ignore) it; map_async cannot reject an
    // update, so dispatch always continues to the branches that follow.
    let schema = dptree::entry()
        .map_async(|update: Update, db: Db| async move {
            track_update(update, db).await;
        })
        .branch(main_menu_handlers(&main_btns, &settings_btns))
        .branch(settings_handlers(&settings_btns))
        .branch(stats_handlers(&settings_btns))
        .branch(content_handlers());

    let addr: SocketAddr = ([0, 0, 0, 0], cfg.port).into();
    let url = webhook_url(&cfg.domain, &cfg.token)
        .parse()
        .expect("invalid webhook URL");

    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("failed to set up webhook listener");

    log::info!("bot started");

    Dispatcher::builder(bot, schema)
        .dependencies(dptree::deps![main_menu, settings_menu, store, db])
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error occurred in the dispatcher"),
        )
        .await;
}
