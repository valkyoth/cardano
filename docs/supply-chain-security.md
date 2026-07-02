# cardano Supply-Chain Security

`cardano` uses overlapping supply-chain controls because each catches a
different failure mode.

## Required Checks

- `cargo deny check` for license, source, and advisory policy.
- `cargo audit` for RustSec advisories.
- `scripts/generate-sbom.sh` for SBOM evidence.
- `scripts/check_latest_tools.sh` before updating pinned tools or release
  infrastructure when network access is available.

## Dependency Admission

Before adding or expanding a dependency:

1. Check the latest released version.
2. Review license compatibility.
3. Inspect default features and `std` requirements.
4. Check maintenance status and security history.
5. Add tests for the behavior being admitted.
6. Document why the dependency belongs in that crate and feature.

Core crates must not gain network, signer, filesystem, clock, TLS, async
runtime, wallet, node transport, or Plutus execution dependencies.

## Admitted Third-Party Crates

| Crate | Version | License | Default? | Reason |
| --- | --- | --- | --- | --- |
| none | n/a | n/a | n/a | `v0.1.0` is a first-party scaffold only. |
