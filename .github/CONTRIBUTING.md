# Contributing To cardano

`cardano` is security-sensitive Cardano protocol infrastructure. Contributions
must keep the workspace small, explicit, tested, and honest about what is
stable.

## License

`cardano` is licensed under `MIT OR Apache-2.0`. By contributing, you agree
that your contribution is provided under the same license expression.

## Development Setup

Use the pinned Rust toolchain from `rust-toolchain.toml`.

```bash
cargo check --workspace --all-features
cargo test --workspace
```

Before opening a pull request, run:

```bash
scripts/checks.sh
```

## Security-Sensitive Changes

Treat these areas as high risk:

- CBOR/CDDL decoding and resource limits;
- era selection and validation state transitions;
- UTxO, certificate, governance, and script validation;
- cryptographic verification boundaries;
- signer and key-management APIs;
- query/submit endpoint policy and response trust;
- Plutus execution and resource accounting boundaries;
- node-to-client and node-to-node protocol adapters;
- CI, release scripts, and dependency updates.

Do not post exploitable security details in public issues. Follow
[SECURITY.md](../SECURITY.md).

## Dependency Policy

When adding or updating crates:

- use crates.io releases unless there is a documented reason not to;
- avoid git dependencies;
- check latest versions before editing dependency declarations;
- keep `Cargo.lock` updated;
- run `cargo deny check` and `cargo audit`;
- document why the crate belongs in this workspace.
