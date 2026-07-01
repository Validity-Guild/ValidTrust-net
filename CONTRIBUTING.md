# Contributing to ValidTrust Network

Thank you for your interest in contributing to the ValidTrust Network! We are excited to build this open-source Web3 protocol together.

## Getting Started

1. **Fork the repository** on GitHub.
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/validtrust-network.git
   ```
3. **Install dependencies**:
   ```bash
   npm install
   ```
4. **Create a new branch** for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Workflow

- Read the `docs/contributing-guide.md` for specific technical tasks and open TODOs.
- Write Rust tests for your new features or bug fixes in `src/test.rs`.
- Run tests before pushing:
  ```bash
  npm run test:contract
  npm run test:ts
  ```

## Drips and Drips Wave Contribution

This project participates in [Drips](https://drips.network) and [Drips Wave](https://docs.drips.network/wave/).

### Finding Issues to Work On
- Look for issues labeled with:
  - `good first issue` - Great for first-time contributors
  - `help wanted` - Issues that need assistance
  - `drips-wave` - Issues eligible for Drips Wave
  - `bounty-ready` - Issues with potential bounties
  - `smart-contract` - Issues related to smart contracts
  - `rust` - Rust-specific issues

### Drips Wave Tasks
Issues labeled `drips-wave` are eligible for Drips Wave contribution cycles. These issues typically have:
- Clear acceptance criteria
- Estimated difficulty level
- Expected files or modules to modify
- Testing requirements

## Priority Contribution Areas

- [ ] Gas and storage optimizations in `src/lib.rs`
- [ ] Improved reward distribution algorithms
- [ ] Additional contract modules (Governance, Staking)
- [ ] Expanded test coverage in `src/test.rs`
- [ ] Security reviews and audit preparation

## Submitting a Pull Request

1. Commit your changes with descriptive messages.
2. Push your branch to your fork on GitHub.
3. Open a Pull Request against the `main` branch.
4. Ensure your PR description clearly explains the changes and references any related issues.
