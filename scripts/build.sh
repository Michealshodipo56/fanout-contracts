#!/usr/bin/env bash
set -euo pipefail

echo "==> Building Fanout Soroban Smart Contracts..."
cargo build --target wasm32-unknown-unknown --release

WASM_FILE="target/wasm32-unknown-unknown/release/fanout_agreement.wasm"

if [ -f "$WASM_FILE" ]; then
    SIZE=$(ls -lh "$WASM_FILE" | awk '{print $5}')
    echo " SUCCESS: Built $WASM_FILE ($SIZE)"
else
    echo " ERROR: Wasm build output missing!"
    exit 1
fi
