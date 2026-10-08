# Local Setup

Install stable Rust, the `wasm32v1-none` target, and the Stellar CLI. Then run:

```bash
cargo test --workspace --all-targets
cargo build --target wasm32v1-none --release
```
