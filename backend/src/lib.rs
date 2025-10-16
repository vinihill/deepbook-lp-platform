//! Manus AI Liquidity Autonomy Platform - Backend Services
//!
//! This crate provides the core backend infrastructure for the Manus AI platform,
//! including API services, AI agents, post-quantum cryptography, and monitoring.

#![warn(missing_docs)]
#![warn(clippy::all)]

/// API server implementation
pub mod api;

/// Autonomous AI agents for capital management
pub mod agents;

/// Post-quantum cryptography primitives
pub mod crypto;

/// Zero-knowledge proof generation and verification
pub mod zkp;

/// WebAssembly plugin system
pub mod wasm;

/// System monitoring and metrics
pub mod monitoring;

/// Configuration management
pub mod config;

/// Error types
pub mod error;

/// Sui blockchain integration
pub mod sui;

use anyhow::Result;

/// Initialize the backend services
pub async fn init() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .init();

    tracing::info!("Manus AI Liquidity Backend initializing...");

    Ok(())
}

