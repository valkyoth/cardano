# Crate Version Matrix

Status: `v0.1.0` repository foundation scaffold.

## Version Rules

- `code`: the crate received meaningful implementation, API, or documentation
  changes and uses the release version for the facade crate or an independent
  minor bump for support crates after `v0.1.0`.
- `dependency`: the crate only needs a manifest update because a related crate
  changed outside its current dependency range.
- `metadata`: the crate must be republished with the milestone version to
  correct immutable package metadata.
- `unchanged`: the crate stays on the previous published version and is not
  published.

## v0.1.0 Tracking Table

| Crate | Version | Change | Publish | Reason |
| --- | --- | --- | --- | --- |
| `cardano-valkyoth-cbor` | `0.1.0` | code | yes | Initial bounded CBOR/CDDL crate boundary. |
| `cardano-valkyoth-primitives` | `0.1.0` | code | yes | Initial Cardano domain primitive crate boundary. |
| `cardano-valkyoth-crypto` | `0.1.0` | code | yes | Initial crypto trait-boundary crate. |
| `cardano-valkyoth-address` | `0.1.0` | code | yes | Initial address crate boundary. |
| `cardano-valkyoth-ledger` | `0.1.0` | code | yes | Initial era-aware ledger crate boundary. |
| `cardano-valkyoth-script` | `0.1.0` | code | yes | Initial script crate boundary. |
| `cardano-valkyoth-governance` | `0.1.0` | code | yes | Initial Conway governance crate boundary. |
| `cardano-valkyoth-node` | `0.1.0` | code | yes | Initial node protocol boundary crate. |
| `cardano-valkyoth-rpc` | `0.1.0` | code | yes | Initial query/submit trust-policy crate. |
| `cardano-valkyoth-sanitization` | `0.1.0` | code | yes | Initial secret-bearing data sanitization crate boundary. |
| `cardano-valkyoth-signer` | `0.1.0` | code | yes | Initial external-signer-first crate boundary. |
| `cardano-valkyoth-testkit` | `0.1.0` | code | yes | Initial fixture and adversarial-test crate boundary. |
| `cardano` | `0.1.0` | code | yes | Initial facade crate. |
