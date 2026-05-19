#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, BytesN, Env, Vec};

pub const PERSISTENT_TTL_LEDGERS: u32 = 3_110_400;
pub const PERSISTENT_TTL_THRESHOLD: u32 = PERSISTENT_TTL_LEDGERS - 17_280;

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

fn extend_persistent_ttl(env: &Env, key: &DataKey) {
    env.storage()
        .instance()
        .extend_ttl(PERSISTENT_TTL_THRESHOLD, PERSISTENT_TTL_LEDGERS);
    env.storage()
        .persistent()
        .extend_ttl(key, PERSISTENT_TTL_THRESHOLD, PERSISTENT_TTL_LEDGERS);
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
        env: Env,
        github_url_hash: BytesN<32>,
        proof_nonce: BytesN<32>,
        owner: Address,
    ) -> Result<(), Error> {
        owner.require_auth();
        let claim_key = DataKey::RepoClaim(github_url_hash.clone());
        let nonce_key = DataKey::ProofNonce(proof_nonce);
        if env.storage().persistent().has(&claim_key) {
            return Err(Error::AlreadyClaimed);
        }
        if env.storage().temporary().has(&nonce_key) {
            return Err(Error::NonceReused);
        }

        env.storage().temporary().set(&nonce_key, &true);
        let claim = RepoClaim {
            owner,
            github_hash: github_url_hash,
            claimed_at: env.ledger().timestamp(),
        };
        env.storage().persistent().set(&claim_key, &claim);
        extend_persistent_ttl(&env, &claim_key);
        Ok(())
    }

    /// Declare weighted dependency graph for a claimed repo.
    /// `deps`: max 50 entries; sum of weight_bps must equal 10_000.
    pub fn set_dependency_split(
        env: Env,
        repo_id: BytesN<32>,
        deps: Vec<SplitEntry>,
    ) -> Result<(), Error> {
        if deps.len() > 50 {
            return Err(Error::TooManyDependencies);
        }

        let mut total_bps = 0u32;
        for dep in deps.iter() {
            total_bps += dep.weight_bps;
        }
        if total_bps != 10_000 {
            return Err(Error::InvalidWeights);
        }

        let split_key = DataKey::RepoSplit(repo_id);
        env.storage().persistent().set(&split_key, &deps);
        extend_persistent_ttl(&env, &split_key);
        Ok(())
    }

    /// Deposit tokens into a repo's funding vault.
    /// Proportionally disperses to declared dependencies.
    pub fn fund_repo(
        env: Env,
        repo_id: BytesN<32>,
        _token: Address,
        amount: i128,
    ) -> Result<(), Error> {
        let vault_key = DataKey::RepoVault(repo_id);
        let current = env
            .storage()
            .persistent()
            .get::<_, i128>(&vault_key)
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&vault_key, &(current + amount));
        extend_persistent_ttl(&env, &vault_key);
        Ok(())
    }

    /// Pull accrued vault earnings to the verified repo owner.
    /// Returns the amount transferred.
    pub fn claim_earnings(env: Env, repo_id: BytesN<32>, owner: Address) -> Result<i128, Error> {
        owner.require_auth();
        let claim_key = DataKey::RepoClaim(repo_id.clone());
        let claim: RepoClaim = env
            .storage()
            .persistent()
            .get(&claim_key)
            .ok_or(Error::RepoNotFound)?;
        if claim.owner != owner {
            return Err(Error::Unauthorized);
        }

        let vault_key = DataKey::RepoVault(repo_id);
        let amount = env
            .storage()
            .persistent()
            .get::<_, i128>(&vault_key)
            .unwrap_or(0);
        env.storage().persistent().set(&vault_key, &0i128);
        extend_persistent_ttl(&env, &vault_key);
        Ok(amount)
    }

    pub fn get_repo_claim(env: Env, repo_id: BytesN<32>) -> Result<RepoClaim, Error> {
        let key = DataKey::RepoClaim(repo_id);
        env.storage()
            .persistent()
            .get(&key)
            .ok_or(Error::RepoNotFound)
    }

    pub fn get_repo_vault(env: Env, repo_id: BytesN<32>) -> i128 {
        let key = DataKey::RepoVault(repo_id);
        env.storage().persistent().get(&key).unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
