//! Manus AI Agent Runner

use manus_liquidity_backend::{agents::*, config::Config, init};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize backend
    init().await?;
    
    // Load configuration
    let config = Config::load().unwrap_or_default();
    
    if !config.agents.enabled {
        info!("Agents are disabled in configuration");
        return Ok(());
    }
    
    info!("Starting Manus AI Agent Runner");
    info!("Rebalance interval: {}s", config.agents.rebalance_interval);
    info!("Risk tolerance: {}", config.agents.risk_tolerance);
    
    // Create agent
    let mut agent = AutonomousAgent::new(
        "main-agent".to_string(),
        1_000_000, // Initial capital
        config.agents.risk_tolerance,
    );
    
    info!("Agent initialized: {}", agent.id());
    
    // Main agent loop
    loop {
        // Decide action
        match agent.decide() {
            Ok(action) => {
                info!("Agent decision: {:?}", action);
                
                // Execute action
                if let Err(e) = agent.execute(action) {
                    tracing::error!("Failed to execute action: {}", e);
                }
            }
            Err(e) => {
                tracing::error!("Failed to decide action: {}", e);
            }
        }
        
        // Sleep until next rebalance
        tokio::time::sleep(tokio::time::Duration::from_secs(config.agents.rebalance_interval)).await;
    }
}

