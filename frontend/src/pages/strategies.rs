//! Strategies page component

use leptos::*;

#[component]
pub fn Strategies() -> impl IntoView {
    view! {
        <div class="page strategies-page">
            <section class="container">
                <h1 class="page-title">"AI-Driven Strategies"</h1>
                <p class="page-subtitle">
                    "Autonomous capital management with formally verified invariants"
                </p>

                <div class="strategies-grid">
                    <StrategyCard
                        name="Market Making"
                        description="Provide liquidity on DeepBook with dynamic spread adjustment"
                        performance="Sharpe Ratio: 2.4"
                        risk="Low-Medium"
                    />
                    <StrategyCard
                        name="Yield Optimization"
                        description="Automatically rebalance across highest-yielding pools"
                        performance="APY: 18.5%"
                        risk="Medium"
                    />
                    <StrategyCard
                        name="Impermanent Loss Hedge"
                        description="AI-driven hedging to minimize IL exposure"
                        performance="IL Reduction: 67%"
                        risk="Low"
                    />
                </div>

                <section class="simulation">
                    <h2 class="section-title">"Live Strategy Simulation"</h2>
                    <div class="simulation-container">
                        <div class="simulation-status">
                            <div class="status-indicator active"></div>
                            <span>"AI Optimization Active"</span>
                        </div>
                        <div class="simulation-metrics">
                            <div class="metric">
                                <span class="metric-label">"Current APY"</span>
                                <span class="metric-value">"18.5%"</span>
                            </div>
                            <div class="metric">
                                <span class="metric-label">"Sharpe Ratio"</span>
                                <span class="metric-value">"2.4"</span>
                            </div>
                            <div class="metric">
                                <span class="metric-label">"Max Drawdown"</span>
                                <span class="metric-value">"-3.2%"</span>
                            </div>
                        </div>
                    </div>
                </section>
            </section>
        </div>
    }
}

#[component]
fn StrategyCard(
    name: &'static str,
    description: &'static str,
    performance: &'static str,
    risk: &'static str,
) -> impl IntoView {
    view! {
        <div class="strategy-card">
            <h3 class="strategy-name">{name}</h3>
            <p class="strategy-description">{description}</p>
            <div class="strategy-stats">
                <div class="stat">
                    <span class="stat-label">"Performance: "</span>
                    <span class="stat-value">{performance}</span>
                </div>
                <div class="stat">
                    <span class="stat-label">"Risk: "</span>
                    <span class="stat-value">{risk}</span>
                </div>
            </div>
        </div>
    }
}

