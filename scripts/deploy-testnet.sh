#!/usr/bin/env bash
set -euo pipefail

echo "==> Preparing Stellar Testnet Deployment for Fanout Agreement Contract..."

NETWORK="testnet"
RPC_URL="https://soroban-testnet.stellar.org"
NETWORK_PASSPHRASE="Test SDF Network ; September 2015"

echo "Building Wasm contract binary..."
cargo build --target wasm32-unknown-unknown --release

WASM_PATH="target/wasm32-unknown-unknown/release/fanout_agreement.wasm"

if command -v stellar &> /dev/null; then
    echo "Stellar CLI detected."
    echo "To deploy live to Testnet, execute:"
    echo "  stellar contract deploy --wasm $WASM_PATH --source <YOUR_IDENTITY> --network testnet"
else
    echo "Stellar CLI not found in PATH."
fi
