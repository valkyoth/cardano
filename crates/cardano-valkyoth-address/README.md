<p align="center">
  <b>era-aware Cardano address boundary for cardano.</b><br>
  Explicit domains, bounded CBOR/CDDL policy, full Cardano implementation work, and security-gated release evidence.
</p>

<div align="center">
  <a href="https://crates.io/crates/cardano">cardano crate</a>
  |
  <a href="https://docs.rs/cardano-valkyoth-address">Docs.rs</a>
  |
  <a href="https://github.com/valkyoth/cardano/blob/main/docs/RELEASE_PLAN.md">Release Plan</a>
  |
  <a href="https://github.com/valkyoth/cardano/blob/main/docs/threat-model.md">Threat Model</a>
  |
  <a href="https://github.com/valkyoth/cardano/blob/main/SECURITY.md">Security</a>
</div>

<br>

<p align="center">
  <a href="https://github.com/valkyoth/cardano">
    <img src="https://raw.githubusercontent.com/valkyoth/cardano/main/.github/images/cardano.webp" alt="cardano Rust crate overview">
  </a>
</p>

# cardano-valkyoth-address

Support crate for `cardano`: future Cardano address boundaries.

Most users should depend on the facade crate instead:

```toml
[dependencies]
cardano = "0.2"
```

Crates.io: <https://crates.io/crates/cardano>

This package is published separately so the `cardano` workspace can keep small,
auditable crate boundaries. Treat it as a lower-level building block unless the
`cardano` documentation explicitly says otherwise.

Planned scope includes Shelley and Byron address parsing, network-id checks,
credential domains, reward account handling, and Bech32/base encodings.
