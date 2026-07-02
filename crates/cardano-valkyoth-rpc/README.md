# cardano-valkyoth-rpc

Support crate for `cardano`: future query and submit trust-policy boundaries.

Most users should depend on the facade crate instead:

```toml
[dependencies]
cardano = "0.2"
```

Crates.io: <https://crates.io/crates/cardano>

This package is published separately so the `cardano` workspace can keep small,
auditable crate boundaries. Treat it as a lower-level building block unless the
`cardano` documentation explicitly says otherwise.

Planned scope includes optional transport adapters, endpoint policy, redaction,
and explicit untrusted-node response models.
