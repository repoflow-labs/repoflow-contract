#![cfg(test)]

extern crate std;

use super::*;
use soroban_sdk::{
    testutils::{storage::Persistent, Address as _, Ledger},
    Address, BytesN, Env, Vec,
};

fn setup() -> (Env, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(RepoFlow, ());
    (env, contract_id)
}

fn dummy_hash(env: &Env, seed: u8) -> BytesN<32> {
    BytesN::from_array(env, &[seed; 32])
}

#[test]
fn claim_repo_writes_claim_with_extended_ttl() {
    let (env, contract_id) = setup();
    let client = RepoFlowClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let hash = dummy_hash(&env, 1);
    let nonce = dummy_hash(&env, 2);

    client.claim_repo(&hash, &nonce, &owner);

    let claim = client.get_repo_claim(&hash);
    assert_eq!(claim.owner, owner);
    assert_eq!(claim.github_hash, hash);

    env.as_contract(&contract_id, || {
        let ttl = env
            .storage()
            .persistent()
            .get_ttl(&DataKey::RepoClaim(hash.clone()));
        assert_eq!(ttl, PERSISTENT_TTL_LEDGERS);
    });
}

#[test]
fn split_and_vault_writes_extend_ttl() {
    let (env, contract_id) = setup();
    let client = RepoFlowClient::new(&env, &contract_id);
    let repo_id = dummy_hash(&env, 3);
    let token = Address::generate(&env);
    let dep_id = dummy_hash(&env, 4);
    let mut deps = Vec::new(&env);
    deps.push_back(SplitEntry {
        dep_repo_id: dep_id,
        weight_bps: 10_000,
    });

    client.set_dependency_split(&repo_id, &deps);
    client.fund_repo(&repo_id, &token, &1_000i128);

    assert_eq!(client.get_repo_vault(&repo_id), 1_000);
    env.as_contract(&contract_id, || {
        let split_ttl = env
            .storage()
            .persistent()
            .get_ttl(&DataKey::RepoSplit(repo_id.clone()));
        let vault_ttl = env
            .storage()
            .persistent()
            .get_ttl(&DataKey::RepoVault(repo_id.clone()));
        assert_eq!(split_ttl, PERSISTENT_TTL_LEDGERS);
        assert_eq!(vault_ttl, PERSISTENT_TTL_LEDGERS);
    });
}

#[test]
fn claim_earnings_resets_vault_and_extends_ttl() {
    let (env, contract_id) = setup();
    let client = RepoFlowClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let repo_id = dummy_hash(&env, 5);
    let nonce = dummy_hash(&env, 6);
    let token = Address::generate(&env);

    client.claim_repo(&repo_id, &nonce, &owner);
    client.fund_repo(&repo_id, &token, &750i128);

    let claimed = client.claim_earnings(&repo_id, &owner);

    assert_eq!(claimed, 750);
    assert_eq!(client.get_repo_vault(&repo_id), 0);
    env.as_contract(&contract_id, || {
        let ttl = env
            .storage()
            .persistent()
            .get_ttl(&DataKey::RepoVault(repo_id.clone()));
        assert_eq!(ttl, PERSISTENT_TTL_LEDGERS);
    });
}

#[test]
fn claim_repo_rejects_reused_nonce() {
    let (env, contract_id) = setup();
    let client = RepoFlowClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let nonce = dummy_hash(&env, 7);

    client.claim_repo(&dummy_hash(&env, 8), &nonce, &owner);
    let result = client.try_claim_repo(&dummy_hash(&env, 9), &nonce, &owner);

    assert_eq!(result, Err(Ok(Error::NonceReused)));
}

#[test]
fn claim_persists_until_extended_ttl_boundary() {
    let (env, contract_id) = setup();
    let client = RepoFlowClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let repo_id = dummy_hash(&env, 10);
    let nonce = dummy_hash(&env, 11);

    client.claim_repo(&repo_id, &nonce, &owner);
    env.ledger().with_mut(|ledger| {
        ledger.sequence_number += PERSISTENT_TTL_LEDGERS - 1;
    });

    let claim = client.get_repo_claim(&repo_id);

    assert_eq!(claim.owner, owner);
}
