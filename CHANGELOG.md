# Changelog

All notable changes to `cardano` will be documented here.

## 0.2.0 - Unreleased

- Added the explicit `scripts/release_0_2_gate.sh` milestone gate.
- Hardened release-readiness error diagnostics for permanent pentest report
  metadata.
- Added negative release-readiness tests for non-PASS status and blank tester
  fields.
- Updated release-crate tracking so `v0.2.0` is a repository tooling release
  with no crate republishing.

## 0.1.0 - 2026-07-02

- Repository foundation for a security-oriented Cardano Rust workspace.
- Added policy, release, spec-source, threat-model, supply-chain, fuzzing, and
  security documentation.
- Added empty first-party crate boundaries for primitives, CBOR, crypto,
  addresses, ledger, scripts, governance, node, RPC, sanitization, signer,
  testkit, and the `cardano` facade.
- Added local release gates, crate publication planning, SBOM generation hooks,
  and pentest-before-tag readiness checks.
- Added spec-lock policy validation and documented the future node/query
  panic-isolation decision before untrusted message decode paths ship.
