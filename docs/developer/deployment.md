# Deployment

Build the optimized WASM, install it with the Stellar CLI, deploy a contract instance, and invoke `initialize` exactly once with reviewed addresses and allocations. Record the network passphrase, WASM hash, contract ID, transaction hash, and source tag in a deployment manifest.

Testnet and mainnet identifiers must never be mixed. Verify the deployed WASM hash before publishing an address.
