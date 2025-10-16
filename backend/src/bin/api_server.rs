//! Manus AI API Server

use manus_liquidity_backend::{api, config::Config, init};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize backend
    init().await?;
    
    // Load configuration
    let config = Config::load().unwrap_or_default();
    
    info!("Starting Manus AI API Server");
    info!("Sui Network: {}", config.sui.network_url);
    info!("PQC Enabled: {}", config.security.pqc_enabled);
    info!("ZK Proofs Enabled: {}", config.security.zk_proofs_enabled);
    
    // Create router
    let app = api::create_router();
    
    // Bind address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    info!("Listening on {}", addr);
    
    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

