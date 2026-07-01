# Drips Readiness - ValidTrust Network

This document outlines the Drips readiness for the ValidTrust Network repository.

## Repository Purpose

ValidTrust Network is the core smart contract protocol powering the ValidTrust ecosystem. It implements a non-custodial vault and reward distribution system on Stellar Soroban.

## Repository Claiming

**Status**: Not yet claimed

### Maintainer Action Items
1. Visit [Drips](https://drips.network) to claim this repository
2. If required, add a `FUNDING.json` file with maintainer-approved ownership information
3. Consider using a multisig/Safe for team ownership
4. Review and approve wallet addresses before committing

## Upstream Dependencies

Key dependencies that may be relevant for Drips funding:
- `soroban-sdk` (Rust) - Stellar Soroban smart contract SDK
- `@stellar/stellar-sdk` (TypeScript) - Stellar blockchain SDK
- `dotenv` - Environment variable management

### Suggested Dependency Review Process
- Review each dependency's license and maintenance status
- Consider whether to include dependencies in Drips split configuration
- Document split decisions (do not commit real split percentages without maintainer approval)

## Drip List Inclusion

**Suggestion**: This repository should be considered for inclusion in relevant Drip Lists as it's the core protocol component of the ValidTrust ecosystem.

## Incoming Fund Splitting

**Suggestion (non-binding)**: Consider splitting incoming funds to upstream dependencies that are critical to the protocol's functionality. Maintainers should review and approve any split configuration.

## Drips Wave Readiness

This repository is ready for Drips Wave participation:
- Issue templates are available including Drips Wave-specific templates
- CONTRIBUTING.md includes guidance on Drips Wave tasks
- Labels are defined for issue categorization
- Clear contribution guidelines are in place

## Label Documentation

See [docs/labels.md](labels.md) for label documentation.
