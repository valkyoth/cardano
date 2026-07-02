# cardano-valkyoth-primitives

Support crate for `cardano`: core `no_std` Cardano protocol primitives.

Most users should depend on the facade crate instead:

```toml
[dependencies]
cardano = "0.2"
```

Crates.io: <https://crates.io/crates/cardano>

This package is published separately so the `cardano` workspace can keep small,
auditable crate boundaries. Treat it as a lower-level building block unless the
`cardano` documentation explicitly says otherwise.

Planned scope includes network identifiers, slots, epochs, coins, hashes,
transaction identifiers, policy identifiers, and era-neutral domain newtypes.
