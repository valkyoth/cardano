# cardano-valkyoth-primitives

Support crate for `cardano`: core `no_std` Cardano protocol primitives.

Most users should depend on the facade crate instead:

```toml
[dependencies]
cardano = "0.3"
```

Crates.io: <https://crates.io/crates/cardano>

This package is published separately so the `cardano` workspace can keep small,
auditable crate boundaries. Treat it as a lower-level building block unless the
`cardano` documentation explicitly says otherwise.

Implemented scope includes network identifiers, eras, slots, epochs, block
numbers, coins, transaction identifiers, block hashes, datum hashes, script
hashes, key hashes, policy identifiers, credentials, and bounded asset names.

This crate does not implement CBOR, address parsing, ledger validation, script
execution, node protocols, signing, query transports, or wallet behavior.
