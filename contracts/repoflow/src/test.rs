#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Events, Ledger},
    Address, BytesN, Env,
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
fn test_claim_repo_success() {
    let (env, contract_id) = setup();
    let client = RepoFlowClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let hash = dummy_hash(&env, 1);
    let nonce = dummy_hash(&env, 2);

    client.claim_repo(&hash, &nonce, &owner);
}

#[test]
fn test_dependency_split_success() {
    let (env, contract_id) = setup();
    let client = RepoFlowClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let repo_id = dummy_hash(&env, 1);
    let nonce = dummy_hash(&env, 2);

    client.claim_repo(&repo_id, &nonce, &owner);

    let deps = soroban_sdk::vec![
        &env,
        SplitEntry { dep_repo_id: dummy_hash(&env, 3), weight_bps: 6000 },
        SplitEntry { dep_repo_id: dummy_hash(&env, 4), weight_bps: 4000 },
    ];

    client.set_dependency_split(&repo_id, &deps);
}

#[test]
fn test_claim_repo_stores_data() {
    let (env, contract_id) = setup();
    let client = RepoFlowClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let hash = dummy_hash(&env, 1);
    let nonce = dummy_hash(&env, 2);

    env.ledger().set_timestamp(1_700_000_000);
    client.claim_repo(&hash, &nonce, &owner);

    let stored: RepoClaim = env.as_contract(&contract_id, || {
        env.storage().persistent()
            .get(&DataKey::RepoClaim(hash))
            .unwrap()
    });
    assert_eq!(stored.owner, owner);
    assert_eq!(stored.github_hash, dummy_hash(&env, 1));
    assert_eq!(stored.claimed_at, 1_700_000_000);
}

#[test]
fn test_proof_nonce_stored_in_temporary() {
    let (env, contract_id) = setup();
    let client = RepoFlowClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let hash = dummy_hash(&env, 1);
    let nonce = dummy_hash(&env, 2);

    client.claim_repo(&hash, &nonce, &owner);

    let stored: bool = env.as_contract(&contract_id, || {
        env.storage().temporary().has(&DataKey::ProofNonce(nonce))
    });
    assert!(stored);
}

#[test]
fn test_nonce_not_in_persistent() {
    let (env, contract_id) = setup();
    let client = RepoFlowClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let hash1 = dummy_hash(&env, 1);
    let nonce1 = dummy_hash(&env, 2);

    client.claim_repo(&hash1, &nonce1, &owner);

    let stored: bool = env.as_contract(&contract_id, || {
        env.storage().persistent().has(&DataKey::ProofNonce(
            dummy_hash(&env, 2),
        ))
    });
    assert!(!stored);
}

#[test]
fn test_claim_repo_emits_event() {
    let (env, contract_id) = setup();
    let client = RepoFlowClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let hash = dummy_hash(&env, 1);
    let nonce = dummy_hash(&env, 2);

    client.claim_repo(&hash, &nonce, &owner);

    let events = env.events().all();
    assert_eq!(events.len(), 1);
    let (event_contract, _topics, _data) = events.get(0).unwrap();
    assert_eq!(event_contract, contract_id);
}
