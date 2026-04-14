# Contributing to Validity Network

Thank you for your interest in contributing to the Validity Network! We are excited to build this open-source Web3 protocol together.

## Getting Started

1. **Fork the repository** on GitHub.
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/validity-network.git
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

## Submitting a Pull Request

1. Commit your changes with descriptive messages.
2. Push your branch to your fork on GitHub.
3. Open a Pull Request against the `main` branch.
4. Ensure your PR description clearly explains the changes and references any related issues.
