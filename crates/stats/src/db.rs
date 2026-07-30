//! Opens the shared SQLite handle used for statistics storage.

use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use crate::db_schema::create_schema;

/// `DB_PATH` is where the SQLite file lives. Railway's filesystem is
/// ephemeral across deploys unless a volume is mounted at this path;
/// mount a volume at `/data` in the Railway service settings to persist
/// stats across deploys.
pub const DB_PATH: &str = "/data/stats.db";

/// A shared, thread-safe handle to the statistics database.
///
/// teloxide's `Dispatcher` runs handlers concurrently as separate tokio
/// tasks (unlike a single-threaded event loop), so — unlike the Go
/// version's `SetMaxOpenConns(1)` pool or the Python port's reliance on a
/// single-threaded event loop — a bare `rusqlite::Connection` cannot be
/// shared directly: it is `Send` but not `Sync`. Wrapping it in
/// `Arc<Mutex<_>>` serializes access the same way, just made explicit by
/// the type system instead of assumed.
pub type Db = Arc<Mutex<Connection>>;

/// Opens the SQLite database at [`DB_PATH`] and ensures the schema exists.
pub fn init_db() -> rusqlite::Result<Db> {
    let conn = Connection::open(DB_PATH)?;
    // rusqlite connections default to a 5000ms busy_timeout already; WAL
    // mode is set explicitly to allow concurrent readers while a write is
    // in progress, matching the Go version's intent.
    conn.pragma_update(None, "journal_mode", "WAL")?;
    create_schema(&conn)?;
    Ok(Arc::new(Mutex::new(conn)))
}
