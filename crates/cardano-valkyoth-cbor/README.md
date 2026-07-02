# cardano-valkyoth-cbor

Support crate for `cardano`: bounded `no_std` Cardano CBOR and CDDL policy.

Most users should depend on the facade crate instead:

```toml
[dependencies]
cardano = "0.2"
```

Crates.io: <https://crates.io/crates/cardano>

This package is published separately so the `cardano` workspace can keep small,
auditable crate boundaries. Treat it as a lower-level building block unless the
`cardano` documentation explicitly says otherwise.

The planned parser surface is covered by the workspace fuzzing policy. See the
project fuzzing guide for target names, committed seed corpus handling, and
crash reproduction:

<https://github.com/valkyoth/cardano/blob/main/docs/fuzzing.md>
