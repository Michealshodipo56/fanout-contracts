# Fanout Smart Contracts

![Stellar](https://img.shields.io/badge/Blockchain-Stellar-blue)
![Soroban](https://img.shields.io/badge/Smart%20Contracts-Soroban%20v22-purple)
![CI](https://github.com/fanout-web/fanout-contracts/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/License-MIT-green)

> One payment. Everyone gets their share.

Fanout is a Soroban protocol for deterministic, atomic revenue sharing on Stellar. Each deployed agreement defines one accepted token, beneficiary addresses, exact basis-point allocations, and a participant-governed update threshold.

This repository is the on-chain source of truth. The web dashboard, REST API, TypeScript SDK, and indexer live in [`fanout-app`](https://github.com/fanout-web/fanout-app).

> **Release status:** `v0.1.0` is a Stellar Testnet submission release. Mainnet use requires an independent security review, verified artifact, and published deployment manifest.

## Core Guarantees

- **Deterministic allocation:** shares total exactly 10,000 basis points.
- **No lost base units:** largest-remainder allocation conserves the payment amount.
- **Atomic settlement:** all beneficiary transfers succeed or the invocation fails.
- **Payer authorization:** only the supplying account authorizes token spending.
- **Replay protection:** a payment reference executes once per agreement.
- **Governed changes:** authorized participants approve beneficiary updates.
- **Version safety:** obsolete proposals cannot overwrite current configuration.
- **Terminal closure:** a closed agreement cannot reopen.
- **Storage durability:** contract access extends instance TTL.
- **Indexable activity:** structured events cover payments and governance.

## Contract Lifecycle

1. Deploy a new agreement instance.
2. Initialize it once with creator, name, token, beneficiaries, allocations, and approval threshold.
3. Accept payments through `distribute` while active.
4. Propose, approve, and execute beneficiary updates.
5. Suspend temporarily or close permanently.

Each instance represents one agreement, isolating replay keys, governance, lifecycle, and accounting between teams.

## Public Interface

### `initialize`

Creates the agreement once and requires creator authorization. Beneficiaries must be unique and within the limit, allocations must total 10,000, and quorum cannot exceed the beneficiary count.

### `distribute`

Accepts payer, positive token amount in base units, and unique symbol reference. It calculates exact allocations, performs all token transfers, records the reference, updates totals, and emits a payment event.

### Governance

`propose_update` creates a complete replacement allocation and automatically records the proposer approval. `approve_proposal` records one approval per authorized participant. `execute_proposal` applies a quorum-approved, current-version proposal and increments the configuration version.

### Lifecycle and reads

`set_status` lets the creator suspend, reactivate, or permanently close the agreement. `get_config` returns current configuration and `get_proposal` returns a proposal by ID.

## Allocation Algorithm

```text
base payout = payment amount × allocation basis points ÷ 10,000
remainder   = payment amount × allocation basis points mod 10,000
```

Unallocated base units are assigned in descending remainder order. Beneficiary order makes ties deterministic. A final conservation check rejects the transaction unless payouts sum exactly to the payment amount.

## Repository Structure

```text
fanout-contracts/
├── contracts/agreement/
│   ├── src/
│   │   ├── lib.rs          # Public interface
│   │   ├── payment.rs      # Allocation math and transfers
│   │   ├── governance.rs   # Validation and authorization
│   │   ├── storage.rs      # State and TTL extension
│   │   ├── types.rs        # Data types and limits
│   │   ├── errors.rs       # Stable error codes
│   │   ├── events.rs       # Soroban events
│   │   └── test.rs         # Contract and invariant tests
│   └── Cargo.toml
├── scripts/                # Build, test, and Testnet deployment
├── .github/workflows/      # Required CI
├── Cargo.toml
├── Cargo.lock
└── SECURITY.md
```

## Getting Started

### Prerequisites

- Rust stable
- Stellar CLI
- `wasm32v1-none` Rust target
- A funded Stellar Testnet identity for deployment

```bash
rustup target add wasm32v1-none
git clone https://github.com/fanout-web/fanout-contracts.git
cd fanout-contracts
```

### Test and lint

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
```

Tests cover initialization, allocation validation, exact rounding, token movement, replay protection, governance, suspension, closure, and terminal lifecycle behavior.

### Build optimized WASM

```bash
cargo build --target wasm32v1-none --release
```

Record the release artifact's SHA-256 checksum before deployment.

## Deployment Outline

1. Build and checksum WASM from a source tag.
2. Install it on the intended Stellar network.
3. Deploy a contract instance from the installed hash.
4. Review every address and allocation before initialization.
5. Publish network, source tag, commit, checksum, WASM ID, contract ID, and transaction hashes.
6. Add the contract ID to the indexer allowlist.

Never mix Testnet and mainnet identifiers, endpoints, or passphrases.

## Events

Events cover agreement creation, payment distribution, proposal creation, approval and execution, and status changes. Indexers should query allowlisted contract IDs, persist a ledger cursor, and deduplicate by transaction hash plus event position.

## Security Invariants

1. Allocations sum to exactly 10,000 basis points.
2. Payouts sum exactly to the payment amount.
3. The payer authorizes spending.
4. A payment reference cannot execute twice.
5. Only authorized participants influence governance.
6. Old-version proposals cannot modify current state.
7. Only the creator changes lifecycle status.
8. Closure is permanent.
9. Accounting and counters fail safely on overflow.

Automated tests are not an independent audit. Report vulnerabilities privately according to [SECURITY.md](SECURITY.md).

## Compatibility and Releases

Releases use semantic version tags. Storage, event, authorization, or interface changes must document compatibility and migration impact. See the [latest release](https://github.com/fanout-web/fanout-contracts/releases/latest).

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md), start from an open issue, and add regression tests. Pull requests must explain authorization, storage, event, and compatibility impact.

## License

Fanout is available under the [MIT License](LICENSE).
