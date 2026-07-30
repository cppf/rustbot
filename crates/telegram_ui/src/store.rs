//! Per-user mode storage, in memory.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use bot_core::{Mode, DEFAULT_MODE};

/// Holds each user's selected mode in memory, keyed by Telegram user ID.
///
/// Safe for concurrent use from multiple tokio tasks — teloxide's
/// `Dispatcher` runs handlers concurrently, so a bare `HashMap` would not
/// be safe to share without synchronization.
#[derive(Debug, Clone, Default)]
pub struct Store {
    modes: Arc<Mutex<HashMap<i64, Mode>>>,
}

impl Store {
    /// Returns the mode for `user_id`, or [`DEFAULT_MODE`] if none has
    /// been set.
    pub fn get(&self, user_id: i64) -> Mode {
        let modes = self.modes.lock().expect("store lock poisoned");
        modes.get(&user_id).copied().unwrap_or(DEFAULT_MODE)
    }

    /// Stores the mode for `user_id`.
    pub fn set(&self, user_id: i64, mode: Mode) {
        let mut modes = self.modes.lock().expect("store lock poisoned");
        modes.insert(user_id, mode);
    }
}

/// Creates an empty store.
pub fn new_store() -> Store {
    Store::default()
}
