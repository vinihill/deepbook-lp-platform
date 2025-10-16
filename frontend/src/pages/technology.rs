//! Technology page component

use leptos::*;

#[component]
pub fn Technology() -> impl IntoView {
    view! {
        <div class="page technology-page">
            <section class="container">
                <h1 class="page-title">"Technology Stack"</h1>
                <p class="page-subtitle">
                    "Built on cutting-edge technologies for maximum security, performance, and reliability"
                </p>

                <div class="tech-stack">
                    <TechCard
                        title="Sui Network"
                        description="Leveraging parallel execution and object-centric model for unprecedented throughput"
                        features=vec!["500k+ TPS", "Sub-second finality", "Native parallelism"]
                    />
                    <TechCard
                        title="Move Language"
                        description="Resource-oriented programming for secure asset management"
                        features=vec!["Memory safety", "Resource safety", "Formal verification"]
                    />
                    <TechCard
                        title="Rust + WASM"
                        description="High-performance backend and frontend with zero-cost abstractions"
                        features=vec!["Memory safe", "Fast execution", "Small binary size"]
                    />
                    <TechCard
                        title="Post-Quantum Crypto"
                        description="Quantum-resistant from day one with Dilithium and Kyber"
                        features=vec!["NIST-approved", "Future-proof", "Hardware accelerated"]
                    />
                    <TechCard
                        title="Hardware Enclaves"
                        description="Intel TDX / AMD SEV-SNP for tamper-proof execution"
                        features=vec!["Confidential computing", "Attestation", "Isolated execution"]
                    />
                    <TechCard
                        title="ZK Scaling"
                        description="SP1-powered zero-knowledge proofs for state compression"
                        features=vec!["99% storage reduction", "Near-zero gas", "Privacy-preserving"]
                    />
                </div>
            </section>
        </div>
    }
}

#[component]
fn TechCard(
    title: &'static str,
    description: &'static str,
    features: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <div class="tech-card">
            <h3 class="tech-title">{title}</h3>
            <p class="tech-description">{description}</p>
            <ul class="tech-features">
                {features.into_iter().map(|feature| {
                    view! { <li>{feature}</li> }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

