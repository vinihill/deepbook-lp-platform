//! API client for backend communication

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

const API_BASE_URL: &str = "http://localhost:8080/api/v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultInfo {
    pub id: String,
    pub total_value: u64,
    pub total_shares: u64,
    pub strategy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub total_value_locked: u64,
    pub total_users: u64,
    pub gas_efficiency: f64,
    pub uptime: f64,
}

/// Fetch all vaults
pub async fn fetch_vaults() -> Result<Vec<VaultInfo>, String> {
    let response = Request::get(&format!("{}/vaults", API_BASE_URL))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    response
        .json::<Vec<VaultInfo>>()
        .await
        .map_err(|e| e.to_string())
}

/// Fetch platform metrics
pub async fn fetch_metrics() -> Result<Metrics, String> {
    let response = Request::get(&format!("{}/metrics", API_BASE_URL))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    response
        .json::<Metrics>()
        .await
        .map_err(|e| e.to_string())
}

