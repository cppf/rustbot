//! SQLite-backed usage statistics: tracking every incoming update,
//! loading aggregate counts, and the in-chat Statistics page.

pub mod db;
pub mod db_schema;
pub mod format_stats;
pub mod is_not_modified_err;
pub mod load_stats;
pub mod register_stats_handlers;
pub mod stats;
pub mod stats_menu;
pub mod track_message;
pub mod track_middleware;

pub use db::{init_db, Db};
pub use format_stats::format_stats;
pub use load_stats::load_stats;
pub use register_stats_handlers::{on_stats_button, on_stats_refresh};
pub use stats::Stats;
pub use stats_menu::{stats_menu, STATS_REFRESH_CALLBACK_DATA};
pub use track_message::track_message;
pub use track_middleware::track_update;
