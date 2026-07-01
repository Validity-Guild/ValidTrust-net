# Contributing to ValidTrust Network

We welcome contributions to the ValidTrust Network! This guide will help you get started with the codebase and our development workflow.

## Prerequisites

1. **Rust**: Install Rust and the `wasm32-unknown-unknown` target.
2. **Soroban CLI**: Install the Soroban CLI to build and test contracts.
3. **Node.js**: Install Node and npm/yarn for deployment scripts.

## Setting Up

1. Fork and clone the repository.
2. Install Node dependencies: `npm install`
3. Build the contract: `npm run build:contract`
4. Run tests: `npm run test:contract`

## Contribution Areas (TODOs)

There are several areas where contributors can make a significant impact. Look for `TODO` comments in the codebase:

- **Reward Optimization**: The current reward calculation is simple. We need an advanced mechanism (like the MasterChef pattern) to scale rewards accurately.
- **Gas Efficiency**: Optimize storage reads/writes in `lib.rs` to minimize resource consumption on Stellar.
- **Additional Security Checks**: Add more robust validation, time-locks, or emergency pause functions.
- **Governance Functionality**: Implement a way for multiple admins or a DAO to control parameters.
- **Upgradeable Contract Pattern**: Add a function to safely upgrade the contract logic (`update_contract_code`).

## Submitting Changes

1. Create a feature branch from `main`.
2. Write tests for any new contract logic using `soroban_sdk::testutils`.
3. Ensure `cargo fmt` and `cargo clippy` pass cleanly.
4. Submit a Pull Request with a clear description of the problem and the proposed solution.
