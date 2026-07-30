//! Cross-cutting handler that records every incoming update as a message.
//!
//! teloxide's dptree dispatch model is branch-based ("try this handler,
//! and if it declines the update, try the next"), not middleware in the
//! Go/Python sense of an always-run wrapper. The idiomatic dptree
//! mechanism for "run this for every update, then always continue" is
//! [`dptree::map_async`]: unlike `.filter`, it cannot reject an update —
//! it always passes control onward, optionally inserting a value into the
//! dependency map. `track_update` is chained at the very front of the
//! dispatch tree in `telegram_ui`'s schema construction, so it runs
//! before any branch gets a chance to handle (or ignore) the update.

use teloxide::types::Update;

use crate::db::Db;
use crate::track_message::track_message;

/// Records the sender of `update` as one message, then returns `()` so
/// dispatch always continues to the next step in the tree.
pub async fn track_update(update: Update, db: Db) {
    if let Some(user) = update.from() {
        track_message(&db, user.id.0 as i64);
    }
}
