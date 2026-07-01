//! # ValidTrust Network Vault Contract
//!
//! This contract implements a secure vault system on the Soroban blockchain.
//! It allows users to deposit supported tokens, accumulate rewards based on their
//! proportional share of the total pool, and withdraw their funds at any time.
//!
//! ## Features
//! - **Secure Deposits**: Uses the standard Token Interface for safe transfers.
//! - **Proportional Rewards**: Implements a weighted reward distribution model.
//! - **Event-Driven**: Emits detailed events for all core operations (Deposit, Withdraw, Reward).
//! - **Admin Controlled**: Certain administrative functions like initialization are restricted.
//!
//! ## Implementation Details
//! The contract uses the `soroban_sdk` for blockchain interaction and follows
//! the recommended storage patterns for efficiency and security.

#![no_std]

mod errors;
mod events;
mod storage;
mod test;

use soroban_sdk::{contract, contractimpl, Address, Env, token, log};
use errors::VaultError;

/// The main contract structure for the ValidTrust Vault.
#[contract]
pub struct ValidTrustVault;

#[contractimpl]
impl ValidTrustVault {
    /// # Initialize the Vault
    ///
    /// Sets up the initial state of the contract, including the administrator address
    /// and the token address that this vault will support.
    ///
    /// ### Arguments
    /// * `env` - The execution environment.
    /// * `admin` - The address that will have administrative privileges.
    /// * `token` - The address of the token (e.g., SAC) to be used in this vault.
    ///
    /// ### Returns
    /// * `Result<(), VaultError>` - Success or an error if already initialized.
    pub fn initialize(env: Env, admin: Address, token: Address) -> Result<(), VaultError> {
        log!(&env, "Initializing ValidTrust Vault with admin: {} and token: {}", admin, token);

        if storage::is_initialized(&env) {
            log!(&env, "Initialization failed: Vault is already initialized");
            return Err(VaultError::AlreadyInitialized);
        }

        storage::write_admin(&env, &admin);
        storage::write_token(&env, &token);
        storage::set_initialized(&env);

        log!(&env, "Vault successfully initialized");
        Ok(())
    }

    /// # Deposit Tokens
    ///
    /// Transfers tokens from the user's account into the vault and updates their share balance.
    ///
    /// ### Arguments
    /// * `env` - The execution environment.
    /// * `from` - The address of the user depositing tokens.
    /// * `amount` - The amount of tokens to deposit. Must be positive.
    ///
    /// ### Requirements
    /// - The caller must be authorized as `from`.
    /// - The contract must be initialized.
    /// - `amount` must be greater than zero.
    pub fn deposit(env: Env, from: Address, amount: i128) -> Result<(), VaultError> {
        from.require_auth();

        log!(&env, "Processing deposit request for user: {} amount: {}", from, amount);

        if !storage::is_initialized(&env) {
            log!(&env, "Deposit failed: Vault is not initialized");
            return Err(VaultError::NotInitialized);
        }
        if amount <= 0 {
            log!(&env, "Deposit failed: Invalid amount {}", amount);
            return Err(VaultError::InvalidAmount);
        }

        let token_addr = storage::read_token(&env);
        let token_client = token::Client::new(&env, &token_addr);

        // Perform the transfer from user to contract
        log!(&env, "Transferring tokens to vault...");
        token_client.transfer(&from, &env.current_contract_address(), &amount);

        // Update user-specific balance in storage
        let current_balance = storage::read_balance(&env, &from);
        let new_balance = current_balance.checked_add(amount)
            .ok_or(VaultError::InvalidAmount)?; // Use checked math for safety
        storage::write_balance(&env, &from, new_balance);

        // Update global total shares
        let current_total = storage::read_total_shares(&env);
        let new_total = current_total.checked_add(amount)
            .ok_or(VaultError::InvalidAmount)?;
        storage::write_total_shares(&env, new_total);

        // Emit standard deposit event
        events::emit_deposit(&env, &from, amount);

        log!(&env, "Deposit successful. New user balance: {}", new_balance);
        Ok(())
    }

