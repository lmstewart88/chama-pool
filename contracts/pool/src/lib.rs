#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contract]
pub struct Contract;

#[contracttype]
enum DataKey {
    Initialized,
    Admin,
    Recipient,
    Token,
    Goal,
    TotalRaised,
    Complete,
}

#[contractimpl]
impl Contract {
    /// Configures the pool. A pool configuration is immutable once created.
    pub fn initialize(env: Env, admin: Address, recipient: Address, token: Address, goal: i128) {
        assert!(
            !env.storage().persistent().has(&DataKey::Initialized),
            "pool already initialized"
        );
        assert!(goal > 0, "goal must be positive");

        let storage = env.storage().persistent();
        storage.set(&DataKey::Initialized, &true);
        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Recipient, &recipient);
        storage.set(&DataKey::Token, &token);
        storage.set(&DataKey::Goal, &goal);
        storage.set(&DataKey::TotalRaised, &0_i128);
        storage.set(&DataKey::Complete, &false);
    }

    /// Moves a contributor's tokens into the pool and releases the entire pool
    /// to the recipient as soon as its target is reached.
    pub fn contribute(env: Env, from: Address, amount: i128) {
        Self::require_initialized(&env);
        assert!(amount > 0, "contribution must be positive");
        assert!(
            !env.storage()
                .persistent()
                .get(&DataKey::Complete)
                .unwrap_or(false),
            "pool already complete"
        );

        from.require_auth();

        let storage = env.storage().persistent();
        let token_address: Address = storage.get(&DataKey::Token).unwrap();
        let total: i128 = storage.get(&DataKey::TotalRaised).unwrap();
        let new_total = total.checked_add(amount).expect("total overflow");
        let pool_address = env.current_contract_address();
        let token_client = token::Client::new(&env, &token_address);

        token_client.transfer(&from, &pool_address, &amount);
        storage.set(&DataKey::TotalRaised, &new_total);

        let goal: i128 = storage.get(&DataKey::Goal).unwrap();
        if new_total >= goal {
            let recipient: Address = storage.get(&DataKey::Recipient).unwrap();
            let pooled_balance = token_client.balance(&pool_address);
            token_client.transfer(&pool_address, &recipient, &pooled_balance);
            storage.set(&DataKey::Complete, &true);
        }
    }

    pub fn get_balance(env: Env) -> i128 {
        Self::require_initialized(&env);
        env.storage()
            .persistent()
            .get(&DataKey::TotalRaised)
            .unwrap()
    }

    pub fn get_goal(env: Env) -> i128 {
        Self::require_initialized(&env);
        env.storage().persistent().get(&DataKey::Goal).unwrap()
    }

    pub fn is_complete(env: Env) -> bool {
        Self::require_initialized(&env);
        env.storage().persistent().get(&DataKey::Complete).unwrap()
    }

    fn require_initialized(env: &Env) {
        assert!(
            env.storage().persistent().has(&DataKey::Initialized),
            "pool not initialized"
        );
    }
}

mod test;
