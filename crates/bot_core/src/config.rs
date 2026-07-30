//! Loads the settings needed to start the bot from environment variables.

use std::env;
use std::fmt;

/// Holds the settings needed to start the bot.
#[derive(Debug, Clone)]
pub struct Config {
    pub token: String,
    pub port: u16,
    pub domain: String,
}

/// Error returned when required configuration is missing or invalid.
#[derive(Debug)]
pub struct ConfigError(String);

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ConfigError {}

/// Reads configuration from environment variables.
///
/// `BOT_TOKEN` is required; `PORT` defaults to `8080` if unset.
pub fn load_config() -> Result<Config, ConfigError> {
    load_config_with_fallback(None, None, None)
}

/// Reads configuration from environment variables, falling back to the
/// given values for any variable that is unset or empty in the
/// environment. Used by `main.rs` together with `deploy_config.rs` so
/// the repository can be deployed with nothing to configure beyond
/// editing that one file — environment variables, where present, always
/// take priority over the fallback values.
pub fn load_config_with_fallback(
    fallback_token: Option<&str>,
    fallback_port: Option<u16>,
    fallback_domain: Option<&str>,
) -> Result<Config, ConfigError> {
    let token = match env::var("BOT_TOKEN") {
        Ok(t) if !t.is_empty() => t,
        _ => fallback_token.unwrap_or("").to_string(),
    };
    if token.is_empty() || token == "REPLACE_WITH_YOUR_BOT_TOKEN_FROM_BOTFATHER" {
        return Err(ConfigError(
            "No BOT_TOKEN found. Set the BOT_TOKEN environment variable, or edit \
             deploy_config.rs and replace the placeholder token with a real one \
             from @BotFather."
                .to_string(),
        ));
    }

    let port: u16 = match env::var("PORT") {
        Ok(p) if !p.is_empty() => p
            .parse()
            .map_err(|_| ConfigError(format!("PORT is not a valid port number: {p}")))?,
        _ => fallback_port.unwrap_or(8080),
    };

    let domain = match env::var("RAILWAY_PUBLIC_DOMAIN") {
        Ok(d) if !d.is_empty() => d,
        _ => {
            let fallback = fallback_domain.unwrap_or("");
            if fallback == "REPLACE_IF_NOT_USING_RAILWAY" {
                String::new()
            } else {
                fallback.to_string()
            }
        }
    };

    Ok(Config {
        token,
        port,
        domain,
    })
}
