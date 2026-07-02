# Release Notes 0.3.0

Status: implementation stop reached; waiting for pentest.

## Scope

- Add explicit `no_std` primitive domains for Cardano network ids, eras, slots,
  epochs, block numbers, coins, transaction ids, block hashes, datum hashes,
  script hashes, key hashes, policy ids, credentials, and bounded asset names.
- Add constructor and conversion tests for every primitive domain.
- Re-export the primitive crate through the `cardano` facade as
  `cardano::primitives`.
- Add the explicit `scripts/release_0_3_gate.sh` milestone gate.
- Pin official Cardano source revisions for the first protocol-domain
  milestone.
- Harden spec-lock validation so pinned official source revisions must be
  fetchable from their declared upstream repositories.

## Security

- No Cardano protocol parser, signer, local key storage, network transport,
  Plutus execution, wallet behavior, or ledger validation is implemented in
  this release.
- Equality remains ordinary Rust equality for these public identifiers and
  hashes. No secret-bearing primitive or constant-time equality policy is
  admitted in this release.
- Asset names are bounded to 0..32 bytes before storage.
- Asset-name equality, hashing, and ordering use significant bytes only, so
  unused padding cannot affect semantic identity.
- Fixed hash/id domains reject slices with the wrong byte length.
- The v0.3.0 scratch pentest found a medium spec-lock integrity gap and a low
  `AssetName` padding-invariant hazard; both are remediated before tag
  readiness.

## Spec Evidence

The primitive constraints were checked against pinned official Cardano source
families recorded in `spec-lock.toml`:

- `cardano-ledger`: `7903c0074df4ecc2d96b780ca4e5299d2b866553`
- `cardano-node`: `4fa3e6c4143df1f17aac9e114517e18f3b775934`
- `ouroboros-network`: `b47dbc2c29108e593cc47524cffb75008b88fe90`
- `CIPs`: `20c819b25abee6551a3ef51778b975e7463e1269`

The implemented byte lengths and bounds come from the Conway CDDL definitions
for `hash32`, `hash28`, `transaction_id`, `script_hash`, `policy_id`,
`asset_name`, `credential`, `network_id`, `slot_no`, `epoch_no`, `block_no`,
and `coin`.

The release gate validates that each pinned source revision can be fetched from
the official repository named in `spec-lock.toml`.

## Publishing

`cardano-valkyoth-primitives` is published at `0.2.0`. The `cardano` facade is
published at `0.3.0`. Other support crates remain at `0.1.0` and are not
republished for this milestone.
