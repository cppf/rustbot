//! Domain types and configuration shared by every other crate: `Mode`,
//! its labels/parsing, config loading, and Telegram's length limits.

pub mod config;
pub mod limits;
pub mod mode;
pub mod mode_label;
pub mod mode_parse;

pub use config::{load_config, load_config_with_fallback, Config, ConfigError};
pub use limits::{TELEGRAM_CAPTION_LIMIT, TELEGRAM_MESSAGE_LIMIT};
pub use mode::{Mode, DEFAULT_MODE};
pub use mode_label::mode_label;
pub use mode_parse::parse_mode;
