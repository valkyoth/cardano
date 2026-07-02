# cardano-valkyoth-script

Support crate for `cardano`: future native-script and Plutus data boundaries.

Most users should depend on the facade crate instead:

```toml
[dependencies]
cardano = "0.2"
```

Crates.io: <https://crates.io/crates/cardano>

This package is published separately so the `cardano` workspace can keep small,
auditable crate boundaries. Treat it as a lower-level building block unless the
`cardano` documentation explicitly says otherwise.

Planned scope includes native scripts, Plutus data, script hashes, execution
unit policy, and optional Plutus execution adapter boundaries.
