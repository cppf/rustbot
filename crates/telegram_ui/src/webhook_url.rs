//! Builds the public webhook URL Telegram uses to deliver updates.

/// Builds the public URL Telegram will use to deliver updates, combining
/// Railway's public domain with the bot token as the path secret.
pub fn webhook_url(domain: &str, token: &str) -> String {
    format!("https://{domain}/{token}")
}
