# Security Policy

`cardano` is pre-1.0 protocol software. Treat all releases as security
sensitive, but do not treat any pre-1.0 release as complete Cardano consensus
software.

## Routine Checks

Before a release can be considered tag-ready:

- `scripts/checks.sh` passes.
- `cargo deny check` passes.
- `cargo audit` passes.
- `scripts/generate-sbom.sh` succeeds.
- CodeQL default setup has been checked in GitHub security settings.
- Release notes exist for the exact version.
- A permanent pentest report exists for the exact implementation commit.

## Dependency Policy

Dependencies are admitted only after current-version, license, feature, default
feature, `no_std`, maintenance, and security review. Cardano protocol behavior
must be checked against pinned official source revisions at the same time as
any dependency admission.

## Reporting

Do not open public issues for suspected vulnerabilities. Send a private report
to the maintainer with:

- affected version or commit;
- impact and reproduction notes;
- whether secrets, signatures, transactions, ledger state, or node endpoints
  are exposed.
