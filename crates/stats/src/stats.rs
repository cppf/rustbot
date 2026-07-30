//! `Stats` holds a snapshot of bot usage counts, as shown on the
//! Statistics page.

/// A snapshot of bot usage counts, as shown on the Statistics page.
#[derive(Debug, Clone, Default)]
pub struct Stats {
    /// Distinct users seen in the last 5 minutes.
    pub active_users: i64,
    /// Distinct users, all-time.
    pub unique_users: i64,
    /// Messages since local midnight UTC.
    pub messages_today: i64,
    /// Messages in the trailing 24 hours.
    pub messages_24h: i64,
    /// Messages in the trailing 7 days.
    pub messages_7d: i64,
    /// Messages in the trailing 30 days.
    pub messages_30d: i64,
    /// Messages in the trailing 1 year.
    pub messages_1y: i64,
    /// All messages ever recorded.
    pub messages_lifetime: i64,
}
