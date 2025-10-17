//! Integration tests for AI agents
//!
//! Tests the behavior and invariants of all AI agents

use manus_backend::agents::{Agent, AgentState, AgentAction};
use manus_backend::agents::ml_agent::{
    RebalancerAgent, StrategyOptimizerAgent, RiskManagerAgent, MarketAnalyzerAgent, MarketData
};
use manus_backend::error::Result;

#[tokio::test]
async fn test_rebalancer_agent_invariants() -> Result<()> {
    let mut agent = RebalancerAgent::new("rebalancer_001".to_string(), 1_000_000);
    
    // Verify initial state
    assert_eq!(agent.state().capital, 1_000_000);
    assert_eq!(agent.state().initial_capital, 1_000_000);
    
    // Test decision making
    let action = agent.decide()?;
    
    // Verify invariants hold
    agent.verify_invariants()?;
    
    Ok(())
}

#[tokio::test]
async fn test_rebalancer_agent_market_analysis() -> Result<()> {
    let agent = RebalancerAgent::new("rebalancer_002".to_string(), 1_000_000);
    
    // Test with bullish market data
    let bullish_data = MarketData {
        prices: vec![1.0, 1.05, 1.10, 1.15, 1.20],
        volumes: vec![1000.0, 1200.0, 1400.0, 1600.0, 1800.0],
        volatility: 0.10,
        liquidity: 10000.0,
    };
    
    let decision = agent.analyze(&bullish_data)?;
    assert!(decision.confidence > 0.0);
    assert!(decision.confidence <= 1.0);
    assert!(decision.predicted_return > 0.0); // Should predict positive return
    
    // Test with bearish market data
    let bearish_data = MarketData {
        prices: vec![1.0, 0.95, 0.90, 0.85, 0.80],
        volumes: vec![1000.0, 800.0, 600.0, 400.0, 200.0],
        volatility: 0.25,
        liquidity: 2000.0,
    };
    
    let decision = agent.analyze(&bearish_data)?;
    assert!(decision.confidence > 0.0);
    assert!(decision.confidence <= 1.0);
    assert!(decision.predicted_return < 0.0); // Should predict negative return
    
    Ok(())
}

#[tokio::test]
async fn test_risk_manager_agent() -> Result<()> {
    let mut agent = RiskManagerAgent::new("risk_001".to_string(), 1_000_000, 0.2);
    
    // Should not trigger emergency withdraw initially
    let action = agent.decide()?;
    match action {
        AgentAction::Hold => {}, // Expected
        AgentAction::EmergencyWithdraw => panic!("Should not trigger emergency withdraw"),
        _ => panic!("Unexpected action"),
    }
    
    // Verify risk limits
    assert!(agent.check_risk_limits()?);
    
    Ok(())
}

#[tokio::test]
async fn test_market_analyzer_agent() -> Result<()> {
    let agent = MarketAnalyzerAgent::new("analyzer_001".to_string(), 1_000_000);
    
    // Test volatile market detection
    let volatile_data = MarketData {
        prices: vec![1.0, 1.5, 0.8, 1.3, 0.9],
        volumes: vec![1000.0, 1200.0, 1100.0, 1300.0, 1400.0],
        volatility: 0.4,
        liquidity: 5000.0,
    };
    
    let regime = agent.analyze_regime(&volatile_data);
    assert_eq!(regime, "volatile");
    
    // Test ranging market detection
    let ranging_data = MarketData {
        prices: vec![1.0, 1.01, 1.0, 1.01, 1.0],
        volumes: vec![1000.0, 1000.0, 1000.0, 1000.0, 1000.0],
        volatility: 0.05,
        liquidity: 5000.0,
    };
    
    let regime = agent.analyze_regime(&ranging_data);
    assert_eq!(regime, "ranging");
    
    // Test trending market detection
    let trending_data = MarketData {
        prices: vec![1.0, 1.1, 1.2, 1.3, 1.4],
        volumes: vec![1000.0, 1100.0, 1200.0, 1300.0, 1400.0],
        volatility: 0.15,
        liquidity: 5000.0,
    };
    
    let regime = agent.analyze_regime(&trending_data);
    assert_eq!(regime, "trending");
    
    Ok(())
}

#[tokio::test]
async fn test_agent_capital_preservation_invariant() -> Result<()> {
    let mut agent = RebalancerAgent::new("rebalancer_003".to_string(), 1_000_000);
    
    // Simulate multiple decisions
    for _ in 0..10 {
        let action = agent.decide()?;
        agent.execute(action)?;
        
        // Invariant: capital should never decrease below initial
        agent.verify_invariants()?;
    }
    
    Ok(())
}

#[tokio::test]
async fn test_strategy_optimizer_agent() -> Result<()> {
    let agent = StrategyOptimizerAgent::new("optimizer_001".to_string(), 1_000_000);
    
    // Test parameter optimization
    let historical_data = vec![
        MarketData {
            prices: vec![1.0, 1.05, 1.03],
            volumes: vec![1000.0, 1200.0, 1100.0],
            volatility: 0.15,
            liquidity: 5000.0,
        },
        MarketData {
            prices: vec![1.03, 1.07, 1.10],
            volumes: vec![1100.0, 1300.0, 1400.0],
            volatility: 0.18,
            liquidity: 5500.0,
        },
    ];
    
    let optimized_params = agent.optimize_parameters(&historical_data)?;
    assert_eq!(optimized_params.len(), 3);
    
    Ok(())
}

#[tokio::test]
async fn test_agent_action_execution() -> Result<()> {
    let mut agent = RebalancerAgent::new("rebalancer_004".to_string(), 1_000_000);
    
    // Test rebalance action
    let rebalance_action = AgentAction::Rebalance {
        targets: vec![
            ("SUI".to_string(), 0.6),
            ("USDC".to_string(), 0.4),
        ],
    };
    agent.execute(rebalance_action)?;
    
    // Test risk adjustment action
    let adjust_risk_action = AgentAction::AdjustRisk {
        new_tolerance: 0.3,
    };
    agent.execute(adjust_risk_action)?;
    assert_eq!(agent.state().risk_tolerance, 0.3);
    
    // Test hold action
    let hold_action = AgentAction::Hold;
    agent.execute(hold_action)?;
    
    Ok(())
}

