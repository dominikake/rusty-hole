use crate::AppState;
use axum::{
    extract::State,
    response::Json,
    routing::get,
    Router,
};
use serde_json::json;
use std::net::SocketAddr;

/// Run the web dashboard server
pub async fn run_dashboard(state: AppState) -> anyhow::Result<()> {
    let port = state.config.web_port;
    let app = Router::new()
        .route("/stats", get(get_stats))
        .route("/dashboard", get(get_dashboard_html))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Web dashboard listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// Get statistics as JSON
async fn get_stats(State(state): State<AppState>) -> Json<serde_json::Value> {
    let stats = state.stats.lock().await.clone();

    Json(json!({
        "total_queries": stats.total_queries,
        "blocked_queries": stats.blocked_queries,
        "allowed_queries": stats.allowed_queries,
        "block_percentage": format!("{:.1}%", stats.block_percentage()),
        "top_blocked_domains": stats.top_blocked_domains
    }))
}

/// Get HTML dashboard
async fn get_dashboard_html(State(state): State<AppState>) -> axum::response::Html<String> {
    let stats = state.stats.lock().await.clone();

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>RustyHole Dashboard</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .stats {{ display: flex; gap: 20px; margin-bottom: 30px; }}
        .stat-card {{ background: #f5f5f5; padding: 20px; border-radius: 8px; flex: 1; }}
        .stat-number {{ font-size: 2em; font-weight: bold; color: #333; }}
        .stat-label {{ color: #666; }}
        table {{ width: 100%; border-collapse: collapse; }}
        th, td {{ padding: 10px; text-align: left; border-bottom: 1px solid #ddd; }}
        th {{ background-color: #f5f5f5; }}
    </style>
    <meta http-equiv="refresh" content="30">
</head>
<body>
    <h1>RustyHole Network Ad Blocker</h1>

    <div class="stats">
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Total Queries</div>
        </div>
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Blocked Queries</div>
        </div>
        <div class="stat-card">
            <div class="stat-number">{:.1}%</div>
            <div class="stat-label">Block Percentage</div>
        </div>
    </div>

    <h2>Top Blocked Domains</h2>
    <table>
        <tr><th>Domain</th><th>Blocks</th></tr>
        {}
    </table>
</body>
</html>"#,
        stats.total_queries,
        stats.blocked_queries,
        stats.block_percentage(),
        stats.top_blocked_domains
            .iter()
            .take(10)
            .map(|(domain, count)| format!("<tr><td>{}</td><td>{}</td></tr>", domain, count))
            .collect::<Vec<_>>()
            .join("")
    );

    axum::response::Html(html)
}