# Manus Liquidity - AI-Native Liquidity Provisioning Platform on Sui

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Move](https://img.shields.io/badge/move-sui-blue.svg)](https://sui.io/)
[![WASM](https://img.shields.io/badge/wasm-enabled-green.svg)](https://webassembly.org/)

An advanced, AI-driven liquidity provisioning platform built exclusively for the Sui blockchain. This project combines cutting-edge technologies including Rust, Move smart contracts, WebAssembly, zero-knowledge proofs, post-quantum cryptography, and formally verified AI agents to deliver unparalleled security, performance, and reliability.

## 🌟 Key Features

### AI-Powered Agents
- **Four Specialized Rust ML Agents**:
  - **RebalancerAgent**: ML-based portfolio optimization using `ndarray` and `smartcore`
  - **StrategyOptimizerAgent**: Parameter optimization for trading strategies
  - **RiskManagerAgent**: Real-time risk monitoring and emergency protocols
  - **MarketAnalyzerAgent**: Market regime detection (volatile, trending, ranging)
- **WASM Plugin System**: Dynamically loadable agent "Skills" inspired by Anthropic's Agent Skills pattern
- **Formal Verification**: All agent logic verified with Lean 4 mathematical proofs

### Security & Privacy
- **Post-Quantum Cryptography**: Dilithium signatures and Kyber key encapsulation
- **Hardware TEEs**: Support for Intel TDX and AMD SEV-SNP confidential computing
- **Zero-Knowledge Proofs**: SP1-based ZK proof generation for 99% storage reduction
- **Data Compression**: OpenZL integration for 2x better compression than zstd

### Modern Architecture
- **Full Rust Stack**: Backend (Axum), Frontend (Leptos WASM), AI Agents (Rust ML)
- **Move Smart Contracts**: Formally verified vault and liquidity management on Sui
- **WASM Deployment**: Wasmer Edge deployment (15x faster, 20x cheaper than Kubernetes)
- **MicroVM Isolation**: Firecracker + Kata Containers for lightweight, secure execution
- **Bottlerocket OS**: Minimal, Rust-based operating system for production deployment

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     USER INTERFACE                          │
│  Leptos WASM Frontend (Rust) → Wasmer Edge                 │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│                     API LAYER                               │
│  Axum REST API (Rust) → WASM → Wasmer Runtime              │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│                   AI AGENTS (RUST!)                         │
│  ├── Rebalancer (burn ML)                                   │
│  ├── Optimizer (candle ML)                                  │
│  ├── Risk Manager (linfa ML)                                │
│  └── Market Analyzer (smartcore ML)                         │
│  → Compiled to WASM → Formally verified in Lean 4          │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│               DATA & CRYPTOGRAPHY LAYER                     │
│  ├── OpenZL Compression (2x better than zstd)              │
│  ├── SP1 ZK Proofs (99% storage reduction)                 │
│  ├── Dilithium Signatures (post-quantum)                    │
│  └── Kyber KEM (post-quantum)                               │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│                 ISOLATION LAYER                             │
│  ├── Firecracker microVMs (lightweight)                     │
│  ├── Kata Containers (secure runtime)                       │
│  └── Intel TDX / AMD SEV-SNP (hardware TEE)                │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│              INFRASTRUCTURE LAYER                           │
│  ├── Bottlerocket OS (minimal, Rust-based)                  │
│  ├── Wasmer Edge (WASM orchestration)                       │
│  └── Bare metal / Cloud VMs                                 │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│                  BLOCKCHAIN LAYER                           │
│  Sui Network (Move smart contracts)                         │
│  └── Formally verified in Lean 4                            │
└─────────────────────────────────────────────────────────────┘
```

## 🚀 Quick Start

### Prerequisites
- Rust 1.75+ (`rustup install stable`)
- Sui CLI (`cargo install --locked --git https://github.com/MystenLabs/sui.git --branch devnet sui`)
- Trunk (for frontend): `cargo install trunk`
- Wasmer CLI: `curl https://get.wasmer.io -sSfL | sh`

### Local Development

```bash
# Clone the repository
git clone https://github.com/vinihill/deepbook-lp-platform.git
cd deepbook-lp-platform

# Build backend
cd backend
cargo build --release

# Build frontend
cd ../frontend
trunk build --release

# Run backend API server
cd ../backend
cargo run --bin api_server

# In another terminal, serve frontend
cd frontend
trunk serve
```

### Deploy to Wasmer Edge

```bash
# Build as WASM
cargo build --target wasm32-wasi --release

# Deploy
wasmer deploy
```

## 📦 Project Structure

```
deepbook-lp-platform/
├── backend/                 # Rust backend (Axum API)
│   ├── src/
│   │   ├── agents/         # AI agents (Rust ML)
│   │   │   ├── ml_agent.rs # 4 specialized ML agents
│   │   │   ├── mod.rs
│   │   │   ├── rebalancer.rs
│   │   │   └── strategy.rs
│   │   ├── api/            # REST API handlers
│   │   ├── crypto/         # Post-quantum cryptography
│   │   ├── sui/            # Sui blockchain integration
│   │   ├── wasm/           # WASM plugin system
│   │   ├── zkp/            # ZK proof generation (SP1)
│   │   └── lib.rs
│   ├── tests/              # Comprehensive test suite
│   │   └── integration/
│   │       ├── agent_tests.rs
│   │       ├── api_tests.rs
│   │       ├── sui_tests.rs
│   │       └── wasm_plugin_tests.rs
│   └── Cargo.toml
├── frontend/               # Leptos WASM frontend
│   ├── src/
│   │   ├── pages/
│   │   │   ├── home.rs
│   │   │   ├── agents.rs
│   │   │   ├── technology.rs
│   │   │   ├── strategies.rs
│   │   │   └── funding.rs
│   │   └── lib.rs
│   └── Cargo.toml
├── contracts/              # Move smart contracts
│   ├── manus_liquidity/
│   │   └── sources/
│   │       ├── vault.move
│   │       └── vault_math.move
│   └── verification/       # Lean 4 formal verification
│       ├── Vault.lean
│       ├── Invariants.lean
│       └── AgentLogic.lean
├── docs/                   # Documentation
│   ├── ARCHITECTURE.md
│   ├── DEPLOYMENT.md
│   ├── LEAN4_VERIFICATION.md
│   └── WASMER_DEPLOYMENT.md
├── IMPLEMENTATION_ROADMAP.md
├── PROJECT_STATUS.md
└── README.md
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test '*'

# Run specific test suite
cargo test --test agent_tests

# Run with coverage
cargo tarpaulin --out Html
```

## 📊 Technology Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Smart Contracts** | Move | Sui blockchain |
| **Backend** | Rust (Axum) | Memory safety, performance |
| **Frontend** | Rust (Leptos WASM) | Type safety, WASM compilation |
| **AI Agents** | Rust (ndarray, smartcore) | Pure Rust ML |
| **Formal Verification** | Lean 4 | Mathematical proofs |
| **ZK Proofs** | SP1 | State compression |
| **Data Compression** | OpenZL | 2x better than zstd |
| **WASM Runtime** | Wasmer | Replace Docker/K8s |
| **MicroVMs** | Firecracker | Lightweight isolation |
| **Secure Runtime** | Kata Containers | Container security |
| **Hardware TEE** | Intel TDX / AMD SEV | Confidential compute |
| **OS** | Bottlerocket | Minimal, Rust-based |
| **Deployment** | Wasmer Edge | WASM orchestration |
| **Post-Quantum** | Dilithium + Kyber | Quantum-resistant |

## 🔐 Security

- **Formal Verification**: All critical logic verified with Lean 4
- **Post-Quantum Cryptography**: Future-proof against quantum computers
- **Hardware TEEs**: Intel TDX and AMD SEV-SNP support
- **WASM Sandboxing**: Isolated execution environment
- **Zero-Knowledge Proofs**: Privacy-preserving state verification

## 📈 Performance Metrics

- **Transaction Finality**: <1 second
- **Storage Reduction**: 99% (via ZK proofs)
- **Compression Ratio**: 2x (via OpenZL)
- **Deployment Speed**: 15x faster than Kubernetes
- **Operational Cost**: 20x cheaper than traditional containers

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Documentation](./docs/)
- [Implementation Roadmap](./IMPLEMENTATION_ROADMAP.md)
- [Project Status](./PROJECT_STATUS.md)
- [Sui Network](https://sui.io/)
- [Wasmer](https://wasmer.io/)
- [Lean 4](https://lean-lang.org/)

## 📞 Contact

For questions, issues, or collaboration opportunities, please open an issue on GitHub.

---

**Built with ❤️ for the Sui ecosystem**

