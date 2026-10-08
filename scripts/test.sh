#!/usr/bin/env bash
set -euo pipefail

echo "==> Running Fanout Smart Contract Unit & Invariant Tests..."
cargo test -- --nocapture
echo " SUCCESS: All contract tests passed!"
