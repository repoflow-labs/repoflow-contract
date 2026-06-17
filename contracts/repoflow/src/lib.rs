#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, BytesN, Env, Symbol, Vec};

#[cfg(test)]
mod test;

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
    pub fn claim_repo(
        env: Env,
        github_url_hash: BytesN<32>,
        proof_nonce: BytesN<32>,
        owner: Address,
    ) -> Result<(), Error> {
        owner.require_auth();

        if env.storage().temporary().has(&DataKey::ProofNonce(proof_nonce.clone())) {
            return Err(Error::NonceReused);
        }

        if env.storage().persistent().has(&DataKey::RepoClaim(github_url_hash.clone())) {
            return Err(Error::AlreadyClaimed);
        }

        let claim = RepoClaim {
            owner: owner.clone(),
            github_hash: github_url_hash.clone(),
            claimed_at: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&DataKey::RepoClaim(github_url_hash.clone()), &claim);
        env.storage().persistent().extend_ttl(
            &DataKey::RepoClaim(github_url_hash.clone()),
            3_110_400,
            3_110_400,
        );

        env.storage().temporary().set(&DataKey::ProofNonce(proof_nonce.clone()), &());
        env.storage().temporary().extend_ttl(
            &DataKey::ProofNonce(proof_nonce),
            17_280,
            17_280,
        );

        env.events().publish((Symbol::new(&env, "RepoClaimed"), github_url_hash), owner);

        Ok(())
    }

    pub fn set_dependency_split(
        env: Env,
        repo_id: BytesN<32>,
        deps: Vec<SplitEntry>,
    ) -> Result<(), Error> {
        let claim: RepoClaim = env.storage().persistent()
            .get(&DataKey::RepoClaim(repo_id.clone()))
            .ok_or(Error::RepoNotFound)?;

        claim.owner.require_auth();

        if deps.len() > 50 {
            return Err(Error::TooManyDependencies);
        }

        let sum: u32 = deps.iter().map(|d| d.weight_bps).sum();
        if sum != 10_000 {
            return Err(Error::InvalidWeights);
        }

        env.storage().persistent().set(&DataKey::RepoSplit(repo_id.clone()), &deps);
        env.storage().persistent().extend_ttl(
            &DataKey::RepoSplit(repo_id.clone()),
            3_110_400,
            3_110_400,
        );

        env.events().publish((Symbol::new(&env, "SplitSet"), repo_id), deps);

        Ok(())
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
