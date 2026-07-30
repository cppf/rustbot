//! Renders a `Stats` snapshot as the text body of the Statistics message.

use chrono::{DateTime, Utc};

use crate::stats::Stats;

/// Renders `s` as the text body of the Statistics message.
pub fn format_stats(s: &Stats, updated_at: DateTime<Utc>) -> String {
    format!(
        "<b>\u{1F4CA} Statistics</b>\n\n\
         \u{1F7E2} Active users: <b>{}</b>\n\
         \u{1F465} Unique users (all-time): <b>{}</b>\n\n\
         \u{1F4AC} Messages today: <b>{}</b>\n\
         \u{1F4AC} Last 24 hours: <b>{}</b>\n\
         \u{1F4AC} Last 7 days: <b>{}</b>\n\
         \u{1F4AC} Last 30 days: <b>{}</b>\n\
         \u{1F4AC} Last 1 year: <b>{}</b>\n\n\
         \u{1F4C8} Lifetime total: <b>{}</b>\n\n\
         <i>Updated {} UTC</i>",
        s.active_users,
        s.unique_users,
        s.messages_today,
        s.messages_24h,
        s.messages_7d,
        s.messages_30d,
        s.messages_1y,
        s.messages_lifetime,
        updated_at.format("%Y-%m-%d %H:%M:%S"),
    )
}
