# Testing

Unit tests cover initialization, invalid allocation totals, deterministic rounding, token movement, replay protection, governance, and lifecycle status. Pull requests must also pass rustfmt, clippy with warnings denied, and a release WASM build.

Security-sensitive changes should add regression tests and document affected invariants.
