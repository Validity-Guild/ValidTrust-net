#![cfg(test)]

use super::*;
use soroban_sdk::testutils::{Address as _, MockAuth, MockAuthInvoke};
use soroban_sdk::{token, Address, Env, IntoVal};

fn create_token_contract<'a>(env: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(env, &env.register_stellar_asset_contract(admin.clone()))
}

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ValidTrustVault);
    let client = ValidTrustVaultClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    client.initialize(&admin, &token.address);

    // Should fail if initialized again
    let result = client.try_initialize(&admin, &token.address);
    assert!(result.is_err());
}

#[test]
fn test_deposit_withdraw() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, ValidTrustVault);
    let client = ValidTrustVaultClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    // Initialize vault
    client.initialize(&admin, &token.address);

    // Mint tokens to user for testing
    let amount = 1000;
    token.mint(&user, &amount);

    // Deposit
    client.deposit(&user, &500);

    // Verify balance
    let balance = client.balance(&user);
    assert_eq!(balance, 500);
    assert_eq!(token.balance(&user), 500);
    assert_eq!(token.balance(&contract_id), 500);

    // Withdraw
    client.withdraw(&user, &200);
    let balance_after_withdraw = client.balance(&user);
    assert_eq!(balance_after_withdraw, 300);
    assert_eq!(token.balance(&user), 700);
}

#[test]
fn test_distribute_rewards() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, ValidTrustVault);
    let client = ValidTrustVaultClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    client.initialize(&admin, &token.address);

    token.mint(&user1, &1000);
    token.mint(&user2, &1000);

    client.deposit(&user1, &100);
    client.deposit(&user2, &300);

    // Distribute a total reward pool of 400
    // user1 should get 100, user2 should get 300
    client.distribute_reward(&user1, &400);
    client.distribute_reward(&user2, &400);

    assert_eq!(client.balance(&user1), 200);
    assert_eq!(client.balance(&user2), 600);
}

// TODO: Add more edge case tests (e.g. withdrawing more than balance, uninitialized state calls)
