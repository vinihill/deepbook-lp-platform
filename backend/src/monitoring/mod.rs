//! System monitoring and metrics

use prometheus::{Counter, Gauge, Histogram, Registry};
use std::sync::Arc;

/// Metrics collector
pub struct Metrics {
    /// Prometheus registry
    registry: Arc<Registry>,
    
    /// Total transactions
    pub transactions_total: Counter,
    
    /// Active users
    pub active_users: Gauge,
    
    /// Gas efficiency
    pub gas_efficiency: Gauge,
    
    /// Transaction latency
    pub transaction_latency: Histogram,
}

impl Metrics {
    /// Create a new metrics collector
    pub fn new() -> Self {
        let registry = Arc::new(Registry::new());
        
        let transactions_total = Counter::new("transactions_total", "Total number of transactions")
            .expect("Failed to create counter");
        
        let active_users = Gauge::new("active_users", "Number of active users")
            .expect("Failed to create gauge");
        
        let gas_efficiency = Gauge::new("gas_efficiency", "Gas efficiency ratio")
            .expect("Failed to create gauge");
        
        let transaction_latency = Histogram::new("transaction_latency", "Transaction latency in seconds")
            .expect("Failed to create histogram");
        
        registry.register(Box::new(transactions_total.clone())).unwrap();
        registry.register(Box::new(active_users.clone())).unwrap();
        registry.register(Box::new(gas_efficiency.clone())).unwrap();
        registry.register(Box::new(transaction_latency.clone())).unwrap();
        
        Self {
            registry,
            transactions_total,
            active_users,
            gas_efficiency,
            transaction_latency,
        }
    }
    
    /// Get the Prometheus registry
    pub fn registry(&self) -> Arc<Registry> {
        self.registry.clone()
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

