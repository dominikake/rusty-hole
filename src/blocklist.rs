use hashbrown::HashSet;
use regex::Regex;

/// Blocklist manager for efficient domain lookups
#[derive(Debug, Clone)]
pub struct Blocklist {
    /// Set of blocked domains for O(1) lookups
    blocked_domains: HashSet<String>,
    /// Regex patterns for wildcard matching
    blocked_patterns: Vec<Regex>,
    /// Whitelisted domains that override blocks
    whitelist: HashSet<String>,
}

impl Blocklist {
    /// Create a new blocklist by fetching from URLs
    pub async fn new(urls: &[String]) -> anyhow::Result<Self> {
        let mut blocked_domains = HashSet::new();
        let blocked_patterns = Vec::new();
        let whitelist = HashSet::new();

        for url in urls {
            Self::fetch_and_parse_blocklist(url, &mut blocked_domains).await?;
        }

        Ok(Self {
            blocked_domains,
            blocked_patterns,
            whitelist,
        })
    }

    /// Fetch and parse a blocklist from a URL
    async fn fetch_and_parse_blocklist(
        url: &str,
        domains: &mut HashSet<String>,
    ) -> anyhow::Result<()> {
        let response = reqwest::get(url).await?;
        let content = response.text().await?;

        for line in content.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse hosts file format: "127.0.0.1 domain.com"
            if let Some(domain) = Self::parse_hosts_line(line) {
                domains.insert(domain.to_lowercase());
            }
        }

        tracing::info!("Loaded {} blocked domains from {}", domains.len(), url);
        Ok(())
    }

    /// Parse a single line from hosts file format
    fn parse_hosts_line(line: &str) -> Option<&str> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            // Skip localhost entries
            let domain = parts[1];
            if domain != "localhost" && !domain.starts_with("localhost.") {
                Some(domain)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Check if a domain should be blocked
    pub fn is_blocked(&self, domain: &str) -> bool {
        let domain = domain.to_lowercase();

        // Check whitelist first (overrides blocklist)
        if self.whitelist.contains(&domain) {
            return false;
        }

        // Check exact domain match
        if self.blocked_domains.contains(&domain) {
            return true;
        }

        // Check regex patterns
        for pattern in &self.blocked_patterns {
            if pattern.is_match(&domain) {
                return true;
            }
        }

        false
    }

    /// Add a domain to the whitelist
    pub fn add_to_whitelist(&mut self, domain: String) {
        self.whitelist.insert(domain.to_lowercase());
    }

    /// Remove a domain from the whitelist
    pub fn remove_from_whitelist(&mut self, domain: &str) {
        self.whitelist.remove(&domain.to_lowercase());
    }
}