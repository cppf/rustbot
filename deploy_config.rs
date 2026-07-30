//! =====================================================================
//! DEPLOYMENT CONFIGURATION — EDIT THIS FILE BEFORE DEPLOYING
//! =====================================================================
//!
//! This is the ONLY file you need to touch before deploying. Every other
//! file in this repository is ready to go as-is.
//!
//! Fill in `BOT_TOKEN` below with a real token from @BotFather on
//! Telegram (search for "BotFather", send it `/newbot`, follow the
//! prompts, and it will give you a token that looks like
//! `123456789:AAExampleTokenTextGoesHere`).
//!
//! `RAILWAY_PUBLIC_DOMAIN` and `PORT` are normally provided automatically
//! by Railway at deploy time — you only need to fill those in here if
//! you are running this bot somewhere other than Railway (see the
//! comments below).
//!
//! HOW THIS FILE IS USED: `bot_core::config::load_config()` checks
//! environment variables FIRST (BOT_TOKEN, PORT, RAILWAY_PUBLIC_DOMAIN)
//! and only falls back to the constants below if the matching
//! environment variable is unset. On Railway, set `BOT_TOKEN` as an
//! environment variable in the service settings instead of editing this
//! file, if you'd rather not commit your token to source control — both
//! approaches work; this file exists for people who want a true
//! single-file, no-environment-variables-to-configure deployment.
//!
//! SECURITY NOTE: if you fill in a real token below and commit this file
//! to a public repository, your bot token will be public. Anyone with
//! your bot token can control your bot. Prefer the Railway environment
//! variable (`BOT_TOKEN`) over editing this file if your repository is
//! or might become public. Never commit a real token to a public repo.

/// Your bot token from @BotFather. Replace the placeholder below.
///
/// Example of what a real token looks like (this is not a real token):
/// `123456789:AAHk3fL9dQZ7xY2wRt8pV1sN4mK6jC0bXeE`
pub const BOT_TOKEN: &str = "REPLACE_WITH_YOUR_BOT_TOKEN_FROM_BOTFATHER";

/// The port the webhook HTTP server listens on. Railway provides this
/// automatically via the `PORT` environment variable — you normally do
/// not need to change this constant. It's only used as a fallback if
/// `PORT` is not set in the environment (e.g. running outside Railway
/// without setting it yourself).
pub const PORT: u16 = 8080;

/// The public domain Telegram will send webhook updates to (no scheme,
/// e.g. `my-bot.up.railway.app`, not `https://my-bot.up.railway.app`).
/// Railway provides this automatically via the `RAILWAY_PUBLIC_DOMAIN`
/// environment variable — leave this as the placeholder if deploying on
/// Railway. Only fill this in if deploying elsewhere (a VPS, another
/// PaaS, etc.) without that environment variable set.
pub const RAILWAY_PUBLIC_DOMAIN: &str = "REPLACE_IF_NOT_USING_RAILWAY";
