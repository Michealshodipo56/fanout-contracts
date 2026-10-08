# Fanout Smart Contracts (`fanout-contracts`)

![Stellar](https://img.shields.io/badge/Blockchain-Stellar-blue)
![Soroban](https://img.shields.io/badge/Smart%20Contracts-Soroban%20v22-purple)
![License](https://img.shields.io/badge/License-MIT-green)
![Build Status](https://img.shields.io/badge/Tests-Passing-brightgreen)

> **One payment. Everyone gets their share.**

`fanout-contracts` houses the official Soroban smart contract suite powering the **Fanout** platform on Stellar. It enables programmable, deterministic revenue sharing and automatic distribution of incoming payments across multiple beneficiary wallets.

---

## 🌟 Architecture & Features

- **Deterministic Revenue Sharing**: Allocations are computed using integer-based basis points ($10,000\text{ BPS} = 100.00\%$) and a **Largest Remainder Algorithm** ensuring zero token truncation or loss.
- **Atomic Multi-Transfer**: Transfers funds directly to all recipient Stellar wallets in a single transaction via SEP-41 token interfaces.
- **Replay Protection**: Guarantees payment reference uniqueness per agreement to prevent duplicate or accidental distributions.
- **Proposal-Based Governance**: Protective unanimous or quorum-based agreement updates. Creators cannot unilaterally redirect funds without beneficiary consent.
- **Storage Management**: Leverages Soroban Instance Storage with automatic TTL extensions.
- **Structured Soroban Events**: Publishes detailed on-chain events for indexer tracking.

---

## 📂 Repository Structure

```
fanout-contracts/
├── contracts/
│   └── agreement/
│       ├── src/
│       │   ├── lib.rs          # Contract entry points & initialization
│       │   ├── payment.rs      # Basis-points math & SEP-41 transfers
│       │   ├── governance.rs   # Proposal creation & approval logic
│       │   ├── storage.rs      # Storage keys & TTL management
│       │   ├── types.rs        # Data structures (Config, Proposal, Beneficiary)
│       │   ├── errors.rs       # ContractError enum definitions
│       │   ├── events.rs       # Soroban event publishers
│       │   └── test.rs         # Unit & financial invariant tests
│       └── Cargo.toml
├── scripts/
│   ├── build.sh                # Wasm release builder
│   ├── test.sh                 # Unit test execution script
│   └── deploy-testnet.sh       # Testnet deployment helper
├── Cargo.toml
├── CONTRIBUTING.md
├── SECURITY.md
└── LICENSE
```

---

## 🚀 Quick Start & Testing

### Prerequisites

- [Rust & Cargo](https://rustup.rs/) `v1.97+`
- [Stellar CLI](https://developers.stellar.org/docs/tools/cli) `v28.0+`
- Target `wasm32-unknown-unknown` (`rustup target add wasm32-unknown-unknown`)

### Building Contracts

```bash
cargo build --target wasm32-unknown-unknown --release
# Or run script:
./scripts/build.sh
```

### Running Tests

```bash
cargo test -- --nocapture
# Or run script:
./scripts/test.sh
```

---

## 🔐 Security Invariants

1. **Allocations Invariant**: $\sum \text{beneficiary\_bps} == 10,000$.
2. **Payout Invariant**: $\sum \text{payouts} == \text{payment\_amount}$.
3. **Replay Protection**: Duplicate `payment_ref` symbols are rejected.
4. **Auth Invariant**: `payer.require_auth()` is enforced on every distribution; proposals require beneficiary authorization.

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).
