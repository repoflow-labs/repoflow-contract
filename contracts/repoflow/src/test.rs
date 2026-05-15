#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
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
fn test_claim_repo_compiles() {
    let (env, contract_id) = setup();
    let client = RepoFlowClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let hash = dummy_hash(&env, 1);
    let nonce = dummy_hash(&env, 2);
    // stubs return unimplemented — test verifies contract instantiates and
    // function signatures are callable without type errors.
    let _ = std::panic::catch_unwind(|| {
        client.claim_repo(&hash, &nonce, &owner);
    });
}

#[test]
fn test_set_dependency_split_compiles() {
    let (env, contract_id) = setup();
    let client = RepoFlowClient::new(&env, &contract_id);
    let repo_id = dummy_hash(&env, 3);
    let deps: Vec<SplitEntry> = Vec::new(&env);
    let _ = std::panic::catch_unwind(|| {
        client.set_dependency_split(&repo_id, &deps);
    });
}

#[test]
fn test_fund_repo_compiles() {
    let (env, contract_id) = setup();
    let client = RepoFlowClient::new(&env, &contract_id);
    let repo_id = dummy_hash(&env, 4);
    let token = Address::generate(&env);
    let _ = std::panic::catch_unwind(|| {
        client.fund_repo(&repo_id, &token, &1000i128);
    });
}

#[test]
fn test_claim_earnings_compiles() {
    let (env, contract_id) = setup();
    let client = RepoFlowClient::new(&env, &contract_id);
    let repo_id = dummy_hash(&env, 5);
    let owner = Address::generate(&env);
    let _ = std::panic::catch_unwind(|| {
        client.claim_earnings(&repo_id, &owner);
    });
}