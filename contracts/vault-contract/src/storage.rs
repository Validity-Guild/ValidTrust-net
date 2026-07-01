//! # Vault Storage Module
//!
//! This module manages the persistent and instance storage for the ValidTrust Vault contract.
//! It defines the keys used for storage and provides helper functions to read and write
//! contract state safely.
//!
//! ## Storage Types
//! - **Instance Storage**: Used for global contract state that is frequently accessed
//!   (e.g., Admin address, Token address, Total Shares).
//! - **Persistent Storage**: Used for user-specific data that might be less frequently
//!   accessed but needs to persist indefinitely (e.g., User Balances).

#![no_std]

use soroban_sdk::{contracttype, Address, Env};

/// Keys used to identify different data points in the contract's storage.
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    /// The address of the token supported by this vault.
    Token,
    /// The address of the contract administrator.
    Admin,
    /// A boolean flag indicating if the contract has been initialized.
    Initialized,
    /// Maps a user address to their current token balance in the vault.
    Balance(Address),
    /// The total amount of tokens currently locked in the vault.
    TotalShares,
}

/// Checks if the administrator address has been set in storage.
pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

/// Retrieves the administrator address from instance storage.
///
/// ### Panics
/// Panics if the admin has not been set.
pub fn read_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).expect("Admin not set")
}

/// Writes the administrator address to instance storage.
pub fn write_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

/// Checks if the supported token address has been set in storage.
pub fn has_token(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Token)
}

/// Retrieves the supported token address from instance storage.
///
/// ### Panics
/// Panics if the token address has not been set.
pub fn read_token(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Token).expect("Token address not set")
}

/// Writes the supported token address to instance storage.
pub fn write_token(env: &Env, token: &Address) {
    env.storage().instance().set(&DataKey::Token, token);
}

/// Checks if the contract has been initialized.
pub fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Initialized)
}

/// Marks the contract as initialized in instance storage.
pub fn set_initialized(env: &Env) {
    env.storage().instance().set(&DataKey::Initialized, &true);
}

/// Reads the token balance for a specific user from persistent storage.
///
/// ### Returns
/// Returns the balance as an `i128`. Defaults to `0` if no balance is found.
pub fn read_balance(env: &Env, user: &Address) -> i128 {
    env.storage().persistent().get(&DataKey::Balance(user.clone())).unwrap_or(0)
}

/// Writes the token balance for a specific user to persistent storage.
pub fn write_balance(env: &Env, user: &Address, amount: i128) {
    env.storage().persistent().set(&DataKey::Balance(user.clone()), &amount);
}

/// Reads the total amount of shares (tokens) in the vault from instance storage.
///
/// ### Returns
/// Returns the total shares as an `i128`. Defaults to `0` if not set.
pub fn read_total_shares(env: &Env) -> i128 {
    env.storage().instance().get(&DataKey::TotalShares).unwrap_or(0)
}

/// Writes the total amount of shares (tokens) in the vault to instance storage.
pub fn write_total_shares(env: &Env, amount: i128) {
    env.storage().instance().set(&DataKey::TotalShares, &amount);
}

// TODO: Implement time-locked storage logic or reward rate scaling variables
// TODO: Add support for temporary 'hot' storage for high-frequency updates
// TODO: Implement storage archival patterns for old user data
