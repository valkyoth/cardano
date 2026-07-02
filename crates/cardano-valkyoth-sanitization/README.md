# cardano-valkyoth-sanitization

Support crate for `cardano`: future secret-bearing data sanitization boundary.

Most users should depend on the facade crate instead:

```toml
[dependencies]
cardano = "0.2"
```

Crates.io: <https://crates.io/crates/cardano>

This package is published separately so the `cardano` workspace can keep small,
auditable crate boundaries. Treat it as a lower-level building block unless the
`cardano` documentation explicitly says otherwise.

Planned scope includes redaction wrappers for keys, signing requests, raw
signed transactions, endpoint credentials, and other values that must not leak
through logs, errors, or debug output.
