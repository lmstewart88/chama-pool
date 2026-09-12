#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, token, Address, Env};

fn setup_pool(env: &Env, goal: i128) -> (ContractClient<'_>, token::Client<'_>, Address, Address) {
    let token_admin = Address::generate(env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin);
    let token_client = token::Client::new(env, &token_contract.address());

    let pool_id = env.register(Contract, ());
    let client = ContractClient::new(env, &pool_id);
    let admin = Address::generate(env);
    let recipient = Address::generate(env);
    client.initialize(&admin, &recipient, &token_contract.address(), &goal);

    (client, token_client, token_contract.address(), recipient)
}

#[test]
fn contribution_below_goal_updates_total_without_release() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, token_client, token_address, recipient) = setup_pool(&env, 1_000);
    let contributor = Address::generate(&env);
    let asset = token::StellarAssetClient::new(&env, &token_address);
    asset.mint(&contributor, &400);

    client.contribute(&contributor, &400);

    assert_eq!(client.get_balance(), 400);
    assert_eq!(client.get_goal(), 1_000);
    assert!(!client.is_complete());
    assert_eq!(token_client.balance(&recipient), 0);
}

#[test]
fn contribution_meeting_goal_releases_entire_pool() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, token_client, token_address, recipient) = setup_pool(&env, 1_000);
    let contributor = Address::generate(&env);
    let asset = token::StellarAssetClient::new(&env, &token_address);
    asset.mint(&contributor, &1_200);

    client.contribute(&contributor, &1_200);

    assert_eq!(client.get_balance(), 1_200);
    assert!(client.is_complete());
    assert_eq!(token_client.balance(&recipient), 1_200);
}

#[test]
#[should_panic(expected = "pool already initialized")]
fn initialize_cannot_be_called_twice() {
    let env = Env::default();
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin);
    let pool_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &pool_id);
    let admin = Address::generate(&env);
    let recipient = Address::generate(&env);

    client.initialize(&admin, &recipient, &token_contract.address(), &100);
    client.initialize(&admin, &recipient, &token_contract.address(), &100);
}
