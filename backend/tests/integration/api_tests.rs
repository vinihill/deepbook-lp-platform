//! Integration tests for REST API
//!
//! Tests all API endpoints

use axum::http::StatusCode;
use axum_test::TestServer;
use manus_backend::api::routes::create_router;
use manus_backend::config::Config;
use serde_json::json;

async fn setup_test_server() -> TestServer {
    let config = Config::default();
    let app = create_router(config);
    TestServer::new(app).unwrap()
}

#[tokio::test]
async fn test_health_endpoint() {
    let server = setup_test_server().await;
    
    let response = server.get("/health").await;
    
    assert_eq!(response.status_code(), StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert_eq!(body["status"], "healthy");
}

#[tokio::test]
async fn test_get_agents() {
    let server = setup_test_server().await;
    
    let response = server.get("/api/v1/agents").await;
    
    assert_eq!(response.status_code(), StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_get_agent_by_id() {
    let server = setup_test_server().await;
    
    let response = server.get("/api/v1/agents/rebalancer_001").await;
    
    assert_eq!(response.status_code(), StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert_eq!(body["id"], "rebalancer_001");
}

#[tokio::test]
async fn test_create_agent() {
    let server = setup_test_server().await;
    
    let new_agent = json!({
        "id": "test_agent_001",
        "type": "rebalancer",
        "initial_capital": 1000000,
        "risk_tolerance": 0.5
    });
    
    let response = server.post("/api/v1/agents")
        .json(&new_agent)
        .await;
    
    assert_eq!(response.status_code(), StatusCode::CREATED);
    let body: serde_json::Value = response.json();
    assert_eq!(body["id"], "test_agent_001");
}

#[tokio::test]
async fn test_agent_decision() {
    let server = setup_test_server().await;
    
    let response = server.post("/api/v1/agents/rebalancer_001/decide").await;
    
    assert_eq!(response.status_code(), StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert!(body.get("action").is_some());
}

#[tokio::test]
async fn test_get_vault_info() {
    let server = setup_test_server().await;
    
    let response = server.get("/api/v1/vaults/SUI").await;
    
    assert_eq!(response.status_code(), StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert!(body.get("balance").is_some());
}

#[tokio::test]
async fn test_deposit_to_vault() {
    let server = setup_test_server().await;
    
    let deposit_request = json!({
        "amount": 1000000,
        "asset": "SUI"
    });
    
    let response = server.post("/api/v1/vaults/deposit")
        .json(&deposit_request)
        .await;
    
    assert_eq!(response.status_code(), StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert!(body.get("transaction_hash").is_some());
}

#[tokio::test]
async fn test_get_market_data() {
    let server = setup_test_server().await;
    
    let response = server.get("/api/v1/market/SUI-USDC").await;
    
    assert_eq!(response.status_code(), StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert!(body.get("price").is_some());
    assert!(body.get("volume").is_some());
}

#[tokio::test]
async fn test_get_strategies() {
    let server = setup_test_server().await;
    
    let response = server.get("/api/v1/strategies").await;
    
    assert_eq!(response.status_code(), StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_invalid_endpoint() {
    let server = setup_test_server().await;
    
    let response = server.get("/api/v1/nonexistent").await;
    
    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_cors_headers() {
    let server = setup_test_server().await;
    
    let response = server.get("/health").await;
    
    assert!(response.headers().contains_key("access-control-allow-origin"));
}

