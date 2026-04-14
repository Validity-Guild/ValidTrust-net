# Validity Network

Validity Network is an open-source, smart-contract based vault and reward distribution protocol built on the Stellar network using Soroban. It is designed to be highly modular, making it a perfect repository for open-source contributors to learn, build, and optimize smart contracts.

## Overview

The Vault contract (`ValidityVault`) allows users to:
- **Deposit** Stellar assets (tokens).
- **Track balances** securely on-chain.
- **Withdraw** funds.
- **Receive proportional rewards** from a distribution pool.

This repository serves as both a production-ready template and a learning hub for Soroban developers. Maintainers regularly post GitHub issues covering optimizations, security, and feature extensions.

## Architecture

The smart contract is written in Rust and structured into specific modules:
- `src/lib.rs`: The main contract interface containing deposit/withdraw logic.
- `src/storage.rs`: Handles the `Persistent` and `Instance` storage interactions.
- `src/events.rs`: Defines structured events for indexers to consume.
- `src/errors.rs`: Defines custom contract errors.

For more details, see [Architecture](docs/architecture.md) and the [Contract Specification](docs/contract-spec.md).

## Setup Instructions

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (with `wasm32-unknown-unknown` target installed)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)
- Node.js (for running deployment scripts)

### Installation
Clone the repository and install Node dependencies:
```bash
git clone https://github.com/validity-network/validity-network.git
cd validity-network
npm install
```

## Building and Testing

### Build the Contract
Compile the Rust smart contract into WebAssembly:
```bash
npm run build:contract
```

### Run Unit Tests
Run the Soroban Rust test suite to verify the logic:
```bash
npm run test:contract
```

### Run TypeScript Integration Tests
Run the Jest tests covering deployment and execution:
```bash
npm run test:ts
```

## Deployment

To deploy the contract to a local sandbox or the Stellar Testnet, use the provided TypeScript script. Ensure you have your `.env` file configured.

```bash
npm run deploy
npm run init
```

## Contributing

We are actively seeking contributors! Whether you are optimizing gas usage, improving reward distribution logic, or adding new tests, your help is welcome.

Please read our [Contributing Guide](docs/contributing-guide.md) to understand our workflow and review the open TODOs in the codebase.

## License

MIT License
