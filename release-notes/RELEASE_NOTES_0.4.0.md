# cardano 0.4.0

Status: implementation stop reached; pentest required before tagging.

## Added

- Added a stable, no_std Cardano error taxonomy with category and code types.
- Added stable primitive error codes, categories, and static messages.
- Re-exported the stable error model through the `cardano` facade crate.
- Added `scripts/release_0_4_gate.sh` for the stable-error-model milestone.

## Security

- Error formatting uses stable code/message text and does not include
  secret-bearing payloads.
- No networking, signing, local key storage, wallet behavior, Plutus execution,
  or node operation is enabled by this release.

## Verification

- `scripts/checks.sh`
- `scripts/release_0_4_gate.sh`
- `cargo deny check`
- `cargo audit`
