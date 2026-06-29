# RepoFlow

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Soroban](https://img.shields.io/badge/soroban--sdk-22-blueviolet)
![Network](https://img.shields.io/badge/network-Stellar-black)
[![Deployed](https://img.shields.io/badge/futurenet-deployed-success)](https://futurenet.stellar.expert/contract/CBAK7SEF7V6CHIZL4GJLJYEL44N3SIUMJXFFE7IGPLAXAR5MTLWFANRW)

GitHub Repository Claiming & Dependency Funding Graph Protocol for Stellar

## Technical Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              RepoFlow Architecture                       │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌──────────┐ │
│  │   Frontend  │───▶│   Backend   │───▶│  Soroban    │◀───│  Indexer │ │
│  │   (React)   │    │   (Rust)    │    │  Contract   │    │  (Rust)  │ │
│  └─────────────┘    └─────────────┘    └─────────────┘    └──────────┘ │
│        │                  │                  │                  │       │
│        ▼                  ▼                  ▼                  ▼       │
│  ┌────────────────────────────────────────────────────────────────────┐  │
│  │                     Data Pipeline                                  │  │
│  │  GitHub OAuth ──▶ Nonce Generation ──▶ Verification ──▶ On-chain   │  │
│  │  Proof         │    (HMAC-SHA256)    │    (GitHub API)  │   Claim  │  │
│  └────────────────────────────────────────────────────────────────────┘  │
│                                                                          │
│  ┌────────────────────────────────────────────────────────────────────┐  │
│  │                     Smart Contract State                           │  │
│  │  RepoClaim ◄──► RepoSplit ◄──► RepoVault ◄──► ProofNonce          │  │
│  └────────────────────────────────────────────────────────────────────┘  │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### Core Components

**Soroban Smart Contract (`contracts/repoflow/`)**
- `claim_repo(github_url_hash, proof_nonce, owner)` — verifies off-chain proof, stores repo → owner binding
- `set_dependency_split(repo_id, deps)` — declares weighted dependency graph (max 50 deps, depth 5)
- `fund_repo(repo_id, token, amount)` — deposits into funding vault, auto-disperses to dependencies
- `claim_earnings(repo_id, owner)` — pulls accrued earnings to owner

For the backend (Axum), indexer, SDK, and web app, see the companion submodules:
- **repoflow-indexer** — on-chain event indexer & REST API
- **repoflow-sdk** — TypeScript client library
- **repoflow-app** — web dashboard (Next.js)

## Local Development Setup

### Prerequisites

| Tool | Version | Installation |
|------|---------|--------------|
| Rust | 1.77+ | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` |
| Node.js | 20 LTS | `nvm install 20` |
| Docker | Latest | `brew install docker` (macOS) |
| PostgreSQL | 15+ | `brew install postgresql@15` |
| Redis | 7+ | `brew install redis` |
|stellar-cli | latest | `cargo install stellar-cli --locked` |

### Environment Setup

```bash
# Clone repository
git clone https://github.com/repoflow-labs/repoflow-contract.git
cd repoflow-contract

# Build the Soroban contract
cargo build --release --manifest-path contracts/repoflow/Cargo.toml

# Run contract tests
cargo test --manifest-path contracts/repoflow/Cargo.toml
```

For the indexer, SDK, and web app, clone the respective companion repositories.

### Soroban Contract Development

```bash
# Build contract
cargo build --release --manifest-path contracts/repoflow/Cargo.toml

# Run contract tests
cargo test --manifest-path contracts/repoflow/Cargo.toml

# Deploy to local network
stellar contract deploy \
  --wasm target/wasm32v1-none/release/repoflow_contract.wasm \
  --source repoflow-deployer-new \
  --network futurenet

Live contract: https://futurenet.stellar.expert/contract/CBAK7SEF7V6CHIZL4GJLJYEL44N3SIUMJXFFE7IGPLAXAR5MTLWFANRW

# Invoke contract locally
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network local \
  -- claim_repo \
  --github_url_hash 0x... \
  --proof_nonce 0x... \
  --owner G...
```

### Running Tests

```bash
# Contract tests
cargo test --manifest-path contracts/repoflow/Cargo.toml

# Contract tests with output
cargo test --manifest-path contracts/repoflow/Cargo.toml -- --nocapture
```


## Technology Stack

| Component | Technology | Version |
|-----------|------------|---------|
| Language | Rust | 1.77+ |
| Framework | Soroban SDK | latest |
| WASM Target | `wasm32v1-none` | stable |
| Build Tool | Cargo | 1.77+ |

## Project Structure

```
repoflow-contract/
├── contracts/
│   └── repoflow/          # Soroban smart contract (Rust)
│       ├── src/
│       │   ├── lib.rs    # Contract entry points
│       │   ├── repo.rs   # Repo claim logic
│       │   ├── split.rs  # Dependency split logic
│       │   └── vault.rs  # Funding vault logic
│       ├── Cargo.toml
│       └── test.rs       # Contract unit tests
├── .github/
│   ├── workflows/        # CI/CD workflows
│   ├── ISSUE_TEMPLATE/  # Issue templates
│   └── PULL_REQUEST_TEMPLATE.md
├── README.md
├── CONTRIBUTING.md
└── LICENSE
```

## Quick Start Commands

```bash
# Build contract
cargo build --release --manifest-path contracts/repoflow/Cargo.toml

# Run tests
cargo test --manifest-path contracts/repoflow/Cargo.toml

# Lint
cargo fmt --check
cargo clippy --manifest-path contracts/repoflow/Cargo.toml -- -D warnings

## Security

See [SECURITY.md](SECURITY.md) for vulnerability reporting.
```