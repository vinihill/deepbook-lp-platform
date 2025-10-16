//! Funding page component

use leptos::*;

#[component]
pub fn Funding() -> impl IntoView {
    view! {
        <div class="page funding-page">
            <section class="container">
                <h1 class="page-title">"Grant Funding Strategy"</h1>
                <p class="page-subtitle">
                    "Phased approach to building the most advanced LP platform on Sui"
                </p>

                <div class="funding-phases">
                    <PhaseCard
                        phase="Phase 1"
                        amount="$250K"
                        duration="3 months"
                        milestones=vec![
                            "TDX-enabled smart contracts",
                            "Dilithium signature integration",
                            "Lean 4 formal verification",
                            "Core vault implementation",
                        ]
                    />
                    <PhaseCard
                        phase="Phase 2"
                        amount="$500K"
                        duration="4 months"
                        milestones=vec![
                            "MCP agents with verified invariants",
                            "Live strategy engine",
                            "DeepBook integration",
                            "Frontend dashboard",
                        ]
                    />
                    <PhaseCard
                        phase="Phase 3"
                        amount="$500K"
                        duration="3 months"
                        milestones=vec![
                            "SP1 ZK proof integration",
                            "State compression (99% reduction)",
                            "Mainnet launch",
                            "Security audit",
                        ]
                    />
                </div>

                <section class="team">
                    <h2 class="section-title">"Team Structure"</h2>
                    <div class="team-grid">
                        <TeamMember role="Move Engineer" focus="Formal verification specialist"/>
                        <TeamMember role="Rust/ZK Engineer" focus="Backend + cryptography"/>
                        <TeamMember role="AI/ML Researcher" focus="Agent optimization"/>
                        <TeamMember role="Security Architect" focus="TDX/PQC implementation"/>
                        <TeamMember role="Frontend Engineer" focus="Rust WASM specialist"/>
                    </div>
                </section>

                <section class="allocation">
                    <h2 class="section-title">"Fund Allocation"</h2>
                    <div class="allocation-grid">
                        <AllocationItem category="Development" percentage="60%"/>
                        <AllocationItem category="Security Audits" percentage="20%"/>
                        <AllocationItem category="Infrastructure" percentage="10%"/>
                        <AllocationItem category="Operations" percentage="10%"/>
                    </div>
                </section>
            </section>
        </div>
    }
}

#[component]
fn PhaseCard(
    phase: &'static str,
    amount: &'static str,
    duration: &'static str,
    milestones: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <div class="phase-card">
            <div class="phase-header">
                <h3 class="phase-name">{phase}</h3>
                <div class="phase-amount">{amount}</div>
            </div>
            <div class="phase-duration">{duration}</div>
            <ul class="phase-milestones">
                {milestones.into_iter().map(|milestone| {
                    view! { <li>{milestone}</li> }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

#[component]
fn TeamMember(
    role: &'static str,
    focus: &'static str,
) -> impl IntoView {
    view! {
        <div class="team-member">
            <div class="member-role">{role}</div>
            <div class="member-focus">{focus}</div>
        </div>
    }
}

#[component]
fn AllocationItem(
    category: &'static str,
    percentage: &'static str,
) -> impl IntoView {
    view! {
        <div class="allocation-item">
            <span class="allocation-category">{category}</span>
            <span class="allocation-percentage">{percentage}</span>
        </div>
    }
}

