# cardano 0.5.0

Status: implementation stop reached; pentest required before tagging.

## Added

- Added `DecodeBudget` for explicit untrusted byte-decoding resource limits.
- Added `DecodeBudgetTracker` for cumulative parser work accounting.
- Added fail-closed `DecodeBudgetError` variants for input bytes, nesting
  depth, item count, map-entry count, allocation bytes, decoded values, and
  invalid nesting-state use.
- Re-exported the CBOR budget model through the `cardano` facade crate.
- Added `scripts/release_0_5_gate.sh` for the decode-budget-model milestone.

## Security

- Future untrusted parsers now have a shared explicit budget boundary before
  scalar, array, map, address, ledger, or script decoders are admitted.
- Future recursive decoders must call `DecodeBudgetTracker::enter_nested()`
  and check its result before descending into nested arrays, maps, tags, or
  container values.
- Budget errors preserve non-secret diagnostic counters for audit-log
  forensics.
- No CBOR scalar parser, address parser, ledger validation, networking,
  signing, local key storage, wallet behavior, Plutus execution, or node
  operation is enabled by this release.

## Verification

- `scripts/checks.sh`
- `scripts/release_0_5_gate.sh`
- `cargo deny check`
- `cargo audit`
