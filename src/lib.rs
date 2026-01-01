pub mod config;
pub mod dns;
pub mod blocklist;
pub mod stats;
pub mod dashboard;

use std::sync::Arc;
use tokio::sync::Mutex;

/// Core RustyHole application state
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<config::Config>,
    pub blocklist: Arc<blocklist::Blocklist>,
    pub stats: Arc<Mutex<stats::Stats>>,
}

/// Main application entry point
pub async fn run() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = config::load_config("config.toml")?;

    // Initialize blocklist
    let blocklist = blocklist::Blocklist::new(&config.blocklist_urls).await?;

    // Initialize stats
    let stats = stats::Stats::new();

    // Create shared application state
    let state = AppState {
        config: Arc::new(config),
        blocklist: Arc::new(blocklist),
        stats: Arc::new(Mutex::new(stats)),
    };

    // Start DNS server
    let dns_state = state.clone();
    let dns_handle = tokio::spawn(async move {
        dns::run_dns_server(dns_state).await
    });

    // Start web dashboard
    let web_state = state.clone();
    let web_handle = tokio::spawn(async move {
        dashboard::run_dashboard(web_state).await
    });

    // Wait for both services
    tokio::try_join!(dns_handle, web_handle)?;

    Ok(())
}