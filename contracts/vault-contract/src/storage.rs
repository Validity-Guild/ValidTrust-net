#![no_std]

use soroban_sdk::{contracttype, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Token,
    Admin,
    Initialized,
    Balance(Address),
    TotalShares,
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub fn read_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn write_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn has_token(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Token)
}

pub fn read_token(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Token).unwrap()
}

pub fn write_token(env: &Env, token: &Address) {
    env.storage().instance().set(&DataKey::Token, token);
}

pub fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Initialized)
}

pub fn set_initialized(env: &Env) {
    env.storage().instance().set(&DataKey::Initialized, &true);
}

pub fn read_balance(env: &Env, user: &Address) -> i128 {
    env.storage().persistent().get(&DataKey::Balance(user.clone())).unwrap_or(0)
}

pub fn write_balance(env: &Env, user: &Address, amount: i128) {
    env.storage().persistent().set(&DataKey::Balance(user.clone()), &amount);
}

pub fn read_total_shares(env: &Env) -> i128 {
    env.storage().instance().get(&DataKey::TotalShares).unwrap_or(0)
}

pub fn write_total_shares(env: &Env, amount: i128) {
    env.storage().instance().set(&DataKey::TotalShares, &amount);
}

// TODO: Implement time-locked storage logic or reward rate scaling variables
