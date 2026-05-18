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

**Backend (`backend/`)**
- GitHub OAuth service with PKCE flow
- Nonce generation: HMAC-SHA256(stellar_address + repo_url + timestamp)
- Verification worker: polls GitHub API every 30s for nonce presence
- Graph indexer: tracks `RepoClaimed`, `SplitSet`, `FundingDeposited` events

**Frontend (`frontend/`)**
- GitHub OAuth login → claimable repos display
- 4-step claiming flow with live verification status
- D3.js force-directed dependency graph visualization
- Funding dashboard with withdrawal functionality

**Indexer (`indexer/`)**
- Event subscription to Soroban contract events
- PostgreSQL materialization with ltree for hierarchical paths
- GraphQL API for dependency tree queries

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

# Install Rust dependencies
cargo build --workspace

# Install frontend dependencies
cd frontend && npm install && cd ..

# Start local services
docker run -d --name stellar-quickstart \
  -p 8000:8000 \
  stellar/quickstart:latest --standalone

# Start PostgreSQL
brew services start postgresql@15

# Start Redis
brew services start redis

# Create database
createdb repoflow_dev

# Run migrations
cd backend && cargo run --bin migrate

# Start backend server
cd backend && cargo run --bin server

# Start frontend dev server (new terminal)
cd frontend && npm run dev
```

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
# Unit tests (all crates)
cargo test --workspace

# Integration tests (backend)
cd backend && cargo test --test integration

# Frontend tests
cd frontend && npm test

# Contract tests with coverage
cargo test --manifest-path contracts/repoflow/Cargo.toml -- --nocapture
```

### Configuration

Create `.env` in project root:

```bash
# Backend
DATABASE_URL=postgres://user:pass@localhost:5432/repoflow_dev
REDIS_URL=redis://localhost:6379
GITHUB_CLIENT_ID=your_github_client_id
GITHUB_CLIENT_SECRET=your_github_client_secret
STELLAR_NETWORK=local
STELLAR_SECRET_KEY=your_secret_key

# Frontend
NEXT_PUBLIC_CONTRACT_ID=your_contract_id
NEXT_PUBLIC_STELLAR_NETWORK=local
```

## Complete Technology Stack

### Smart Contract

| Component | Technology | Version |
|-----------|------------|---------|
| Language | Rust | 1.77+ |
| Framework | Soroban SDK | latest (soroban-sdk crate) |
| WASM Target | `wasm32v1-none` | stable |
| Build Tool | Cargo | 1.77+ |

### Backend

| Component | Technology | Version |
|-----------|------------|---------|
| Language | Rust | 1.77+ |
| Runtime | Tokio | 1.x |
| Web Framework | Axum | 0.7.x |
| HTTP Client | reqwest | 0.11.x |
| GitHub Client | octocrab | 0.40.x |
| Database | PostgreSQL | 15.x |
| ORM | SQLx | 0.7.x |
| GraphQL | async-graphql | 7.x |
| Caching | Redis | 7.x (via redis-rs) |
| Signing | ed25519-dalek | 1.0.x |

### Frontend

| Component | Technology | Version |
|-----------|------------|---------|
| Framework | React | 18.x |
| Build Tool | Next.js | 14.x |
| Language | TypeScript | 5.x |
| Styling | Tailwind CSS | 3.x |
| Graph Viz | D3.js | 7.x |
| Wallet | @stellar/freighter-api | latest |
| State | Zustand | 4.x |
| GraphQL Client | urql | 4.x |

### Infrastructure

| Component | Technology | Version |
|-----------|------------|---------|
| Database | PostgreSQL + ltree | 15.x |
| Cache | Redis | 7.x |
| Node Runtime | Stellar Quickstart Docker | latest |
| CI/CD | GitHub Actions | - |
| Package Manager | npm | 10.x |
| Package Manager (Rust) | Cargo | 1.77+ |

### Key Dependencies (Cargo.toml - contract)

```toml
[dependencies]
soroban-sdk = "22"
soroban-sdk-macros = "22"
stellar-xdr = "22"
```

### Key Dependencies (package.json - frontend)

```json
{
  "dependencies": {
    "@stellar/freighter-api": "^1.7.0",
    "d3": "^7.8.5",
    "next": "^14.1.0",
    "react": "^18.2.0",
    "urql": "^4.0.6",
    "zustand": "^4.5.0"
  },
  "devDependencies": {
    "@types/d3": "^7.4.3",
    "tailwindcss": "^3.4.1",
    "typescript": "^5.3.3"
  }
}
```

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
├── backend/               # Rust Axum backend
│   ├── src/
│   │   ├── api/          # HTTP handlers
│   │   ├── db/           # SQLx models
│   │   ├── github/       # GitHub OAuth + verification
│   │   └── indexer/      # Event indexer
│   └── migrations/       # PostgreSQL migrations
├── frontend/              # Next.js + React frontend
│   ├── src/
│   │   ├── components/   # React components
│   │   ├── pages/        # Next.js pages
│   │   ├── hooks/        # Custom hooks
│   │   └── lib/          # Utility functions
│   └── public/           # Static assets
├── indexer/               # Event indexer service
├── .github/
│   ├── workflows/        # CI/CD workflows
│   ├── ISSUE_TEMPLATE/  # Issue templates
│   └── PULL_REQUEST_TEMPLATE.md
├── README.md
├── CONTRIBUTING.md
└── Cargo.toml            # Workspace manifest
```

## Quick Start Commands

```bash
# Full local environment
make dev

# Build all
make build

# Run all tests
make test

# Deploy contract
make deploy

# Lint
cargo fmt --check
cargo clippy -- -D warnings
npm run lint
```