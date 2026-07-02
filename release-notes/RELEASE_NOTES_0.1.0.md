# Release Notes 0.1.0

Status: planned foundation release.

## Scope

- Establish Cardano repository, release, security, and spec-source discipline.
- Add empty first-party crate boundaries for the planned workspace, including
  the first-party sanitization boundary.
- Add local checks, SBOM generation hook, release readiness validation, and
  pentest-before-tag process.

## Security

- No protocol parser, signer, local key storage, network transport, Plutus
  execution, wallet behavior, or ledger validation is implemented in this
  release.
- First-party crates forbid unsafe code.

## Spec Evidence

- `spec-lock.toml` records official source repositories.
- Exact revisions are intentionally unpinned until the first protocol
  implementation milestone.
