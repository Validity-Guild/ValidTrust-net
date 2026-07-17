<div align="center">

<img src="logo/Valid%20net%20logo.png" alt="ValidTrust Network Logo" width="120" height="120" />

# ValidTrust Network

**An open-source, modular vault and reward distribution protocol built on Stellar Soroban.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-Soroban-orange?logo=rust)](https://www.rust-lang.org/)
[![Stellar](https://img.shields.io/badge/Stellar-Soroban-7C3AED?logo=stellar)](https://soroban.stellar.org/)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

</div>

---

## Overview

ValidTrust Network is the core smart contract protocol powering the ValidTrust ecosystem. It implements a non-custodial vault and reward distribution system on the [Stellar](https://stellar.org/) blockchain, leveraging [Soroban](https://soroban.stellar.org/) smart contracts written in **Rust**.

The protocol is designed with modularity and open-source contribution in mind. Maintainers regularly post GitHub Issues covering gas optimisations, security improvements, and feature extensions — making this an ideal repository for Soroban developers to learn, contribute, and build.

---

## Table of Contents

- [How It Works](#how-it-works)
- [Architecture](#architecture)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Building & Testing](#building--testing)
- [Deployment](#deployment)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)

---

## How It Works

The `ValidTrustVault` contract is the centrepiece of the protocol. It allows users to:

| Action | Description |
|---|---|
| **Deposit** | Lock Stellar assets (tokens) into the vault |
| **Track Balances** | Balances are recorded securely on-chain using Soroban's persistent storage |
| **Withdraw** | Retrieve deposited funds at any time without a lock-up period |
| **Claim Rewards** | Receive proportional rewards from a shared distribution pool based on vault share |

All actions emit structured on-chain events that can be consumed by off-chain indexers and the [ValidTrust Dashboard](https://github.com/validtrust-network/validtrust-dashboard).

---

## Architecture

The contract is written in **Rust** and compiled to **WebAssembly** for deployment on Soroban. It is structured into focused modules:

| Module | Description |
|---|---|
| `src/lib.rs` | Main contract interface — deposit, withdraw, and reward claim entrypoints |
| `src/storage.rs` | Manages `Persistent` and `Instance` storage for balances and state |
| `src/events.rs` | Defines typed events for on-chain indexing |
| `src/errors.rs` | Custom contract error types for predictable failure handling |

For a deeper dive, see the [Architecture Overview](docs/architecture.md) and the [Contract Specification](docs/contract-spec.md).

---

## Getting Started

### Prerequisites

| Tool | Purpose | Install |
|---|---|---|
| [Rust](https://www.rust-lang.org/tools/install) + `wasm32` target | Compile the smart contract | `rustup target add wasm32-unknown-unknown` |
| [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup) | Deploy and invoke contracts locally | See Soroban docs |
| [Node.js](https://nodejs.org/) v18+ | Run deployment and test scripts | nodejs.org |

### Installation

Clone the repository and install Node.js dependencies:

```bash
git clone https://github.com/Validity-Guild/ValidTrust-net.git
cd ValidTrust-net
npm install
```

Configure your environment by creating a `.env` file based on `.env.example`:

```env
SOROBAN_RPC_URL=https://soroban-testnet.stellar.org
NETWORK_PASSPHRASE=Test SDF Network ; September 2015
SOURCE_SECRET=YOUR_SECRET_KEY_HERE
VAULT_CONTRACT_ID=
TOKEN_CONTRACT_ID=
ADMIN_ADDRESS=
```

---

## Building & Testing

### Build the Contract

Compile the Rust smart contract to WebAssembly:

```bash
npm run build:contract
```

The compiled `.wasm` binary will be output to `contracts/vault-contract/target/wasm32-unknown-unknown/release/`.

### Run Rust Unit Tests

Execute the Soroban native test suite:

```bash
npm run test:contract
```

### Run TypeScript Integration Tests

Execute the Jest tests covering deployment and contract interactions:

```bash
npm run test:ts
```

---

## Deployment

Deploy the compiled contract to the Stellar Testnet (or a local sandbox):

```bash
# Deploy the contract and obtain a Contract ID
npm run deploy

# Initialise contract state with default parameters
npm run init
```

Ensure your `.env` file is configured with a funded Testnet account before deploying. You can fund a Testnet account using [Stellar Laboratory](https://laboratory.stellar.org/#account-creator?network=test).

---

## Project Structure

```
validtrust-network/
├── contracts/
│   └── vault-contract/         # Rust/Soroban smart contract source
│       ├── src/
│       │   ├── lib.rs           # Contract entrypoints
│       │   ├── storage.rs       # On-chain state management
│       │   ├── events.rs        # Structured event definitions
│       │   └── errors.rs        # Custom error types
│       └── Cargo.toml
├── scripts/
│   ├── deploy.ts               # Contract deployment script
│   └── initialize.ts           # Contract initialization script
├── tests/                      # TypeScript integration tests
├── docs/                       # Architecture and specification docs
└── README.md
```

---

---

## Contributing

We actively welcome contributions from developers of all experience levels. Whether you're a Soroban beginner or an experienced Rust developer, there is a meaningful task for you.

Please read our [Contributing Guide](docs/contributing-guide.md) to understand our workflow before opening a Pull Request.

**Priority contribution areas:**
- Gas and storage optimisations in `src/lib.rs`
- Improved reward distribution algorithms
- Additional contract modules (Governance, Staking)
- Expanded test coverage in `src/test.rs`
- Security reviews and audit preparation

---

## License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.
