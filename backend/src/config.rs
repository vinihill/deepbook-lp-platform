//! Configuration management for Manus AI backend

use serde::{Deserialize, Serialize};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Server configuration
    pub server: ServerConfig,
    
    /// Sui network configuration
    pub sui: SuiConfig,
    
    /// Database configuration
    pub database: DatabaseConfig,
    
    /// AI agent configuration
    pub agents: AgentConfig,
    
    /// Security configuration
    pub security: SecurityConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Host address
    pub host: String,
    
    /// Port number
    pub port: u16,
    
    /// Enable TLS
    pub tls_enabled: bool,
}

/// Sui network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiConfig {
    /// Network URL (testnet, devnet, mainnet)
    pub network_url: String,
    
    /// Wallet address
    pub wallet_address: Option<String>,
    
    /// Package ID for deployed contracts
    pub package_id: Option<String>,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// PostgreSQL connection URL
    pub url: String,
    
    /// Maximum connections
    pub max_connections: u32,
}

/// AI agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Enable autonomous agents
    pub enabled: bool,
    
    /// Rebalancing interval (seconds)
    pub rebalance_interval: u64,
    
    /// Risk tolerance (0.0 - 1.0)
    pub risk_tolerance: f64,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable post-quantum cryptography
    pub pqc_enabled: bool,
    
    /// Enable hardware enclaves (TDX/SEV)
    pub hardware_enclaves_enabled: bool,
    
    /// Enable ZK proofs
    pub zk_proofs_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                tls_enabled: true,
            },
            sui: SuiConfig {
                network_url: "https://fullnode.testnet.sui.io:443".to_string(),
                wallet_address: None,
                package_id: None,
            },
            database: DatabaseConfig {
                url: "postgres://localhost/manus_liquidity".to_string(),
                max_connections: 10,
            },
            agents: AgentConfig {
                enabled: true,
                rebalance_interval: 300, // 5 minutes
                risk_tolerance: 0.5,
            },
            security: SecurityConfig {
                pqc_enabled: true,
                hardware_enclaves_enabled: false, // Requires special hardware
                zk_proofs_enabled: true,
            },
        }
    }
}

impl Config {
    /// Load configuration from environment and config file
    pub fn load() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();
        
        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("MANUS"))
            .build()?;
        
        Ok(config.try_deserialize()?)
    }
}

