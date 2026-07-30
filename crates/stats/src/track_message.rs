//! Records one incoming Telegram update as a message event.

use chrono::Utc;
use rusqlite::params;

use crate::db::Db;

/// Records one incoming Telegram update as a message event for `user_id`,
/// and upserts that user's first/last seen timestamps. Safe to call from
/// every update handler (text, caption, media, sticker, document, etc.) —
/// each call counts as exactly one message.
pub fn track_message(db: &Db, user_id: i64) {
    let now = Utc::now().to_rfc3339();

    let conn = match db.lock() {
        Ok(c) => c,
        Err(e) => {
            log::error!("track_message: db lock poisoned: {e}");
            return;
        }
    };

    if let Err(e) = conn.execute(
        "INSERT INTO users (user_id, first_seen, last_seen) VALUES (?1, ?2, ?2)
         ON CONFLICT(user_id) DO UPDATE SET last_seen = excluded.last_seen",
        params![user_id, now],
    ) {
        log::error!("track_message: upsert user {user_id}: {e}");
        return;
    }

    if let Err(e) = conn.execute(
        "INSERT INTO messages (user_id, created_at) VALUES (?1, ?2)",
        params![user_id, now],
    ) {
        log::error!("track_message: insert message for user {user_id}: {e}");
    }
}
