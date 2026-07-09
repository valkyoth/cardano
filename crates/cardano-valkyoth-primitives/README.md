<p align="center">
  <b>no_std Cardano primitives and stable errors for cardano.</b><br>
  Explicit domains, bounded CBOR/CDDL policy, full Cardano implementation work, and security-gated release evidence.
</p>

<div align="center">
  <a href="https://crates.io/crates/cardano">cardano crate</a>
  |
  <a href="https://docs.rs/cardano-valkyoth-primitives">Docs.rs</a>
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

# cardano-valkyoth-primitives

Support crate for `cardano`: core `no_std` Cardano protocol primitives.

Most users should depend on the facade crate instead:

```toml
[dependencies]
cardano = "0.4"
```

Crates.io: <https://crates.io/crates/cardano>

This package is published separately so the `cardano` workspace can keep small,
auditable crate boundaries. Treat it as a lower-level building block unless the
`cardano` documentation explicitly says otherwise.

Implemented scope includes network identifiers, eras, slots, epochs, block
numbers, coins, transaction identifiers, block hashes, datum hashes, script
hashes, key hashes, policy identifiers, credentials, and bounded asset names.
It also exposes the shared stable Cardano error taxonomy used by public
protocol-facing APIs.

This crate does not implement CBOR, address parsing, ledger validation, script
execution, node protocols, signing, query transports, or wallet behavior.
