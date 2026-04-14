# Validity Network Architecture

The Validity Network smart contract is designed with a modular approach to separate concerns and make testing and contributing straightforward.

## Modules

- **lib.rs**: The main contract entry point. It contains the logic for `initialize`, `deposit`, `withdraw`, `balance`, and `distribute_reward`. It orchestrates interactions between the storage, events, and token layers.
- **storage.rs**: Handles all interactions with Soroban's storage layer. It uses `Persistent` storage for user balances (which survive upgrades) and `Instance` storage for configuration (like the admin address and token).
- **events.rs**: Encapsulates the emission of all contract events (`deposit`, `withdraw`, `reward`). This helps indexers and frontend dashboards easily track contract state changes over time.
- **errors.rs**: Contains custom `contracterror` definitions for clear failure states (e.g. `InsufficientBalance`, `NotInitialized`).

## Storage Design

- **Admin**: Stored at `DataKey::Admin`.
- **Token**: The underlying Stellar asset contract address. Stored at `DataKey::Token`.
- **Balances**: Stored persistently using a composite key `DataKey::Balance(Address)`.
- **Total Shares**: Tracks the global pool size for reward distributions. Stored at `DataKey::TotalShares`.

## Event System

Events are structured to be easily parsed by indexers:
- Topic 1: Action Name (e.g., `symbol_short!("deposit")`)
- Topic 2: User Address
- Data: Amount (i128)