    /// # Withdraw Tokens
    ///
    /// Withdraws tokens from the vault back to the user's account.
    ///
    /// ### Arguments
    /// * `env` - The execution environment.
    /// * `to` - The address receiving the tokens.
    /// * `amount` - The amount of tokens to withdraw.
    ///
    /// ### Requirements
    /// - The caller must be authorized as `to`.
    /// - The contract must be initialized.
    /// - User must have sufficient balance in the vault.
    pub fn withdraw(env: Env, to: Address, amount: i128) -> Result<(), VaultError> {
        to.require_auth();

        log!(&env, "Processing withdrawal request for user: {} amount: {}", to, amount);

        if !storage::is_initialized(&env) {
            return Err(VaultError::NotInitialized);
        }
        if amount <= 0 {
            return Err(VaultError::InvalidAmount);
        }

        let current_balance = storage::read_balance(&env, &to);
        if current_balance < amount {
            log!(&env, "Withdrawal failed: Insufficient balance. Available: {}", current_balance);
            return Err(VaultError::InsufficientBalance);
        }

        // Update user balance state
        let new_balance = current_balance - amount;
        storage::write_balance(&env, &to, new_balance);
        
        // Update global total shares
        let current_total = storage::read_total_shares(&env);
        storage::write_total_shares(&env, current_total - amount);

        // Execute the token transfer from contract to user
        let token_addr = storage::read_token(&env);
        let token_client = token::Client::new(&env, &token_addr);
        
        log!(&env, "Transferring tokens back to user...");
        token_client.transfer(&env.current_contract_address(), &to, &amount);

        // Emit standard withdrawal event
        events::emit_withdraw(&env, &to, amount);

        log!(&env, "Withdrawal successful. Remaining balance: {}", new_balance);
        Ok(())
    }

    /// # Query User Balance
    ///
    /// Returns the current amount of tokens a user has deposited in the vault.
    ///
    /// ### Arguments
    /// * `env` - The execution environment.
    /// * `user` - The address to query.
    pub fn balance(env: Env, user: Address) -> i128 {
        storage::read_balance(&env, &user)
    }

    /// # Distribute Rewards
    ///
    /// Distributes rewards from a pool to a specific user based on their share of the vault.
    /// This implementation currently compounds the rewards directly into the user's balance.
    ///
    /// ### Arguments
    /// * `env` - The execution environment.
    /// * `user` - The address to receive rewards.
    /// * `total_reward_pool` - The total amount available in the pool for distribution.
    ///
    /// ### Requirements
    /// - The caller must be authorized as the contract admin.
    /// - Total shares in the vault must be greater than zero.
    pub fn distribute_reward(env: Env, user: Address, total_reward_pool: i128) -> Result<(), VaultError> {
        let admin = storage::read_admin(&env);
        admin.require_auth(); 

        log!(&env, "Admin initiating reward distribution for user: {}", user);

        let user_bal = storage::read_balance(&env, &user);
        let total_shares = storage::read_total_shares(&env);

        if total_shares == 0 || user_bal == 0 {
            log!(&env, "Reward distribution failed: No active shares for calculation");
            return Err(VaultError::InvalidAmount);
        }

        // Proportional reward calculation: (user_bal / total_shares) * total_reward_pool
        // We use multiplication before division to maintain precision in integer arithmetic.
        let user_reward = match user_bal.checked_mul(total_reward_pool) {
            Some(product) => product / total_shares,
            None => {
                log!(&env, "Overflow error during reward calculation");
                return Err(VaultError::InvalidAmount);
            }
        };

        if user_reward > 0 {
            log!(&env, "Distributing reward: {} to user: {}", user_reward, user);
            
            // Credit reward to user balance (compounding)
            storage::write_balance(&env, &user, user_bal + user_reward);
            storage::write_total_shares(&env, total_shares + user_reward);

            // Emit standard reward event
            events::emit_reward(&env, &user, user_reward);
        } else {
            log!(&env, "Reward distribution skipped: Calculated reward is zero");
        }

        Ok(())
    }
    
    // TODO: Add upgradeable contract pattern (e.g. `update_contract_code`)
    // TODO: Implement time-locked withdrawals or unbonding periods
    // TODO: Add support for multi-token vaults or synthetic assets
}
