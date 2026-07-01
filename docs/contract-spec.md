# Contract Specification

## `ValidTrustVault`

The Vault is a smart contract designed to hold tokens on behalf of users and distribute proportional rewards.

### Initialization

```rust
pub fn initialize(env: Env, admin: Address, token: Address) -> Result<(), VaultError>
```
Sets the admin and the single supported token for the vault. Can only be called once.

### Core Operations

#### `deposit`
```rust
pub fn deposit(env: Env, from: Address, amount: i128) -> Result<(), VaultError>
```
Transfers `amount` of tokens from the `from` address to the contract. The `from` address must sign the transaction. Increases the user's tracked balance and the total shares.

#### `withdraw`
```rust
pub fn withdraw(env: Env, to: Address, amount: i128) -> Result<(), VaultError>
```
Transfers `amount` of tokens from the contract back to the `to` address. The `to` address must sign. Fails if the requested amount exceeds the user's recorded balance.

#### `balance`
```rust
pub fn balance(env: Env, user: Address) -> i128
```
Returns the current token balance tracked by the vault for the given `user`.

### Reward System

#### `distribute_reward`
```rust
pub fn distribute_reward(env: Env, user: Address, total_reward_pool: i128) -> Result<(), VaultError>
```
(Admin only in this simple implementation). Calculates a user's share of the `total_reward_pool` based on their balance versus the `total_shares`, and credits their balance with the reward amount.
