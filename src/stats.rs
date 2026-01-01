use serde::Serialize;

/// Statistics tracking for DNS queries and blocks
#[derive(Debug, Clone, Serialize)]
pub struct Stats {
    /// Total DNS queries received
    pub total_queries: u64,
    /// Number of queries that were blocked
    pub blocked_queries: u64,
    /// Number of queries that were allowed
    pub allowed_queries: u64,
    /// Top blocked domains (domain -> count)
    pub top_blocked_domains: std::collections::HashMap<String, u64>,
}

impl Stats {
    /// Create new empty statistics
    pub fn new() -> Self {
        Self {
            total_queries: 0,
            blocked_queries: 0,
            allowed_queries: 0,
            top_blocked_domains: std::collections::HashMap::new(),
        }
    }

    /// Record a blocked query
    pub fn record_blocked(&mut self, domain: &str) {
        self.total_queries += 1;
        self.blocked_queries += 1;

        let domain = domain.to_lowercase();
        *self.top_blocked_domains.entry(domain).or_insert(0) += 1;
    }

    /// Record an allowed query
    pub fn record_allowed(&mut self) {
        self.total_queries += 1;
        self.allowed_queries += 1;
    }

    /// Get block percentage
    pub fn block_percentage(&self) -> f64 {
        if self.total_queries == 0 {
            0.0
        } else {
            (self.blocked_queries as f64 / self.total_queries as f64) * 100.0
        }
    }
}
