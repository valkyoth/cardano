<p align="center">
  <b>bounded no_std Cardano CBOR/CDDL policy for cardano.</b><br>
  Explicit domains, bounded CBOR/CDDL policy, full Cardano implementation work, and security-gated release evidence.
</p>

<div align="center">
  <a href="https://crates.io/crates/cardano">cardano crate</a>
  |
  <a href="https://docs.rs/cardano-valkyoth-cbor">Docs.rs</a>
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
