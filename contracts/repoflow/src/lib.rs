#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, BytesN, Env, Vec};

// ── Storage Keys ─────────────────────────────────────────────────────────────
// RepoClaim   → Persistent  (user funds; must never expire)
// RepoSplit   → Persistent  (dependency graph; long-lived)
// RepoVault   → Persistent  (accrued earnings; long-lived)
// ProofNonce  → Temporary   (anti-replay; auto-expires ~24h)
// Admin       → Instance    (hot path; never changes)
#[contracttype]
pub enum DataKey {
    RepoClaim(BytesN<32>),
    RepoSplit(BytesN<32>),
    RepoVault(BytesN<32>),
    ProofNonce(BytesN<32>),
    Admin,
}

// ── Data Structures ───────────────────────────────────────────────────────────
/// On-chain record of a claimed GitHub repository.
#[contracttype]
#[derive(Clone)]
pub struct RepoClaim {
    /// Stellar address of the verified repo owner.
    pub owner: Address,
    /// SHA-256 hash of the canonical GitHub repository URL.
    pub github_hash: BytesN<32>,
    /// Ledger timestamp at claim time.
    pub claimed_at: u64,
}

/// One entry in a repo's weighted dependency split.
#[contracttype]
#[derive(Clone)]
pub struct SplitEntry {
    /// SHA-256 hash of the dependent repo URL.
    pub dep_repo_id: BytesN<32>,
    /// Basis points (sum of all entries must equal 10_000).
    pub weight_bps: u32,
}

// ── Error Enum ────────────────────────────────────────────────────────────────
#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyClaimed = 1,
    NonceReused = 2,
    InvalidWeights = 3,
    TooManyDependencies = 4,
    Unauthorized = 5,
    RepoNotFound = 6,
}

// ── Contract ──────────────────────────────────────────────────────────────────
#[contract]
pub struct RepoFlow;

#[contractimpl]
impl RepoFlow {
    /// Claim ownership of a GitHub repository on-chain.
    /// `github_url_hash`: SHA-256 of canonical repo URL.
    /// `proof_nonce`: HMAC-SHA256 nonce verified off-chain by the backend.
    /// `owner`: Stellar address asserting ownership; must authorize this call.
    pub fn claim_repo(
        _env: Env,
        _github_url_hash: BytesN<32>,
        _proof_nonce: BytesN<32>,
        owner: Address,
    ) -> Result<(), Error> {
        owner.require_auth();
        unimplemented!()
    }

    /// Declare weighted dependency graph for a claimed repo.
    /// `deps`: max 50 entries; sum of weight_bps must equal 10_000.
    pub fn set_dependency_split(
        _env: Env,
        _repo_id: BytesN<32>,
        _deps: Vec<SplitEntry>,
    ) -> Result<(), Error> {
        unimplemented!()
    }

    /// Deposit tokens into a repo's funding vault.
    /// Proportionally disperses to declared dependencies.
    pub fn fund_repo(
        _env: Env,
        _repo_id: BytesN<32>,
        _token: Address,
        _amount: i128,
    ) -> Result<(), Error> {
        unimplemented!()
    }

    /// Pull accrued vault earnings to the verified repo owner.
    /// Returns the amount transferred.
    pub fn claim_earnings(_env: Env, _repo_id: BytesN<32>, owner: Address) -> Result<i128, Error> {
        owner.require_auth();
        unimplemented!()
    }
}
