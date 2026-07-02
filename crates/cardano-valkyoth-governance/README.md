# cardano-valkyoth-governance

Support crate for `cardano`: future Cardano governance boundaries.

Most users should depend on the facade crate instead:

```toml
[dependencies]
cardano = "0.2"
```

Crates.io: <https://crates.io/crates/cardano>

This package is published separately so the `cardano` workspace can keep small,
auditable crate boundaries. Treat it as a lower-level building block unless the
`cardano` documentation explicitly says otherwise.

Planned scope includes Conway governance actions, DRep data, committee data,
votes, proposals, and CIP-1694 evidence.
