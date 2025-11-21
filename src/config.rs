use anyhow::Result;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub http_port: u16,
}

impl Config {
    /// Load configuration from environment variables.
    /// - `HTTP_PORT` (optional, defaults to 8080)
    pub fn from_env() -> Result<Self> {
        let http_port = env::var("HTTP_PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(8080);

        Ok(Self {
            http_port,
        })
    }
}
