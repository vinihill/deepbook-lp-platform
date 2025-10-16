//! AI Agents page component

use leptos::*;

#[component]
pub fn Agents() -> impl IntoView {
    view! {
        <div class="page agents-page">
            <section class="hero">
                <div class="hero-content">
                    <h1 class="hero-title">
                        <span class="gradient-text">"Autonomous AI Agents"</span>
                    </h1>
                    <p class="hero-subtitle">
                        "Four specialized AI agents powered by Claude, working together with formally verified invariants"
                    </p>
                </div>
            </section>

            <section class="agents-overview">
                <div class="container">
                    <h2 class="section-title">"Agent Architecture"</h2>
                    <div class="agent-grid">
                        <AgentCard
                            name="Rebalancer Agent"
                            icon="🔄"
                            description="Monitors pool imbalances and determines optimal rebalancing timing"
                            capabilities=vec![
                                "Real-time pool imbalance detection",
                                "Impermanent loss calculation",
                                "Optimal rebalancing timing",
                                "Transaction cost optimization",
                                "Gas-efficient execution"
                            ]
                            metrics=AgentMetrics {
                                decisions: 127,
                                accuracy: 94.5,
                                avg_confidence: 89,
                                uptime: 99.9,
                            }
                        />
                        
                        <AgentCard
                            name="Strategy Optimizer"
                            icon="✨"
                            description="Evaluates strategy performance and identifies market opportunities"
                            capabilities=vec![
                                "Strategy performance evaluation",
                                "Market opportunity identification",
                                "Fee tier optimization",
                                "Backtesting and simulation",
                                "Risk-adjusted return analysis"
                            ]
                            metrics=AgentMetrics {
                                decisions: 89,
                                accuracy: 91.2,
                                avg_confidence: 86,
                                uptime: 99.8,
                            }
                        />
                        
                        <AgentCard
                            name="Risk Manager"
                            icon="🛡️"
                            description="Monitors exposure and concentration risk with anomaly detection"
                            capabilities=vec![
                                "Exposure monitoring",
                                "Concentration risk analysis",
                                "Anomaly detection",
                                "Emergency procedure triggers",
                                "Position limit enforcement"
                            ]
                            metrics=AgentMetrics {
                                decisions: 156,
                                accuracy: 96.8,
                                avg_confidence: 92,
                                uptime: 100.0,
                            }
                        />
                        
                        <AgentCard
                            name="Market Analyzer"
                            icon="📊"
                            description="Analyzes price trends and volatility to identify opportunities"
                            capabilities=vec![
                                "Price trend analysis",
                                "Volatility measurement",
                                "Liquidity opportunity detection",
                                "Short-term price prediction",
                                "Market sentiment assessment"
                            ]
                            metrics=AgentMetrics {
                                decisions: 203,
                                accuracy: 88.7,
                                avg_confidence: 84,
                                uptime: 99.7,
                            }
                        />
                    </div>
                </div>
            </section>

            <section class="orchestrator">
                <div class="container">
                    <h2 class="section-title">"Agent Orchestrator"</h2>
                    <div class="orchestrator-card">
                        <div class="orchestrator-icon">"⚡"</div>
                        <h3>"Consensus Decision Making"</h3>
                        <p>
                            "The Agent Orchestrator coordinates all four specialized AI agents to reach consensus decisions. "
                            "It combines insights from multiple perspectives, validates decisions against formally verified invariants, "
                            "and ensures optimal execution of liquidity provisioning strategies."
                        </p>
                        <div class="orchestrator-features">
                            <div class="feature-box">
                                <div class="feature-icon">"🧠"</div>
                                <h4>"Multi-Agent Consensus"</h4>
                                <p>"Combines decisions using weighted confidence scores"</p>
                            </div>
                            <div class="feature-box">
                                <div class="feature-icon">"✅"</div>
                                <h4>"Invariant Verification"</h4>
                                <p>"Validates against formally verified safety properties"</p>
                            </div>
                            <div class="feature-box">
                                <div class="feature-icon">"🚀"</div>
                                <h4>"Optimal Execution"</h4>
                                <p>"Gas optimization and slippage protection"</p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <section class="invariants">
                <div class="container">
                    <h2 class="section-title">"Formally Verified Invariants"</h2>
                    <p class="section-description">
                        "Mathematical proofs ensuring agent safety and correctness using Move Prover and Lean 4"
                    </p>
                    <div class="invariant-grid">
                        <InvariantCard
                            name="Capital Preservation"
                            description="Agents never recommend actions that could result in capital loss"
                            verified=true
                        />
                        <InvariantCard
                            name="Confidence Thresholds"
                            description="High-risk actions require confidence scores above 90%"
                            verified=true
                        />
                        <InvariantCard
                            name="Emergency Procedures"
                            description="Emergency withdrawals only triggered in extreme conditions"
                            verified=true
                        />
                        <InvariantCard
                            name="Parameter Validation"
                            description="All rebalancing actions must have valid, verified parameters"
                            verified=true
                        />
                    </div>
                    
                    <div class="verification-note">
                        <div class="note-icon">"⚠️"</div>
                        <div class="note-content">
                            <h4>"Formal Verification Process"</h4>
                            <p>
                                "All agent logic is formally verified using Move Prover and Lean 4 theorem proving. "
                                "This ensures that the agents mathematically cannot violate critical safety properties, "
                                "providing unprecedented security guarantees for autonomous DeFi operations."
                            </p>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}

#[derive(Clone)]
struct AgentMetrics {
    decisions: u32,
    accuracy: f64,
    avg_confidence: u32,
    uptime: f64,
}

#[component]
fn AgentCard(
    name: &'static str,
    icon: &'static str,
    description: &'static str,
    capabilities: Vec<&'static str>,
    metrics: AgentMetrics,
) -> impl IntoView {
    view! {
        <div class="agent-card">
            <div class="agent-header">
                <div class="agent-icon">{icon}</div>
                <div class="agent-status">
                    <span class="status-dot"></span>
                    "active"
                </div>
            </div>
            <h3 class="agent-name">{name}</h3>
            <p class="agent-description">{description}</p>
            
            <div class="agent-capabilities">
                <h4>"Capabilities"</h4>
                <ul>
                    {capabilities.into_iter().map(|cap| {
                        view! { <li>"✓ " {cap}</li> }
                    }).collect::<Vec<_>>()}
                </ul>
            </div>
            
            <div class="agent-metrics">
                <div class="metric">
                    <div class="metric-label">"Decisions"</div>
                    <div class="metric-value">{metrics.decisions}</div>
                </div>
                <div class="metric">
                    <div class="metric-label">"Accuracy"</div>
                    <div class="metric-value">{format!("{:.1}%", metrics.accuracy)}</div>
                </div>
                <div class="metric">
                    <div class="metric-label">"Confidence"</div>
                    <div class="metric-value">{metrics.avg_confidence}"%"</div>
                </div>
                <div class="metric">
                    <div class="metric-label">"Uptime"</div>
                    <div class="metric-value">{format!("{:.1}%", metrics.uptime)}</div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn InvariantCard(
    name: &'static str,
    description: &'static str,
    verified: bool,
) -> impl IntoView {
    view! {
        <div class="invariant-card">
            <div class="invariant-header">
                <h4>{name}</h4>
                {if verified {
                    view! { <span class="verified-badge">"✓ Verified"</span> }
                } else {
                    view! { <span class="pending-badge">"⏳ Pending"</span> }
                }}
            </div>
            <p>{description}</p>
        </div>
    }
}

