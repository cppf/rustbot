//! Queries SQLite for a fresh `Stats` snapshot.

use crate::db::Db;
use crate::stats::Stats;
use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Utc};

/// How recently a user must have been seen to count as "currently using
/// the bot".
const ACTIVE_WINDOW_MINUTES: i64 = 5;

fn scan_message_count(db: &Db, since: DateTime<Utc>) -> rusqlite::Result<i64> {
    let conn = db.lock().expect("db lock poisoned");
    conn.query_row(
        "SELECT COUNT(*) FROM messages WHERE created_at >= ?1",
        [since.to_rfc3339()],
        |row| row.get(0),
    )
}

/// Subtracts exactly one year from `dt`, mirroring Go's
/// `time.AddDate(-1, 0, 0)`: if `dt` is Feb 29 and the target year is not
/// a leap year, it rolls over to Mar 1 rather than producing an invalid
/// date.
fn minus_one_year(dt: DateTime<Utc>) -> DateTime<Utc> {
    let target_year = dt.year() - 1;
    match Utc.with_ymd_and_hms(
        target_year,
        dt.month(),
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second(),
    ) {
        chrono::LocalResult::Single(d) => d,
        _ => {
            // dt is Feb 29 and target_year is not a leap year: roll to Mar 1.
            Utc.with_ymd_and_hms(target_year, 3, 1, dt.hour(), dt.minute(), dt.second())
                .single()
                .expect("Mar 1 is always a valid date")
        }
    }
}

/// Queries SQLite for a fresh `Stats` snapshot as of now.
pub fn load_stats(db: &Db) -> rusqlite::Result<Stats> {
    let mut s = Stats::default();
    let now = Utc::now();

    {
        let conn = db.lock().expect("db lock poisoned");
        let active_since = now - Duration::minutes(ACTIVE_WINDOW_MINUTES);
        s.active_users = conn.query_row(
            "SELECT COUNT(*) FROM users WHERE last_seen >= ?1",
            [active_since.to_rfc3339()],
            |row| row.get(0),
        )?;

        s.unique_users = conn.query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))?;

        s.messages_lifetime =
            conn.query_row("SELECT COUNT(*) FROM messages", [], |row| row.get(0))?;
    }

    let midnight = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .expect("midnight is always a valid time")
        .and_utc();

    s.messages_today = scan_message_count(db, midnight)?;
    s.messages_24h = scan_message_count(db, now - Duration::hours(24))?;
    s.messages_7d = scan_message_count(db, now - Duration::days(7))?;
    s.messages_30d = scan_message_count(db, now - Duration::days(30))?;
    s.messages_1y = scan_message_count(db, minus_one_year(now))?;

    Ok(s)
}
