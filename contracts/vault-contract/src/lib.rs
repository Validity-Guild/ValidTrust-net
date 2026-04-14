#![no_std]

mod errors;
mod events;
mod storage;
mod test;

use soroban_sdk::{contract, contractimpl, Address, Env, token};
use errors::VaultError;

#[contract]
pub struct ValidityVault;

#[contractimpl]
impl ValidityVault {
    /// Initialize the vault with an admin and the supported token.
    pub fn initialize(env: Env, admin: Address, token: Address) -> Result<(), VaultError> {
        if storage::is_initialized(&env) {
            return Err(VaultError::AlreadyInitialized);
        }

        storage::write_admin(&env, &admin);
        storage::write_token(&env, &token);
        storage::set_initialized(&env);

        Ok(())
    }

    /// Deposit tokens into the vault.
    pub fn deposit(env: Env, from: Address, amount: i128) -> Result<(), VaultError> {
        from.require_auth();

        if !storage::is_initialized(&env) {
            return Err(VaultError::NotInitialized);
        }
        if amount <= 0 {
            return Err(VaultError::InvalidAmount);
        }

        let token_addr = storage::read_token(&env);
        let token_client = token::Client::new(&env, &token_addr);

        // Transfer tokens from the user to this contract
        token_client.transfer(&from, &env.current_contract_address(), &amount);

        // Update state
        let current_balance = storage::read_balance(&env, &from);
        storage::write_balance(&env, &from, current_balance + amount);

        let current_total = storage::read_total_shares(&env);
        storage::write_total_shares(&env, current_total + amount);

        // Emit event
        events::emit_deposit(&env, &from, amount);

        Ok(())
    }

    /// Withdraw tokens from the vault.
    pub fn withdraw(env: Env, to: Address, amount: i128) -> Result<(), VaultError> {
        to.require_auth();

        if !storage::is_initialized(&env) {
            return Err(VaultError::NotInitialized);
        }
        if amount <= 0 {
            return Err(VaultError::InvalidAmount);
        }

        let current_balance = storage::read_balance(&env, &to);
        if current_balance < amount {
            return Err(VaultError::InsufficientBalance);
        }

        // Update state
        storage::write_balance(&env, &to, current_balance - amount);
        
        let current_total = storage::read_total_shares(&env);
        storage::write_total_shares(&env, current_total - amount);

        // Transfer tokens back to user
        let token_addr = storage::read_token(&env);
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&env.current_contract_address(), &to, &amount);

        // Emit event
        events::emit_withdraw(&env, &to, amount);

        Ok(())
    }

    /// Get user's current vault balance.
    pub fn balance(env: Env, user: Address) -> i128 {
        storage::read_balance(&env, &user)
    }

    /// Distribute rewards to a user based on their balance weight.
    /// In a production scenario, this might be called by the admin, or baked into withdrawals.
    /// This is a simplified placeholder logic for contributors to expand.
    pub fn distribute_reward(env: Env, user: Address, total_reward_pool: i128) -> Result<(), VaultError> {
        let admin = storage::read_admin(&env);
        admin.require_auth(); // Only admin can trigger reward distribution in this simple model

        let user_bal = storage::read_balance(&env, &user);
        let total_shares = storage::read_total_shares(&env);

        if total_shares == 0 || user_bal == 0 {
            return Err(VaultError::InvalidAmount); // No shares to calculate reward
        }

        // Proportional reward calculation (safe math would be necessary for prod)
        // user_reward = (user_bal * total_reward_pool) / total_shares
        let user_reward = (user_bal * total_reward_pool) / total_shares;

        if user_reward > 0 {
            // In a real scenario, the reward might be minted or transferred from a reserve pool
            // Here, we just credit it to their balance as an example of compounding.
            // TODO: Implement reward optimization logic and secure scaling
            storage::write_balance(&env, &user, user_bal + user_reward);
            storage::write_total_shares(&env, total_shares + user_reward);

            events::emit_reward(&env, &user, user_reward);
        }

        Ok(())
    }
    
    // TODO: Add upgradeable contract pattern (e.g. `update_contract_code`)
    // TODO: Implement time-locked withdrawals or unbonding periods
}
