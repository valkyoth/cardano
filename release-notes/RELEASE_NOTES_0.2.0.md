# Release Notes 0.2.0

Status: implementation stop reached; waiting for pentest.

## Scope

- Add an explicit `scripts/release_0_2_gate.sh` milestone gate.
- Harden release-readiness diagnostics for permanent pentest report metadata.
- Add negative release-readiness tests for non-PASS status and blank tester
  fields.
- Publish only the `cardano` facade crate at `0.2.0` so crates.io shows the
  repository README for the current release.

## Security

- No Cardano protocol parser, signer, local key storage, network transport,
  Plutus execution, wallet behavior, or ledger validation is implemented in
  this release.
- The release gate continues to require `scripts/checks.sh`, `cargo deny
  check`, `cargo audit`, SBOM generation, release notes, and a permanent
  pentest report before a tag-ready claim.

## Spec Evidence

- `spec-lock.toml` remains in scaffold mode with `spec_required = false`.
- Protocol implementation remains blocked by the spec-lock policy before
  ledger, script, or governance crates gain real behavior.

## Publishing

Only the `cardano` facade crate is republished for `v0.2.0`. Support crates
remain at `0.1.0` and will be republished only when they receive meaningful
crate-level changes.
