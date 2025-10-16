//! Home page component

use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="page home-page">
            <section class="hero">
                <div class="hero-content">
                    <h1 class="hero-title">
                        "Manus AI Liquidity"
                        <br/>
                        <span class="gradient-text">"Autonomy Platform"</span>
                    </h1>
                    <p class="hero-subtitle">
                        "The world's first AI-native, formally verified, quantum-resistant liquidity provisioning platform on Sui"
                    </p>
                    <div class="hero-cta">
                        <button class="btn btn-primary">"Launch Platform"</button>
                        <button class="btn btn-secondary">"View Documentation"</button>
                    </div>
                </div>
            </section>

            <section class="features">
                <div class="container">
                    <h2 class="section-title">"Core Features"</h2>
                    <div class="feature-grid">
                        <FeatureCard
                            icon="🤖"
                            title="AI Autonomy"
                            description="MCP-style agents with verified invariants for autonomous capital management"
                        />
                        <FeatureCard
                            icon="🔐"
                            title="Quantum-Resistant"
                            description="Post-quantum cryptography (Dilithium + Kyber) from day one"
                        />
                        <FeatureCard
                            icon="✅"
                            title="Formally Verified"
                            description="Lean 4 + Move Prover mathematical proofs of correctness"
                        />
                        <FeatureCard
                            icon="⚡"
                            title="Ultra-Low Gas"
                            description="€0.0017/month via ZK compression (99% reduction)"
                        />
                        <FeatureCard
                            icon="🛡️"
                            title="Hardware Security"
                            description="Intel TDX / AMD SEV-SNP for tamper-proof execution"
                        />
                        <FeatureCard
                            icon="🚀"
                            title="Sui-Native"
                            description="Leveraging parallel execution and object-centric model"
                        />
                    </div>
                </div>
            </section>

            <section class="metrics">
                <div class="container">
                    <h2 class="section-title">"Platform Metrics"</h2>
                    <div class="metrics-grid">
                        <MetricCard value="67%" label="Gas Efficiency"/>
                        <MetricCard value="95/100" label="Performance Score"/>
                        <MetricCard value="99.99%" label="Uptime SLA"/>
                        <MetricCard value="500k+" label="TPS Capacity"/>
                    </div>
                </div>
            </section>
        </div>
    }
}

#[component]
fn FeatureCard(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="feature-card">
            <div class="feature-icon">{icon}</div>
            <h3 class="feature-title">{title}</h3>
            <p class="feature-description">{description}</p>
        </div>
    }
}

#[component]
fn MetricCard(
    value: &'static str,
    label: &'static str,
) -> impl IntoView {
    view! {
        <div class="metric-card">
            <div class="metric-value">{value}</div>
            <div class="metric-label">{label}</div>
        </div>
    }
}

