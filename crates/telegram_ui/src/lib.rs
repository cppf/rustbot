//! The bot-facing layer: keyboards, handler registration, media
//! re-sending, and webhook construction, built on teloxide.

pub mod handle_media;
pub mod main_menu;
pub mod mode_handler;
pub mod new_bot;
pub mod register_content_handlers;
pub mod register_main_menu_handlers;
pub mod register_settings_handlers;
pub mod resend_media;
pub mod send_rendered;
pub mod settings_menu;
pub mod store;
pub mod webhook_url;
pub mod welcome_text;

pub use main_menu::{new_main_menu, MainMenuButtons};
pub use new_bot::new_bot;
pub use register_content_handlers::content_handlers;
pub use register_main_menu_handlers::main_menu_handlers;
pub use register_settings_handlers::settings_handlers;
pub use settings_menu::{new_settings_menu, SettingsMenuButtons};
pub use store::{new_store, Store};
pub use webhook_url::webhook_url;
pub use welcome_text::WELCOME_TEXT;
