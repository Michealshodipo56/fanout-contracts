#!/usr/bin/env bash
set -euo pipefail

: "${STELLAR_ACCOUNT:?Set STELLAR_ACCOUNT to a funded Stellar CLI identity}"
: "${FANOUT_NAME:?Set FANOUT_NAME}"
: "${FANOUT_ASSET_CONTRACT:?Set FANOUT_ASSET_CONTRACT to a Testnet SAC contract ID}"
: "${FANOUT_BENEFICIARIES_JSON:?Set FANOUT_BENEFICIARIES_JSON to the CLI JSON array}"
: "${FANOUT_REQUIRED_APPROVALS:?Set FANOUT_REQUIRED_APPROVALS}"

stellar contract build
wasm_path="target/wasm32v1-none/release/fanout_agreement.wasm"
sha256sum "$wasm_path"
contract_id=$(stellar contract deploy --wasm "$wasm_path" --source "$STELLAR_ACCOUNT" --network testnet)
creator=$(stellar keys address "$STELLAR_ACCOUNT")
stellar contract invoke --id "$contract_id" --source "$STELLAR_ACCOUNT" --network testnet -- initialize \
  --creator "$creator" --name "\"$FANOUT_NAME\"" --accepted_asset "$FANOUT_ASSET_CONTRACT" \
  --beneficiaries "$FANOUT_BENEFICIARIES_JSON" --required_approvals "$FANOUT_REQUIRED_APPROVALS"
printf '%s\n' "Contract ID: $contract_id" "Explorer: https://stellar.expert/explorer/testnet/contract/$contract_id"
