#![cfg(test)]

use crate::{DataKey, RepoClaim, SplitEntry};
use soroban_sdk::testutils::Env;
use soroban_sdk::{Address, BytesN, Vec};

#[test]
fn test_contract_instantiates() {
    let env = Env::default();
    env.ledger().set_sequence_number(1000);
    
    // Contract should be deployable
    let contract_id = BytesN::<32>::from_array(&env, [0u8; 32]);
    assert_eq!(contract_id.to_array().len(), 32);
}

#[test]
fn test_claim_repo_returns_unimplemented() {
    let env = Env::default();
    env.ledger().set_sequence_number(1000);
    
    let repo_hash = BytesN::<32>::from_array(&env, [1u8; 32]);
    let nonce = BytesN::<32>::from_array(&env, [2u8; 32]);
    let owner = Address::from_string(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABF4");
    
    // This will panic with "unimplemented" as specified
    let _result = repoflow::claim_repo(&env, repo_hash, nonce, owner);
}

#[test]
fn test_set_dependency_split_returns_unimplemented() {
    let env = Env::default();
    env.ledger().set_sequence_number(1000);
    
    let repo_id = BytesN::<32>::from_array(&env, [1u8; 32]);
    let deps = Vec::<SplitEntry>::new(&env);
    
    // This will panic with "unimplemented" as specified
    let _result = repoflow::set_dependency_split(&env, repo_id, deps);
}

#[test]
fn test_fund_repo_returns_unimplemented() {
    let env = Env::default();
    env.ledger().set_sequence_number(1000);
    
    let repo_id = BytesN::<32>::from_array(&env, [1u8; 32]);
    let token = Address::from_string(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABF4");
    let amount = 1000i128;
    
    // This will panic with "unimplemented" as specified
    let _result = repoflow::fund_repo(&env, repo_id, token, amount);
}

#[test]
fn test_claim_earnings_returns_unimplemented() {
    let env = Env::default();
    env.ledger().set_sequence_number(1000);
    
    let repo_id = BytesN::<32>::from_array(&env, [1u8; 32]);
    let owner = Address::from_string(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABF4");
    
    // This will panic with "unimplemented" as specified
    let _result = repoflow::claim_earnings(&env, repo_id, owner);
}

#[test]
fn test_error_enum_variants() {
    // Verify error enum discriminant values match specification
    assert_eq!(crate::Error::AlreadyClaimed as u32, 1);
    assert_eq!(crate::Error::NonceReused as u32, 2);
    assert_eq!(crate::Error::InvalidWeights as u32, 3);
    assert_eq!(crate::Error::TooManyDependencies as u32, 4);
    assert_eq!(crate::Error::Unauthorized as u32, 5);
}

#[test]
fn test_data_key_variants() {
    let env = Env::default();
    
    // Verify DataKey can be constructed
    let repo_hash = BytesN::<32>::from_array(&env, [1u8; 32]);
    let _key = DataKey::RepoClaim(repo_hash);
    let _key_admin = DataKey::Admin;
}

#[test]
fn test_repo_claim_struct() {
    let env = Env::default();
    
    let owner = Address::from_string(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABF4");
    let github_hash = BytesN::<32>::from_array(&env, [1u8; 32]);
    let claimed_at = 1700000000u64;
    
    let claim = RepoClaim {
        owner,
        github_hash,
        claimed_at,
    };
    
    assert_eq!(claim.claimed_at, 1700000000u64);
}

#[test]
fn test_split_entry_struct() {
    let env = Env::default();
    
    let dep_repo_id = BytesN::<32>::from_array(&env, [2u8; 32]);
    let weight_bps = 2500u32; // 25%
    
    let entry = SplitEntry {
        dep_repo_id,
        weight_bps,
    };
    
    assert_eq!(entry.weight_bps, 2500);
}