//! Unit tests for the stats crate. Uses an in-memory SQLite database, so
//! these run in CI with no network or filesystem dependency.

use std::sync::{Arc, Mutex};

use chrono::{TimeZone, Utc};
use rusqlite::Connection;

use stats::{db_schema::create_schema, format_stats, load_stats, track_message, Stats};

fn in_memory_db() -> stats::Db {
    let conn = Connection::open_in_memory().expect("open in-memory sqlite");
    create_schema(&conn).expect("create schema");
    Arc::new(Mutex::new(conn))
}

#[test]
fn tracking_messages_updates_counts() {
    let db = in_memory_db();

    track_message(&db, 111);
    track_message(&db, 111);
    track_message(&db, 222);

    let s = load_stats(&db).expect("load stats");
    assert_eq!(s.unique_users, 2);
    assert_eq!(s.messages_lifetime, 3);
    assert_eq!(s.active_users, 2); // both seen within the last 5 minutes
    assert_eq!(s.messages_today, 3);
}

#[test]
fn format_stats_includes_all_fields() {
    let s = Stats {
        active_users: 2,
        unique_users: 5,
        messages_today: 10,
        messages_24h: 12,
        messages_7d: 50,
        messages_30d: 200,
        messages_1y: 1000,
        messages_lifetime: 1500,
    };
    let updated_at = Utc.with_ymd_and_hms(2026, 7, 29, 14, 30, 0).unwrap();
    let out = format_stats(&s, updated_at);

    assert!(out.contains("<b>\u{1F4CA} Statistics</b>"));
    assert!(out.contains("<b>2</b>"));
    assert!(out.contains("<b>1500</b>"));
    assert!(out.contains("2026-07-29 14:30:00 UTC"));
}
