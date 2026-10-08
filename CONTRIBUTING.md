# Contributing to Fanout Smart Contracts

Thank you for your interest in contributing to Fanout!

## Code Discipline & Standards

1. **No Floating Point Operations**: Financial logic must strictly use basis points (`u32`) and base unit integers (`i128`).
2. **Soroban Security Requirements**: Enforce `require_auth()` on privileged calls and check arithmetic bounds.
3. **Git Workflow**:
   - Single-purpose logical commits.
   - Use conventional commit messages: `feat(contract): ...`, `fix(contract): ...`, `test(contract): ...`.
   - Never commit private keys, `.env` files, or build artifacts.

## Submitting Pull Requests

1. Fork the repository and create your feature branch.
2. Run `cargo test` to verify all tests pass.
3. Submit a Pull Request with acceptance criteria details.
