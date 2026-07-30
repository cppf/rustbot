//! Constructs the teloxide `Bot` instance.

use teloxide::Bot;

/// Creates a teloxide `Bot` configured with the given token.
///
/// The webhook listener itself is constructed and started separately
/// (see `main.rs`), mirroring the Go version's separate `newBot` +
/// `bot.Start()` and the Python port's separate `Application` build +
/// `run_webhook` call.
pub fn new_bot(token: &str) -> Bot {
    Bot::new(token)
}
