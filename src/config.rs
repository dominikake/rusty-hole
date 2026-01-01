use serde::{Deserialize, Serialize};
use std::fs;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Upstream DNS server to forward legitimate queries to
    pub upstream_dns: String,
    /// List of URLs to fetch blocklists from
    pub blocklist_urls: Vec<String>,
    /// List of whitelisted domains (overrides blocklist)
    pub whitelist: Vec<String>,
    /// Port for the web dashboard (default: 8080)
    pub web_port: u16,
    /// DNS server bind address (default: "0.0.0.0:53")
    pub dns_bind: String,
}

/// Load configuration from TOML file
pub fn load_config(path: &str) -> anyhow::Result<Config> {
    let contents = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

impl Default for Config {
    fn default() -> Self {
        Self {
            upstream_dns: "8.8.8.8:53".to_string(),
            blocklist_urls: vec![
                "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts".to_string(),
            ],
            whitelist: vec![],
            web_port: 8080,
            dns_bind: "0.0.0.0:53".to_string(),
        }
    }
}
